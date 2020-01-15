use std::pin::Pin;

use futures_util::future::TryFutureExt;
use tonic::codegen::Stream;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

use tonic_learn::greeter_server::{Greeter, GreeterServer};
use tonic_learn::{HelloReply, HelloRequest};

#[derive(Default)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            msg: request.into_inner().msg,
        }))
    }

    async fn c_stream_say_hello(
        &self,
        _request: Request<Streaming<HelloRequest>>,
    ) -> Result<Response<HelloReply>, Status> {
        unimplemented!();
    }

    type SStreamSayHelloStream =
        Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send + Sync + 'static>>;

    async fn s_stream_say_hello(
        &self,
        _request: Request<HelloRequest>,
    ) -> Result<Response<Self::SStreamSayHelloStream>, Status> {
        unimplemented!();
    }

    type BStreamSayHelloStream =
        Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send + Sync + 'static>>;

    async fn b_stream_say_hello(
        &self,
        _request: Request<Streaming<HelloRequest>>,
    ) -> Result<Response<Self::BStreamSayHelloStream>, Status> {
        unimplemented!();
    }
}

fn main() {
    let mut runtime = tokio::runtime::Builder::new()
        .max_threads(4)
        .core_threads(4)
        .thread_name("server-runtime")
        .threaded_scheduler()
        .enable_io()
        .build()
        .unwrap();

    let serve = Server::builder()
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve("127.0.0.1:44444".parse().unwrap())
        .map_err(|e| eprintln!("server side exists: {:?}", e));

    let _ = runtime.block_on(serve);
}
