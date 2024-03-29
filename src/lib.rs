#[macro_use]
extern crate rbatis;

mod server;
mod model;
mod db;
mod kafka;
mod utils;
mod config;

pub struct App {
    config: std::collections::HashMap<String, String>,
    topics: Vec<String>,
    num_workers: usize,
}


impl App {
    pub async fn new() -> App {
        db::new_rb().await;
        let (config, topics) = kafka::kafka_config_init();
        let num_workers = utils::get_env("KAFKA_NUM_WORKERS", config::DAFAULT_KAFKA_NUM_WORKERS)
            .parse::<usize>().unwrap();
        // log output
        fast_log::init_log("log/requests.log", 1000, log::Level::Info, None, true).unwrap();
        App {
            config,
            topics,
            num_workers,
        }
    }
    pub async fn run(&self) -> std::io::Result<()>{
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