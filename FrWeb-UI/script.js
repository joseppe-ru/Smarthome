//Endpunkt (Adresse) vom WebSocket (HTTPS-Server)
const BACKEND_URL_WS ="ws://"+ window.location.hostname+":9231/" //Basis-Pfad für den Server
//import in HTML -> Paho ist als 'const' deklariert und deshalb erreichbar
const client=new Paho.MQTT.Client('localhost',1886,'/',"FrWeb-UI");
//Initialisiert den Websocket
const socket = new WebSocket(BACKEND_URL_WS+"websocket")
//Die Geräteklassen (wie 'Schalter') sind ebenfalls als 'const' definiert und aus device_classes.js in HTML importiert

const options = {
    useSSL: false,
    timeout: 3,
    onSuccess: onConnect,
    onFailure: onFailure,
    //cleanSession : true,
    keepAliveInterval:60,
    // Authentifizierung
    //userName: "client072", // Benutzername
    //password: "", // Passwort
};

const sub_options={
    qos:1,
    invocationContext:"d",
    onFailure:fail,
    onSuccess:succ,
    timeout:3
}

//Callbacks für MQTT:
client.connect(options);
client.onMessageArrived=function handle_mqtt_message(e) {console.log(e._getPayloadString())}

function fail(){console.log("failed to sub")}
function succ(){console.log("sub has worked")}
function onConnect() {console.log('Connected to MQTT broker');client.subscribe('test')}
function onFailure(errorMessage) {console.error('Failed to connect to MQTT broker:', errorMessage);}



//Websocket Callbacks:
socket.onopen=()=>{console.log("Websocket opened");socket.send("client connected: "+window.location.hostname)}
socket.onmessage=(msg)=>handle_server_message(msg)
socket.onerror=(err)=>console.error("Websocket: err: ",err)
socket.onclose=()=> {console.log("Websocket closed");window.location.reload();}

//Dieses Array beinhaltet alle Geräte, die eingerichtet worden (also die Objekte der Klassen)
const all_devices = [];

function isjson(str){
    try{
        JSON.parse(str.data)
        return true
    }
    catch (e){
        return false
    }
}

//Welches Gerät hat das Event getriggert?
function handle_events(e){
    for (let i=0;i<all_devices.length;i++){
        if (all_devices[i].ID == e.currentTarget.id){
            all_devices[i].event(socket);
        }
    }
}

function handle_server_message(msg){
    if (isjson(msg)){
        let json = JSON.parse(msg.data) //string als Json-Objekt speichern
        for (let device in json.device_list){ //Alle angegebenen Geräte heraussuchen
            if(json.device_list.hasOwnProperty(device)){
                const dev = json.device_list[device]
                create_device(dev)
            }
        }
        console.log(all_devices)
    }
    else {
        console.log(msg)
    }
}

function create_device(dev){

    let position
    //Positionsbestimmung (Aufteilung nach id, 32 Geräte pro Raum)
    if (dev.dev_id < 32){
        position = document.getElementById("Schlafzimmer")
    }else if(dev.dev_id < 64){
        position=document.getElementById("Wohnzimmer")
    }else if(dev.dev_id < 96){
        position=document.getElementById("Küche")
    }else{
        position=document.getElementById("Arbeitszimmer")
    }

    //Geräte erstellen nach Typ
    switch (dev.type){
        case "Schalter":{
            let _switch=new Device_Classes.Schalter(dev) //Geräteklasse instanziieren
            let dom_obj=_switch.build_frontend(position) //HTML-Objekt (Gerätespezifisch) erzeugen
            position.appendChild(dom_obj) //Element zu Tap (Raum) hinzufügen
            dom_obj.addEventListener('click',handle_events) //Event + Handler fkt einrichten
            all_devices.push(_switch) // Geräteobjekt der Geräteliste anhängen
            break
        }
        default:{
            console.log("Gerätetyp noch nicht eingerichtet...")
            break
        }
    }
}