use std::{net::UdpSocket, sync::mpsc};
use super::messages::{ClientMessages, ServerMessages};

pub fn start(port: u16, client_tx: Receiver<ClientMessages>) -> std::io::Result<Receiver<ServerMessages>> {

    let (tx, rx) = mpsc::channel::<ServerMessages>();

    let socket = Arc::new(Mutex::new(UdpSocket::bind(format!("127.0.0.1:{port}"))?));

    let read_socket = socket.clone();

    std::thread::spawn(move || {

        let mut buf = [0; 1024];

        let (amt, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..amt];

        buf.reverse();

        socket.send_to(buf, &src)?;
    });


    Ok(rx)
}
