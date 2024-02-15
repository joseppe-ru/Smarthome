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

//Cleverer Ansatz zum Server-ausgelösten übermitteln von daten:
// ->beim initialisieren der websocket-Verbindung einen neuen Thread in dauerschleife startten
//      -> dieser thread bekommt einen mpsc-Kanal übergeben
//      -> semaphore stoppt den Thread
//      -> senden einer nachricht, wenn semaphore gelöst wird??


//andere idee zum aktualisieren..
//  -> JS müsste regelmäßig einen Pfad(Filter) abfragen
//  diersre Filter muss eine information über den auktuellen systemstatus enthalten

async fn system_input(shut_channel_tx: oneshot::Sender<()>) -> Result<(), &'static str>{
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

/* # Hier ist der Endpunkt des Websockets "wss://[ip:port]/websocket"
 */
async fn handle_websocket_message(message:Message, _tx: &mut SplitSink<WebSocket,Message>){
    println!("Nachricht empfangen: {:?}",message);
    //TODO: Auswerten von Empfangenen Daten (auf allgemeingültige Vorschrift einigen)
    // eine Art "Bibliothek"/"Dictionary" festlegen
    // (evtl sogar in WSAM (RUST) -> damit ich das nur einmal festlegen muss?)
    // tx für eine Reaktion...


    //TODO: eine Message soll bitte den Server neustarten -> weil datnebank neu einlesen???
}

async fn handle_client(web_socket: WebSocket){

        let (mut tx, mut rx) = web_socket.split();

        //Senden einer Initialisierungsnachricht (zum Aufbauen der Website, welche Geräte vorhanden sind)
        let db_json_file = std::fs::read_to_string("nosql_ids/db.json").unwrap(); //db-datei einlesen

        tx.send(Message::text(db_json_file)).await.expect("failed to send init message");

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

//TODO: irgendwie muss ich den Client noch registrieren???
// -> gibt es eine UUID, evtl, eine ipv6, die ich überprüfen kann.
// -> weil damit kann ich sicherstellen, dass sich hier niemand fremdes anmeddelt
async fn http_server_setup(shut_channel_rx: oneshot::Receiver<()>) ->Result<(), &'static str> {
    let ws_route = warp::path("websocket")
        .and(warp::ws())
        .map(|ws: ws::Ws|{
            println!("Connection was upgraded to websocket.");
            ws.on_upgrade(move |websocket| async move {

                handle_client(websocket).await;
            }
            )});

    let curr_dir = std::env::current_dir().expect("failed to read current directory");

    let routes=warp::get()
        .and(ws_route
            .or(warp::fs::dir(curr_dir.join("FrWeb-UI"))));

    //Certificate: openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.rsa -out cert.pem
    let (_,server)=warp::serve(routes)
        .tls()
        .cert_path("cert.pem")
        .key_path("key.rsa")
        .bind_with_graceful_shutdown(([0,0,0,0],9231),async { shut_channel_rx.await.ok(); });

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
        println!("HTTP Server hosted at https://localhost:9231/");
    }

    loop {
        //TODO: tx und rx oneshot muss von Server-Funktion erstellt werden, damit der Server neugestartet werden kann
        // -> tx muss dann auch neu an Input-Funktion übergeben werden?

        //Signal-Kanal für shutdown
        let (shut_channel_sender, shut_channel_receiver) = tokio::sync::oneshot::channel::<()>();

        let http_server = tokio::spawn(http_server_setup(shut_channel_receiver));
        let input = tokio::spawn(system_input(shut_channel_sender));

        let processing_res = tokio::try_join!(
            flatten(http_server),
            flatten(input)
        );

        //TODO: Schließroutine anlegen:
        // -> websocket "schieß"-Nachricht absenden
        // -> Mutex für websocket freigeben
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