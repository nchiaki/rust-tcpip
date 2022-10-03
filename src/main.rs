use std::env;

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

    //let stream = tcpip::connect(dsthost, dstport).unwrap();
    //println!("{:?}", stream);
    //tcpip::function(stream);

    let mut streams = Vec::new();
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
                    break;
                },
            };
            streams.push(strm);
            cnctcnt += 1;
        }
        println!("{}/{} connects", cnctcnt, streams.len());
        cnctcnt = 0;
    }

}
