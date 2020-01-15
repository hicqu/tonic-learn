use futures_util::future::TryFutureExt;
use tonic::Request;

use tonic_learn::greeter_client::GreeterClient;
use tonic_learn::HelloRequest;

fn main() {
    let mut runtime = tokio::runtime::Builder::new()
        .max_threads(4)
        .core_threads(4)
        .thread_name("client-runtime")
        .threaded_scheduler()
        .enable_io()
        .build()
        .unwrap();

    let mut client = runtime
        .block_on(GreeterClient::connect("http://127.0.0.1:44444"))
        .map_err(|e| eprintln!("connect to server fail: {:?}", e))
        .unwrap();

    let request = Request::new(HelloRequest {
        msg: "test message".into(),
    });

    let f = client
        .say_hello(request)
        .map_ok(|resp| println!("resp: {:?}", resp.into_inner().msg))
        .map_err(|e| eprintln!("RPC fail: {:?}", e));

    let _ = runtime.block_on(f);
}
