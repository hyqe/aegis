use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Router};

use std::collections::HashMap;
use std::net::SocketAddr;

const PORT: &str = "PORT";

pub struct Config {
    pub port: u16,
}

impl Config {
    fn missing_env(s: &str) -> String {
        format!("Missing <{}> from environment variables...!", s)
    }

    fn get_env(env_vars: &HashMap<String, String>, var_name: &str) -> Result<u16, String> {
        env_vars
            .get(var_name)
            .ok_or(Self::missing_env(PORT))?
            .parse::<u16>()
            .or_else(|err| Err(err.to_string()))
    }

    pub fn get_config() -> Result<Config, String> {
        // Get Environment Variables
        let env_vars = &std::env::vars().collect::<HashMap<String, String>>();

        let port = Self::get_env(env_vars, PORT)?;

        Ok(Config { port })
    }
}

pub async fn run(cfg: Config) {
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));

    // TODO: add gracefull shutdown
    axum::Server::bind(&addr)
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new().route("/", get(health))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "This is a healthy Server!".to_string())
}
