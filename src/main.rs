mod db;
mod server;

#[tokio::main]
async fn main() {
    let cfg = server::Config::get_config().unwrap();

    println!("Server running on port: {}", cfg.port);
    server::run(cfg).await;
}
