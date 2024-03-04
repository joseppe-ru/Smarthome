use std::io;
use tokio::sync::oneshot;

pub async fn system_input(shut_channel_tx: oneshot::Sender<()>) -> Result<(), &'static str>{
    loop{
        //Display Options
        println!("1 Restart HTTPServer");
        println!("0 Shut Down Server");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("fehler beim lesen der Eingabe");

        match input.trim(){
            "0"=>{
                println!("Input: 0; beenden");
                shut_channel_tx.send(()).expect("Onshot tx_shut fehler");
                return Err("programm wird beendet")
            }
            "1"=>{
                println!("Server is going to be restarted;");
                shut_channel_tx.send(()).expect("Onshot tx_shut fehler");
                return Ok(())
            }
            _=>{
                println!("Bitte gib eine gültige Zahl ein!");
                continue;
            }
        }
    }
}