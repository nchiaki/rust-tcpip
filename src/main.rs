use std::env;
use std::net::TcpStream;
use std::io::{stdout, Write};
use std::rc::Rc;
use std::borrow::Borrow;

mod tcpip;

fn main() {
    let mut dsthost = "chiaki.sakura.ne.jp";
    let mut dstport = 80;

    println!("Hello, world!");

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    println!("[{}]:{:?}", argc, argv);
    if 2 <= argc
    {dsthost = &argv[1];}
    if 3 <= argc
    {
        dstport =  argv[2].parse().unwrap();
    }

    let mut streams: Vec<TcpStream> = Vec::new();
    let mut cnctcnt = 0;
    loop
    {
        loop
        {
            let strm = match tcpip::connect(dsthost, dstport)
            {
                Ok(v) => v,
                Err(e) =>
                {
                    println!("{}", e);

                    match e.find("nodename nor servname provided")
                    {
                        Some(_) =>
                        {
                            loop
                            {
                                let trgt = match streams.pop()
                                {
                                    Some(v) => v,
                                    None => break,
                                };
                                let trgt_info = format!("{:?} {}", trgt, dsthost);
                                match tcpip::shutdown(trgt)
                                {
                                    Ok(_) => (),
                                    Err(_) => (),
                                };
                                print!("{} conections {:?}\r", streams.len(), trgt_info);stdout().flush();
                            }
                        },
                        None => (),
                    };
                    break;
                },
            };
            let strm_info = format!("{:?} {}", strm, dsthost);
            streams.push(strm);
            print!("{} conections {}\r", streams.len(), strm_info);stdout().flush();
            cnctcnt += 1;
        }
        let dst_info = format!("{}:{}", dsthost, dstport);
        println!("\n{}/{} connects {}", cnctcnt, streams.len(), dst_info);
        if cnctcnt == 0 && streams.len() == 0
        {break;}
        cnctcnt = 0;
    }

}
