use atri_core::executor::runtime::smol::Runtime;
use atri_core::executor::Executor;
use atri_core::net::connector::send_all;
use std::io::Read;
use std::net::TcpListener;

#[test]
fn smol() {
    let listener = TcpListener::bind("127.0.0.1:8889").unwrap();

    Runtime.spawn(async {
        let mut c = smol::net::TcpStream::connect("127.0.0.1:8889")
            .await
            .unwrap();
        send_all(&mut c, "123".as_bytes()).await.unwrap();
    });

    let mut s = String::new();
    listener.accept().unwrap().0.read_to_string(&mut s).unwrap();
    assert_eq!(s, "123");
}

#[test]
fn tokio() {
    let listener = TcpListener::bind("127.0.0.1:8881").unwrap();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    Executor::spawn(&rt, async {
        let mut c = tokio::net::TcpStream::connect("127.0.0.1:8881")
            .await
            .unwrap();
        send_all(&mut c, "123".as_bytes()).await.unwrap();
    });

    let mut s = String::new();
    listener.accept().unwrap().0.read_to_string(&mut s).unwrap();
    assert_eq!(s, "123");
}

#[test]
fn blocking() {
    let listener = TcpListener::bind("127.0.0.1:8800").unwrap();

    Runtime.spawn(async {
        let mut c = std::net::TcpStream::connect("127.0.0.1:8800").unwrap();
        send_all(&mut c, "123".as_bytes()).await.unwrap();
    });

    let mut s = String::new();
    listener.accept().unwrap().0.read_to_string(&mut s).unwrap();
    assert_eq!(s, "123");
}
