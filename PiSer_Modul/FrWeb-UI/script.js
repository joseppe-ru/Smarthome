const BACKEND_URL ="wss://"+ window.location.hostname+":9231/" //Basis-Pfad für den Server

//öffnet den initialisierungs Websocket
const init_ws_socket = new WebSocket(BACKEND_URL+"websocket")

//Webscoket Callbacks/Events hinzufügen
init_ws_socket.onopen=()=>{console.log("init_Websocket opened")}
init_ws_socket.onmessage=(msg)=>handle_server_message(msg)
init_ws_socket.onerror=(err)=>console.error("init_Websocket: err: ",err)
init_ws_socket.onclose=()=>console.log("init_Websocket closed")


//diese Funktionen müssen für das übergebene Gerät Anzeigeelemente erschaffen und die nötigen Trigger/Events erstellen...
function create_bedroom_device(dev){
    console.log("Gerät für Schlafzimmer einrichten: ",dev)
}

function create_livinroom_device(dev){
    console.log("Gerät für Wohnzimmer einrichten: ",dev)
}

function create_kitchen_device(dev){
    console.log("Gerät für Küche einrichten: ",dev)
}

function create_gamingroom_device(dev){
    console.log("Gerät für Arbeitszimmer einrichten: ",dev)
}

//gegenstück für Geräte muss in JS vorhanden sein.
//Wie mit dem Gerät in JS umgegangen wird, kann nicht in Jason festgelegt sein
//das Json-Datenbank-file beinhaltet nur werte, welche für Darstellung un Kommunikation wichtig sind
//Geräte ID ist nach Anbringungsort aufgeteilt:
// 0...31 -> Schlafzimmer
// 32...63 -> Wohnzimmer
// 64...95 -> Küche
// 96...127 -> Arbeitszimmer
//Typen für Kommunikationsarten festgelegt: (b -> 1bit, x -> x-fache-bit)
// bout_bin -> Schalter
// xout_bin -> Anzeige, Update
// bout_xin -> Nachricht
// xout_xin -> Chat

function handle_server_message(msg){
    let json = JSON.parse(msg.data) //string als Json-Objekt zurückgeben
    //console.log(json)
    for (device in json.device_list){ //
        if(json.device_list.hasOwnProperty(device)){
            const dev = json.device_list[device]
            //console.log(dev)
            if (dev.dev_id < 32){
                create_bedroom_device(dev)
            }
            else if(dev.dev_id < 64){
                create_livinroom_device(dev)
            }
            else if(dev.dev_id < 96){
                create_kitchen_device(dev)
            }
            else if(dev.dev_id < 96){
                create_gamingroom_device(dev)
            }
        }
    }
}