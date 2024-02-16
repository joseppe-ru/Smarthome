import {Schalter} from "/device_classes.js";
import mqtt from "/mqtt.js  ";

const BACKEND_URL_WS ="wss://"+ window.location.hostname+":9231/" //Basis-Pfad für den Server
const BACKEND_URL_MQTT="mqtts://"+window.location.hostname+":1884/"

//öffnet den initialisierungs Websocket
const init_ws_socket = new WebSocket(BACKEND_URL_WS+"websocket")
//Webscoket Callbacks/Events hinzufügen
init_ws_socket.onopen=()=>{console.log("init_Websocket opened");init_ws_socket.send("connected client")}
init_ws_socket.onmessage=(msg)=>handle_server_message(msg)
init_ws_socket.onerror=(err)=>console.error("init_Websocket: err: ",err)
init_ws_socket.onclose=()=>console.log("init_Websocket closed")

let clientId="mqtt_17725"

let options={
    keepalive:60,
    clientId,
    protocolId:'MQTT',
    protocolVersion:4,
    clear:true,
    reconnectPeriod: 1000,
    connectTimeout: 1000,
    will:{
        topic:'test',
        payload:'connection closed abnormaly',
        qos:0,
        retain:false
    },
    rejectUnauthorized:false
}
const mqtt_client = mqtt.connect(BACKEND_URL_MQTT)
console.log(mqtt_client)
//Dieses Array beinhaltet alle Geräte, die eingerichtet worden (also die Objekte der Klassen)
const all_devices = [];

//Dem Event ein Gerät zuordnen
function handle_events(e){
    for (let i=0;i<all_devices.length;i++){
        if (all_devices[i].ID == e.currentTarget.id){
            all_devices[i].event(init_ws_socket);
        }
    }
}

function handle_server_message(msg){
    let json = JSON.parse(msg.data) //string als Json-Objekt speichern
    for (let device in json.device_list){ //Alle angegebenen Geräte erstellen
        if(json.device_list.hasOwnProperty(device)){
            const dev = json.device_list[device]
            create_device(dev)
        }
    }
    console.log(all_devices)
}

function create_device(dev){
    let position

    //Positionsbestimmung
    if (dev.dev_id < 32){
        console.log("Gerät für Schlafzimmer einrichten: ",dev, " vom Typ: ",dev.type)
        position = document.getElementById("Schlafzimmer")
    }
    else if(dev.dev_id < 64){
        console.log("Gerät für Wohnzimmer einrichten: ",dev)
        position=document.getElementById("Wohnzimmer")
    }
    else if(dev.dev_id < 96){
        console.log("Gerät für Küche einrichten: ",dev)
        position=document.getElementById("Küche")
    }
    else{
        console.log("Gerät für Arbeitszimmer einrichten: ",dev)
        position=document.getElementById("Arbeitszimmer")
    }

    //Sortieren nach Typ
    switch (dev.type){
        case "Schalter":{
            //neues Gerät erstellen, Frontend elemente hinzufügen...
            let _switch=new Schalter(dev)
            let dom_obj=_switch.build_frontend(position)

            //das HTML-Element einer Position zuordnen und erstellen (zu HTML Body hinzufügen)
            position.appendChild(dom_obj)

            dom_obj.addEventListener('click',handle_events)
            //Das Gerät einer internen Geräteliste anfügen
            all_devices.push(_switch)

        }
    }
}


