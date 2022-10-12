#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use atri_core::atri_executor::runtime::blocking::Runtime;
    use atri_core::client::Client;

    #[test]
    fn smol() {
        let client = Client::builder()
            .with_default_handler()
            .with_executor(Runtime)
            .with_handler(|_| async {

            })
            .with_connector(TcpStream::connect("127.0.0.1:2255").unwrap())
            .run();
    }
}
