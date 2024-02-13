use std::io::{stdout, BufWriter};
use warp::{Filter, ws};
use ferris_says::say;
use futures::{SinkExt, StreamExt};
use futures::stream::SplitSink;
use warp::ws::{Message, WebSocket};
use std::io;
use tokio::task::JoinHandle;
use tokio::sync::oneshot;
use local_ip_address::list_afinet_netifas;

//Experimentlell: Benutzerregistrierung:
/*

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

*/


async fn system_input(tx_shut: oneshot::Sender<()>) -> Result<(), &'static str>{
    //TODO: Nachrichten eingeben und senden

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
            "2"=>{
                println!("Was soll gesendet werden?:");
                let mut input=String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("fehler beim lesen der Eingabe");
                //Senden der Nachricht über den Websocket
            }
            _=>{
                println!("Bitte gib eine gültige Zahl ein!");
                continue;
            }
        }
    }
}

/* # Hier ist der Endpunkt des Websockets "wss://[ip:port]/websocket"
 */
async fn handle_websocket_message(message:Message, tx: &mut SplitSink<WebSocket,Message>){
    println!("Nachricht empfangen: {:?}",message);
    //TODO: Auswerten von Empfangenen Daten (auf allgemeingültige Vorschrift einigen)
    // eine Art "Bibliothek"/"Dictionary" festlegen
    // (evtl sogar in WSAM (RUST) -> damit ich das nur einmal festlegen muss?)
    // tx für eine Reaktion...
}

async fn handle_client(web_socket: WebSocket){

    let (mut tx, mut rx) = web_socket.split();

    //Senden einer Initialisierungsnachricht (zum Aufbauen der Website, welche Geräte vorhanden sind...)
    let tx_message=Message::text("Hello Munke. Here is Rust!");
    tx.send(tx_message).await.expect("failed to send init message");

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
        let network_interfaces = list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
            if name == "wlp0s20f3"{
                if ip.is_ipv4(){
                    println!("HTTP Server hosted at https://{:?}:9231/", ip);
                }
            }
        }
    }

    //multi-producer_single-consumer Queue anlegen Type: String; Größe: 1;
    let (ws_sender_channel,mut ws_receiver_channel)=tokio::sync::mpsc::channel::<String>(1);

    loop {
        //TODO: tx und rx oneshot muss von Server-Funktion erstellt werden, damit der Server neugestartet werden kann
        // -> tx muss dann auch neu an Input-Funktion übergeben werden?
        //Signal für shutdown
        let (shut_channel_sender, shut_channel_receiver) = tokio::sync::oneshot::channel::<()>();

        let http_server = tokio::spawn(http_server_setup(shut_channel_receiver));
        let input = tokio::spawn(system_input(shut_channel_sender));

        let processing_res = tokio::try_join!(
            flatten(http_server),
            flatten(input)
        );

        //TODO: Schließroutine anlegen:
        // -> websocket "schieß"-Nachricht absenden
        // -> http-server stoppen?
        // -> Programm beenden


        match processing_res {
            Ok((server_res, input_res)) => {
                println!("Rückgabe = (Server: {:?}) (Input: {:?} ", server_res, input_res);
                continue;
            }
            Err(e) => {
                println!("Fehler in Tokio! err: {e}");
                return;
            }
        }
    }
}