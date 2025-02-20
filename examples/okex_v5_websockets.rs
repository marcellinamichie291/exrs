use env_logger::Builder;
use exrs::okex_v5::websockets::*;
use exrs::okex_v5::ws_model::WebsocketEvent;
use std::sync::atomic::AtomicBool;

#[actix_rt::main]
async fn main() {
    Builder::new().parse_default_env().init();
    ticker_websocket().await;
}

async fn ticker_websocket() {
    let keep_running = AtomicBool::new(true);
    let ticker_req = r#"{"op": "subscribe","args": [{"channel": "bbo-tbt","instId": "SOL-ETH"}]}"#;
    let (tx, mut rx) = local_channel::mpsc::channel();
    let mut web_socket: WebSockets<WebsocketEvent> = WebSockets::new(tx);

    actix_rt::spawn(async move {
        loop {
            let msg = rx.recv().await.unwrap();
            println!("msg: {:?}", msg);

            actix_rt::task::yield_now().await;
        }
    });

    web_socket.connect("public").await.unwrap();
    web_socket.subscribe_request(ticker_req).await.unwrap();
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {}", e);
    }
    // web_socket.disconnect().await.unwrap();
    // println!("disconnected");
}
