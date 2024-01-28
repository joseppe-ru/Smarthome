use std::convert::Infallible;
use std::net::SocketAddr;

use std::fs;
use std::path::{PathBuf,Path};
use http_body_util::Full;
use hyper::body::{Body, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper::header::FROM;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

fn extract_file_extension(path:&str)->Option<&str> {
    if path == "/"{
        return Some(path);
    }
    else if let ext = path.rsplit(".").next(){
       return ext;
    }
    return None;
}

async fn cli_request(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    // Extrahiere den Pfad aus der Anfrage
    let req_path = req.uri().path();
    println!("searched for: {}",req_path);
    let ext_path = extract_file_extension(req_path);
    let base_path_str = "/home/russe/Coding/Smarthome/FrWeb-UI";

    //gesuchten Pfad im Browser auswerten
    match ext_path {
        Some("/") => {
            println!("Root-Pfad");

            let file_path = Path::new(base_path_str).join("index.html");
            // Überprüfe, ob die Datei existiert und lesbar ist
            if file_path.is_file() {
                // Wenn die Datei existiert, sende sie als Antwort
                if let Ok(contents) = fs::read(&file_path) {
                    return Ok(Response::new(Bytes::from(contents).into()));
                }
            }
        },
        Some("js")=>{
            println!("script Pfad");
            let file_path = Path::new(base_path_str).join(req_path);
            if file_path.is_file() {
                // Wenn die Datei existiert, sende sie als Antwortfi
                if let Ok(contents) = fs::read(&file_path) {
                    return Ok(Response::new(Bytes::from(contents).into()));
                }
            }
        },
        None => println!("in progress..."),
        _=>println!("Fehler, falscher Typ"),
    }
    Ok(Response::new(Bytes::from("Hello Munke!").into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); //Addresse muss 0.0.0.0 sein, weil ich auf alle Geräte im Netzt hören will!!

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;
	println!("HTTP Server hostet at http://{}/",addr);
    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            //welcher teil meiner website (pfad) wurde aufgerufen
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(cli_request)).await  // `service_fn` converts our function in a `Service`
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
