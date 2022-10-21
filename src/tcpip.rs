use std::net::ToSocketAddrs;
use std::net::{Shutdown, TcpStream};
use std::io::{BufReader, BufRead};
use std::io::BufWriter;
use std::io::Write;

pub fn connect(dsthost:&str, dstport:u16) -> Result<TcpStream, String>
{
    let dst_host_port = format!("{}:{}", dsthost, dstport);
    let mut dstaddr = match dst_host_port.to_socket_addrs()
    {
        Ok(v) => v,
        Err(e) =>
        {
            eprintln!("SockAddr: {}", e);
            return Err(e.to_string());
        },
    };

    if let Some(addr) = dstaddr.find(|x| (*x).is_ipv4())
    {
        match TcpStream::connect(addr)
        {
            Ok(stream) =>
            {
                //println!("Connecct success. : {}", dst_host_port);
                Ok(stream)
            },
            Err(er) =>
            {
                let xer = format!("Connecct faile. {} : {}", er, dst_host_port);
                //eprintln!("{}",xer);
                Err(xer)
            }
        }
    }
    else
    {
        let xer = format!("Illegal destination : {}", dst_host_port);
        Err(xer)
    }
}

pub fn shutdown(stream:TcpStream) -> Result<(), String>
{
    match stream.shutdown(Shutdown::Both)
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            //println!("{:?} shutdown: {}", stream, e);
            return Err(e.to_string());
        },
    };
}

pub fn function(stream:TcpStream)
{
    let mut sndbuf = format!("GET / HTTP/1.1\r\nHost: chiaki.sakura.ne.jp\r\nUser-Agent: webget/0.1\r\nAccept: */*\r\n\r\n");
    let mut rcvbuf = String::new();

    let mut sender = BufWriter::new(&stream);
    let mut receiver = BufReader::new(&stream);

    let sact = sender.write(sndbuf.as_bytes()).expect("Send error!!!");
    println!("Snd:[{}]{}", sact, sndbuf);
    sender.flush();

    let mut rttl = 0;
    let mut ract = receiver.read_line(&mut rcvbuf).expect("Receive error!!!");
    println!("Rcv:[{}]",ract);
    while ract != 0
    {
        rttl += ract;
        ract = receiver.read_line(&mut rcvbuf).expect("Receive error!!!");
        println!("Rcv:[{}]",ract);
    }
    println!("Rcv:[{}]{}", rttl, rcvbuf);
}
