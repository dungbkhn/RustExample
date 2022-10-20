use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = [0; 50240];
    //let mut c:u32 = 0;
    loop {
    	//c = c+1;
    	//if c==100 {break;}
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
    
    Ok(())
}
