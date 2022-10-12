pub extern crate atri_executor;

pub mod client;
pub mod error;
pub mod event;

mod crypto;
mod network;

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::event::ClientEvent;
    use atri_executor::Executor;
    use std::net::{Ipv4Addr, TcpListener, TcpStream};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn handler() {
        static RUNTIME: atri_executor::runtime::blocking::Runtime =
            atri_executor::runtime::blocking::Runtime;
        async fn handle(_: ClientEvent) {
            println!("233");
        }

        thread::spawn(|| {
            let ip = Ipv4Addr::new(127, 0, 0, 1);
            let server = TcpListener::bind((ip, 2255)).unwrap();
            for incoming in server.incoming() {
                let d = incoming.unwrap();
            }
        });

        let client = Client::builder()
            .with_default_handler()
            .with_executor(&RUNTIME)
            .with_handler(handle)
            .with_connector(TcpStream::connect("127.0.0.1:2255").unwrap())
            .run();

        RUNTIME.spawn(async move {
            //client.handle(ClientEvent::Test).await;
            println!("Hey!");
        });

        thread::sleep(Duration::from_millis(1));
    }
}
