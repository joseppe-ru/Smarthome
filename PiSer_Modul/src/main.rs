use std::io::{stdout,BufWriter};
use warp::{Filter, ws};
use ferris_says::say;
use futures::StreamExt;
use local_ip;
use futures::stream::SplitSink;
use warp::ws::{Message, WebSocket};
use std::io;
use tokio::task::JoinHandle;
use tokio::sync::oneshot;


async fn system_input(tx_shut: oneshot::Sender<()>) -> Result<(), &'static str>{
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
                tx_shut.send(()).expect("Onshot tx_shut fehler");
                return Err("programm wird beendet")
            }
            "1"=>{
                println!("Server is going to be restarted;");
                tx_shut.send(()).expect("Onshot tx_shut fehler");
                return Ok(())
            }
            _=>{
                println!("Bitte gib eine gültige Zahl ein!");
                continue;
            }
        }
    }
}

async fn handle_websocket_message(message:Message, tx: &mut SplitSink<WebSocket,Message>){
    println!("Nachricht empfangen: {:?}",message);
}

async fn handle_client(web_socket: WebSocket){
    let (mut tx, mut rx) = web_socket.split();

    while let Some(body) = rx.next().await{
        let message = match body{
            Ok(msg)=>msg,
            Err(e)=>{
                println!("error reading websocket received message: {e}");
                break;
            }
        };
        handle_websocket_message(message, &mut tx).await;
    }

    println!("WebSocket verbindung unterbrochen");
}

async fn http_server_setup(rx_shut: oneshot::Receiver<()>) ->Result<(), &'static str> {
    let ws_route = warp::path("websocket")
        .and(warp::ws())
        .map(|ws: ws::Ws|{
            println!("Connection was upgraded to websocket.");
            ws.on_upgrade(handle_client)
        });

    let curr_dir = std::env::current_dir().expect("failed to read current directory");
    let routes=warp::get().and(ws_route.or(warp::fs::dir(curr_dir.join("FrWeb-UI"))));

    //Certificate: openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.rsa -out cert.pem
    let (addr,server)=warp::serve(routes)
        .tls()
        .cert_path("cert.pem")
        .key_path("key.rsa")
        .bind_with_graceful_shutdown(([0,0,0,0],9231),async { rx_shut.await.ok(); });

    println!("Adresse?: {addr}");
    // Spawn the server into a runtime
    tokio::task::spawn(server);

    return Ok(())
}

async fn flatten<T>(handle: JoinHandle<Result<T, &'static str>>) -> Result<T, &'static str> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(_err) => Err("handling failed"),
    }
}

#[tokio::main]
async fn main() {
    //Greetings
    {
        let stdout = stdout();
        let writer = BufWriter::new(stdout.lock());

        say("Hello, Munke",12,writer).unwrap();
        println!("HTTP Server hosted at https://{:?}:9231/",local_ip::get().unwrap()); //eigne ip ausgeben
    }

    //TODO: tx und rx oneshot muss von Server-Funktion erstellt werden, damit der Server neugestartet werden kann
    // -> tx muss dann auch neu an Input-Funktion übergeben werden
    //Signal für shutdown
    let (tx_shut, rx_shut) = oneshot::channel::<()>();

    let http_server=tokio::spawn(http_server_setup(rx_shut));
    let input=tokio::spawn(system_input(tx_shut));

    let processing_res = tokio::try_join!(
        flatten(http_server),
        flatten(input));

    match processing_res{
        Ok((server_res,input_res))=>{
            println!("Rückgabe = (Server: {:?}) (Input: {:?} ",server_res,input_res);
        }
        Err(e)=>{
            println!("Fehler in Tokio! err: {e}");
        }
    }
}