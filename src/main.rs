use std::{net::SocketAddr, sync::atomic::AtomicBool};

use anyhow::Error;
use axum::{routing::get, Router};
use tokio::time;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(|| async {
        let cache = ConcreteAsyncImpl::new();
        println!("Before");
        tokio::spawn(async move {
            tokio::time::sleep(time::Duration::from_millis(5000)).await;
            println!("From a spawned thread");
        });
        let _temp = cache.get("123".to_string()).await;
        println!("Before sleep after spawned thread");
        tokio::time::sleep(time::Duration::from_millis(1000)).await;
        println!("After");
        "Hello, World!"
    }));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[async_trait::async_trait]
pub trait AsyncTrait {
    async fn get(&self, module_uuid: String) -> Result<String, Error>;
}

pub struct ConcreteAsyncImpl {
    counter: AtomicBool,
}

impl ConcreteAsyncImpl {
    fn new() -> Self {
        ConcreteAsyncImpl {
            counter: AtomicBool::new(false)
        }
    }

    fn swap(&self) {
        if self.counter.load(std::sync::atomic::Ordering::Relaxed) {
            self.counter.swap(false, std::sync::atomic::Ordering::Relaxed);
        } else {
            self.counter.swap(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[async_trait::async_trait]
impl AsyncTrait for ConcreteAsyncImpl {
    async fn get(&self, module_uuid: String) -> Result<String, Error> {
        println!("Into the async trait");
        tokio::time::sleep(time::Duration::from_millis(5000)).await;
        println!("Got this inside of the async trait");
        self.swap();
        if self.counter.load(std::sync::atomic::Ordering::Relaxed) {
            Ok(module_uuid)
        } else {
            Err(anyhow::anyhow!("Error"))
        }
    }
}
