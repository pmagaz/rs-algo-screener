use std::env;
use dotenv::dotenv;
use rs_algo_shared::ws::ws_client::{WebSocket};
use actix::prelude::*;
use rs_algo_shared::ws::message::{Message, CommandType, Command, Subscribe};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

  // #[derive(Clone, Message, Serialize, Deserialize, Debug)]
  // #[rtype(result = "()")]
  // pub struct ChatMessage {
  //   msg: String
  // };
    
  dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let server_url = env::var("WS_SERVER_URL").expect("WS_SERVER_URL not found");
    let port = env::var("WS_SERVER_PORT").expect("WS_SERVER_PORT not found");

    log::info!("Connecting to {} on port {} !", server_url, port);
    
    let mut ws_client = WebSocket::connect(&[&server_url, ":", &port].concat()).await;

    let mut delay = tokio::time::interval(std::time::Duration::from_secs(1));
    for _ in 0..5 {
        delay.tick().await;

        log::info!("Sending to {} on port {} !", server_url, port);

        ws_client
            .send(&serde_json::to_string(&Command{
              command: CommandType::Subscribe,
              arguments:Subscribe {
                symbol: "EURUSD",
                time_frame: "W",
              } 
            }).unwrap())
            .await.unwrap();
    }


    loop {
      let msg = ws_client.read().await.unwrap();
      let txt_msg = match msg {
          Message::Text(txt) => txt,
          Message::Ping(txt) => {
            log::info!("Ping received");
            ws_client.pong(b"");
            "hola".to_string()
          },
          _ => panic!(),
      };
         tokio::spawn(async move {
            // Process each socket concurrently.
            log::debug!("{}", &txt_msg);
        });
    }

}
