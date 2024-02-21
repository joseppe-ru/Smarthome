use futures::{SinkExt, StreamExt};
use futures::stream::SplitSink;
use tokio::sync::oneshot;
use warp::{
    Filter,
    ws,
    Reply,
    ws::{Message, WebSocket},
};
use serde_json;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::fs;
use std::io::Read;
use rustc_serialize::json::Json;

/* # Hier ist der Endpunkt des Websockets "wss://[ip:port]/websocket"
 */
async fn handle_websocket_message(message:Message, _tx: &mut SplitSink<WebSocket,Message>){

    //message.my_protokoll_sting?
    //Zuordnung über datenbank
    //weiterleiten der Nachricht
    if message.is_text() {
        println!("Nachricht empfangen: {:?}",message);
    }else if message.is_binary() {
        println!("Daten empfangen...");

        //message entschlüsseln
        //vergleich mit Datenbank
        //alle notwendigen Informationen aus datenbank (zum Senden)
        //angehängte Nachricht schicken bzw. Nachrichten sind hier ja schon Gerätespezifisch

        //alle informationen, die ich brauche/haben will aus Datenbank ziehen/Vergleichen
        let mut file = File::open("nosql_ids/db.json").expect("Fehler beim Öffnen");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Fehler beim parsen in String");

        let json:serde_json::Value = serde_json::from_str(&data).expect("schuwidder e fahler");
        if let Some(dev_list)=json.as_object(){
            for (key,val) in dev_list{
                println!("list:{}",key);
                if let Some(device) = val.as_object(){
                    for (key,val)in device{
                        println!("dev: {}",key);
                        if let Some(prop)=val.as_object(){
                            for (key,val)in prop{
                                println!("prop: {}, val: {}",key,val);
                                //prop mit dev_id und val mit empfangener id aus message vergleichen
                                //dann kenne ich das Gerät und brauche dann nur noch die darauf folgenden parameter, welche zum Senden per mqtt wichtig sind.
                                //die nachricht muss noch aus empfangener message rausgezogen werden

                            }
                        }
                    }
                }
            }
        }

        println!("")
    }else {
        eprintln!("Websocket daten in unbekanntem Format!")
    }
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


/*
fn get_mime_type(file_path: &str) -> &str {
    match Path::new(file_path).extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html;charset=utf-8",
        Some("css") => "text/css;charset=utf-8",
        Some("js") => "application/javascript;charset=utf-8",
        _ => "application/octet-stream;charset=utf-8",
    }
}
*/


pub async fn http_server_setup(shut_channel_rx: oneshot::Receiver<()>) ->Result<(), &'static str> {
    //Pfad: vom FrWeb-UI (HTML dateien etc...)
    let backüath=std::path::Path::new("../");
    let curr_dir=std::env::current_dir().expect("failed to read current directory");
    let project_folder=curr_dir.join(backüath).join("FrWeb-UI");

    if !project_folder.is_dir(){
        panic!("[html-server] Falsche Pfadangabe!"); //falls ich was beim Pfad verkackt habe, wird beendet
    }

    //websocket route
    let ws_route = warp::path("websocket")
        .and(warp::ws())
        .map(|ws: ws::Ws|{
            println!("Connection was upgraded to websocket.");
            ws.on_upgrade(move |websocket| async move {

                handle_client(websocket).await;
            }
            )});

    //Routen für Website mit MIME-Typ
    let all_in_one_routes = warp::get()
        .and(warp::fs::dir(project_folder.clone()))
        .map(|reply: warp::filters::fs::File| {
            //println!("replyed path: {:?}",reply.path());
            if reply.path().ends_with("device_classes.js") {
                warp::reply::with_header(reply, "content-type", "text/javascript").into_response()
            }else if reply.path().ends_with("script.js") {
                warp::reply::with_header(reply, "content-type", "application/javascript").into_response()
            }else if reply.path().ends_with("style.css") {
                warp::reply::with_header(reply, "content-type", "text/css").into_response()
            }else {
                reply.into_response()
            }
        });

        let alternative_route = warp::get()
            .and(warp::fs::dir(project_folder.clone()))
            .map(|reply: warp::filters::fs::File|{
                println!("du bist doof!");
                println!("Pfad noch nicht geroutet: {:?}",reply.path());
                reply.into_response()
            });

    let routes=warp::get()
        .and(ws_route
            .or(all_in_one_routes));



    //Certificate: openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.rsa -out cert.pem
    let (_,server)=warp::serve(routes)
        //.tls()
        //.cert_path("cert.pem")
        //.key_path("key.rsa")
        .bind_with_graceful_shutdown(([0,0,0,0],9231),async { shut_channel_rx.await.ok(); });

    // Spawn the server into a runtime
    tokio::task::spawn(server);


    return Ok(())
}