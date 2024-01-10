use std::{net::UdpSocket, sync::mpsc};
use super::messages::{ClientMessages, ServerMessages};

pub fn start(port: u16, client_rx: Receiver<ClientMessages>) -> std::io::Result<Receiver<ServerMessages>> {

    let (tx, rx) = mpsc::channel::<ServerMessages>();

    // if this is the client, then we need to start it  as well
    let socket = UdpSocket::bind(format!("127.0.0.1:{port}"))?;

    socket.set_nonblocking(true);

    std::thread::spawn(move || {

        let mut recv_buf = [0; 1024];
        let mut send_buf = [0; 1024];


        loop {

            if let Ok((amt, src)) = socket.recv_from(&mut buf) {
                // deserialize the message and 

                

            };

            if let Ok(message) = client_rx.try_recv() {
                  // serialize the message and send to the socket
            } 
            

        }





    //     let buf = &mut buf[..amt];

    //     buf.reverse();

    //     socket.send_to(buf, &src)?;
    });


    Ok(rx)
}
