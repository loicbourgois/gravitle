use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use tungstenite::Message;
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum FirstMessageRequest {
    CreateSender,
    CreateReceiver,
}
#[derive(Serialize, Deserialize, Debug)]
struct FirstMessage {
    request: FirstMessageRequest,
    uuid: Uuid,
}

fn retry_receive() {
    println!("Could not connect. Retry in 1 sec.");
    thread::sleep(Duration::from_millis(1000));
    receive();
}

fn receive() {
    match tungstenite::client::connect("ws://127.0.0.1:8000/ws") {
        Ok(connection) => {
            let (mut client_receiver, response) = connection;
            println!("response: {:?}", response);
            client_receiver
                .write_message(Message::Text(
                    serde_json::to_string(&FirstMessage {
                        request: FirstMessageRequest::CreateSender,
                        uuid: Uuid::new_v4(),
                    })
                    .unwrap(),
                ))
                .unwrap();
            let mut c = 0;
            loop {
                match client_receiver.read_message() {
                    Ok(message) => {
                        c += 1;
                        let message_data = message.into_data();
                        let data_client: core::data_client::DataClient =
                            bincode::deserialize(&message_data[..]).unwrap();
                        if c % 10 == 0 {
                            println!("# {}", data_client.step);
                            println!("  part_count: {}", data_client.part_count);
                        }
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                        retry_receive()
                    }
                }
            }
        }
        Err(error) => {
            println!("Error: {}", error);
            retry_receive()
        }
    }
}

fn main() {
    receive();
}
