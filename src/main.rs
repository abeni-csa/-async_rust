use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use hyper::rt::Executor;
use hyper::server::conn::http2;
use std::future::Future;

async fn handle(body: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("Recived a request");
    println!("Headers: {:#?}", body.headers());
    println!("Body: {:#?}", body.into_body());
    Ok(Response::new(Full::new(Bytes::from("Hello World"))))
}

async fn false_main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // process the TCP request
            println!("Accepted connection from: {}", socket.peer_addr().unwrap());
        });
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 300));
    let listiner = TcpListener::bind(&addr).await;
    loop {
        let (stream, _) = listiner.expect("END").accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new(TokioExecutor)
                .serve_connection(io, service_fn(handle))
                .await
            {
                println!("Error Serving COnnection {:#?}", err);
            }
        });
    }
}
#[derive(Clone)]
struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        tokio::spawn(future);
    }
}
