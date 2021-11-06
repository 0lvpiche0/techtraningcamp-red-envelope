use red_packet_transport::App;


#[tokio::main]
async fn main() {
    let app = App::new().await;
    match app.run().await {
        Ok(_) => log::info!("main ends normally"),
        Err(e) => log::error!("{}", e),
    };
}