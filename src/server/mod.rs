use futures::TryStreamExt;
use log::info;
use rbatis::crud::CRUD;
use rdkafka::Message;
use crate::{db, model};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::message::BorrowedMessage;

// test
async fn record_borrowed_message_receipt(msg: &BorrowedMessage<'_>) {
    // Simulate some work that must be done in the same order as messages are
    // received; i.e., before truly parallel processing can begin.
    info!("Message received partition: {:?}", msg.partition());
    let p = msg.payload_view::<str>();
    println!("{:?}", p);
    // todo json to envelop
    // envelope_server(&envelop)
    // todo handle err
}

// async fn record_owned_message_receipt(_msg: &OwnedMessage) {
//     // Like `record_borrowed_message_receipt`, but takes an `OwnedMessage`
//     // instead, as in a real-world use case  an `OwnedMessage` might be more
//     // convenient than a `BorrowedMessage`.
// }

// Emulates an expensive, synchronous computation.
// fn expensive_computation<'a>(msg: OwnedMessage) -> String {
//     info!("Starting expensive computation on message {}", msg.offset());
//     // thread::sleep(Duration::from_millis(rand::random::<u64>() % 5000));
//     info!(
//         "Expensive computation completed on message {}",
//         msg.offset()
//     );
//     match msg.payload_view::<str>() {
//         Some(Ok(payload)) => format!("Payload len for {} is {}", payload, payload.len()),
//         Some(Err(_)) => "Message payload is not a string".to_owned(),
//         None => "No payload".to_owned(),
//     }
// }



// Creates all the resources and runs the event loop. The event loop will:
//   1) receive a stream of messages from the `StreamConsumer`.
//   2) filter out eventual Kafka errors.
pub async fn run_async_processor(consumer: StreamConsumer) {
    // Create the outer pipeline on the message stream.
    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        async move {
            // Here are the different ways to deal with it, but I think that the task is expensive_computation in this case (lvpiche)
            // Process each message, this func is used to test and it print msg
            record_borrowed_message_receipt(&borrowed_message).await;
            // Borrowed messages can't outlive the consumer they are received from, so they need to
            // be owned in order to be sent to a separate thread.
            
            // let owned_message = borrowed_message.detach();
            // tokio::spawn(async move {
            //     owned_message.payload_view::<str>().and_then(|msg| async {

                    
            //     });
            // });
            Ok(())
        }
    });

    info!("Starting event loop");
    stream_processor.await.expect("stream processing failed");
    info!("Stream processing terminated");
}

// The envelope must be a complete instance
// msg maybe stored in different partition
// so it it possible that msg is unordered
async fn envelope_server(envelope: &model::Envelope) -> std::io::Result<()> {
    // Check whether there is this record in the database
    match model::select_by_rid(&envelope.rid).await {
        // if there is the record in the database -> update status
        Ok(r) => {
            if envelope.status == 1 && r.status == 0 {
                model::update_status_by_rid(&envelope.rid).await?;
            }
            Ok(())
        },
        // else -> insert
        Err(_) => {
            db::RB.save(&envelope, &[]).await?;
            Ok(())
        }
    }

}