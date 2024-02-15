//https://medium.com/@yuriy.voshch/building-an-mqtt-broker-from-scratch-with-rust-and-lunatic-part-1-bff5a2a58f61

use std::io::{BufWriter, Read, stdout, Write};
use std::time::Duration;
use lunatic::net::TcpListener;
use ferris_says::say;
use lunatic::spawn_link;

//Up-, und Downframe für MQTT (fix header, variable payload)
/*
let mqtt_connect_msg = vec![
    0x10,       // Packet Type: CONNECT
    0x1F,       // Remaining Length: 31
    0x00, 0x04, // Protocol Name Length: 4
    0x4D, 0x51, 0x54, 0x54, // Protocol Name: MQTT
    0x05,             // Protocol Level: MQTT 3.1.1
    0xC2, 0x00,       // Connect Flags: Clean Session, Will QoS 2, Will Retain, No User Name or Password, Keep Alive: 30 seconds
    0x00, 0x1E,       // Keep Alive: 30 seconds
    0x00, 0x06,       // Client ID Length: 6
    0x64, 0x65, 0x76, 0x69, 0x63, 0x65, // Client ID: device1
    0x00, 0x08,       // Will Topic Length: 8
    0x68, 0x6F, 0x6D, 0x65, 0x2F, 0x74, 0x65, 0x73, // Will Topic: home/test
    0x00, 0x0A,       // Will Message Length: 10
    0x64, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65, 0x66, 0x67, 0x68, // Will Message: deadbeefgh
];


const MQTT_CONN_ACK_MSG:Vec<u8>=vec![
    0x20,
    0x02,
    0x00,
    0x00
];
*/
fn main()-> std::io::Result<()> {
    {//Greetings
        let stdout = stdout();
        let writer = BufWriter::new(stdout.lock());
        say("MQTT-Broker", 12, writer).unwrap();
    }

    let listener = TcpListener::bind("127.0.0.1:1883")?;
    while let Ok((stream,_))=listener.accept(){
        println!("New Client Connected:{:?}",stream.peer_addr());
        spawn_link!(|stream| connection_handler(stream));
    }
    Ok(())
}

fn connection_handler(mut stream: lunatic::net::TcpStream){
    let mut buffer =[0;1024];
    loop{
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let message_type = buffer[0] >> 4; //Typ der Nachricht????
                let message_length = buffer[1];//nachrichtenlänge???
                println!("Received message of type {message_type}, with length {message_length}");
                match message_type {
                    1 => { // Typ
                        let ack_msg: Vec<u8> = vec![
                            0x20,
                            0x02,
                            0x00,
                            0x00,
                        ];
                        stream.write(&ack_msg).expect("Failed to send Acknowledged");
                        println!("Responded to CONNECT");
                    }
                    t => eprintln!("Unknown type of MQTT!"),
                }
            }
            Ok(_) => { eprintln!("Nothing to read . . .");
                lunatic::sleep(Duration::from_secs(1)); //Bullshiat Statt sleep bracuhe ich irgendwas asynchrones????
            },
            Err(e) => eprintln!("unbekannter Fehler: {e}"),
        }
    }
}