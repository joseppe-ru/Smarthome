//https://medium.com/@yuriy.voshch/building-an-mqtt-broker-from-scratch-with-rust-and-lunatic-part-1-bff5a2a58f61

use std::io::{BufWriter, stdout};
use lunatic::net::TcpListener;
use ferris_says::say;

fn main()-> std::io::Result<()> {
    let stdout = stdout();
    let writer = BufWriter::new(stdout.lock());
    say("MQTT-Broker",12,writer).unwrap();

    

    Ok(())
}
