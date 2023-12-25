use std::{env, net::UdpSocket, thread};

fn main() {
    let mut args = env::args();
    args.next();
    let args: Vec<String> = args.collect();
    let len = args.len();
    for i in 0..len {
        match args[i].as_str() {
            "client" => {
                let msg = &args[i+1];
                client(msg).expect("expect my greate ideas to work");
            },
            "server" => {
                server().expect("expect the examples to work");
            },
            _ => {
                ()
            }
        }
    }
}
fn server() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:12345")?;
    let mut buf = [0; 4096];
    loop {
        let sock = socket.try_clone()?;
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", &src);
                    let buf = &mut buf[..amt];
                    dbg!(String::from_utf8_lossy(buf));
                    buf.reverse();
                    sock.send_to(&buf, &src).expect("error sending");
                });
            },
            Err(err) => {
                eprintln!("Err: {}", err);
            }
        }
    }
    
}
fn client(msg: &str) -> std::io::Result<()>  {
    println!("called client");
    let socket = UdpSocket::bind("127.0.0.1:1337")?;
    socket.connect("127.0.0.1:12345")?;
    socket.send(msg.as_bytes())?;

    let mut buf = [0; 4096];
    match socket.recv_from(&mut buf) {
        Ok((amt, _src)) => {
            let buf = &mut buf[..amt];
            dbg!(String::from_utf8_lossy(buf));
        },
        Err(err) => {
            eprintln!("Err: {}", err);
        }
    }
   
    Ok(())
}

