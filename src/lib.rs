#[macro_use]
extern crate rbatis;



mod server;
mod model;
mod db;
mod kafka;
mod utils;

pub struct App {
    config: std::collections::HashMap<String, String>,
    topics: Vec<String>,
    num_workers: usize,
}


impl App {
    pub async fn new() -> App {
        db::new_rb().await;
        let (config, topics) = kafka::kafka_config_init();
        let num_workers = utils::get_env("NUM_WORKERS", "1").parse::<usize>().unwrap();
        App {
            config,
            topics,
            num_workers,
        }
    }
    pub async fn run(&self) -> std::io::Result<()>{
        // (0..self.num_workers)
        // .map(|_| {
        //     tokio::spawn( server::run_async_processor(&self.config))
        // })
        // .collect::<FuturesUnordered<_>>()
        // .for_each(|_| async { () })
        // .await
        let mut workers: Vec<tokio::task::JoinHandle<_>> = Vec::with_capacity(self.num_workers);
        for _ in 0..self.num_workers {
            let consumer = kafka::new_stream_consumer(&self.config, &self.topics);
            workers.push(tokio::spawn( server::run_async_processor(consumer)));
        }
        for worker in workers {
            worker.await?;
        }
        Ok(())
    }
}