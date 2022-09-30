pub mod client;
pub mod error;
pub mod event;

#[cfg(test)]
mod tests {
    use crate::client::{Client};
    use crate::event::ClientEvent;
    use atri_executor::Executor;
    use std::net::TcpStream;
    use std::time::Duration;

    #[test]
    fn handler() {
        static RUNTIME: atri_executor::runtime::blocking::Runtime =
            atri_executor::runtime::blocking::Runtime;
        async fn handle(_: ClientEvent) {
            println!("233");
        }

        let client = Client::builder()
            .with_default_handler()
            .with_executor(&RUNTIME)
            .with_handler(handle)
            .with_stream(TcpStream::connect("").unwrap());

        RUNTIME.spawn(async move {
            //client.handle(ClientEvent::Test).await;
            println!("Hey!");
        });

        std::thread::sleep(Duration::from_millis(1));
    }
}
