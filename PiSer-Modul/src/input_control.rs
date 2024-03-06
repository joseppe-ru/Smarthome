use std::io;
use tokio::sync::oneshot;

pub async fn system_input(shut_channel_tx: oneshot::Sender<()>) -> Result<(), &'static str>{
    loop{
        //Display Options
        println!("[input] 1 Restart HTTPServer");
        println!("[input] 0 Shut Down Server");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("[input] fehler beim lesen der Eingabe");

        match input.trim(){
            "0"=>{
                println!("[input] Input: 0; beenden");
                shut_channel_tx.send(()).expect("[input] Onshot tx_shut fehler");
                return Err("[input] programm wird beendet")
            }
            "1"=>{
                println!("[input] Server is going to be restarted;");
                shut_channel_tx.send(()).expect("[input] Onshot tx_shut fehler");
                return Ok(())
            }
            _=>{
                println!("[input] Bitte gib eine gültige Zahl ein!");
                continue;
            }
        }
    }
}