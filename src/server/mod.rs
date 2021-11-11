
use std::str::Utf8Error;
use std::sync::Arc;

use futures::TryStreamExt;
use log::info;
use rbatis::crud::CRUD;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use crate::{db, model};
use rdkafka::consumer::stream_consumer::StreamConsumer;


// Creates all the resources and runs the event loop. The event loop will:
//   1) receive a stream of messages from the `StreamConsumer`.
//   2) filter out eventual Kafka errors.
pub async fn run_async_processor(consumer: StreamConsumer) {
    // Create the outer pipeline on the message stream.
    let consumer_ref = Arc::new(consumer);
    let main = consumer_ref.clone();

    let stream_processor = main.stream().try_for_each(|borrowed_message| {
        let commiter = consumer_ref.clone();
        async move {
            let envelope: Option<model::Envelope> = borrowed_message.payload_view::<str>()
                .and_then(|msg| msg_to_envelope(msg));
            if let Some(envelope) = envelope {
                if let Err(e) = envelope_server(envelope).await {
                    log::error!("envelope_server {}", e);
                } else {
                    if let Err(e) = commiter.commit_message(&borrowed_message, rdkafka::consumer::CommitMode::Async) {
                        log::error!("kafka commite {}", e);
                    }
                }
            }
            Ok(())
        }
    });
    info!("Stream loop begin");
    stream_processor.await.expect("stream processing failed");
    info!("Stream processing terminated");
}


fn msg_to_envelope(msg: Result<&str, Utf8Error>) -> Option<model::Envelope> {
    match msg {
        Ok(msg) => {
            match serde_json::from_str(msg) {
                Ok(envlope) => Some(envlope),
                Err(e) => {
                    log::error!("{}",e);
                    None
                }
            } 
        },
        Err(e) => {
            log::error!("{}", e);
            None
        },
    }
}


// The envelope must be a complete instance
// msg maybe stored in different partition
// so it it possible that msg is unordered
async fn envelope_server(envelope: model::Envelope) -> std::io::Result<()> {
    // Check whether there is this record in the database
    let res: Option<model::Envelope> = 
        db::RB.fetch_by_column("envelope_id", &envelope.envelope_id).await?;
    match res {
        // if there is the record in the database -> update status
        Some(r) => {
            if envelope.opened  && !r.opened {
                model::update_status_by_rid(&envelope.envelope_id).await?;
            }
            Ok(())
        },
        // else -> insert
        None => {
            db::RB.save(&envelope, &[]).await?;
            Ok(())
        }
    }
}