console.log("Hello Munke from the other side");

const BACKEND_URL ="wss://"+ window.location.hostname+":9231/websocket"


const socket = new WebSocket(BACKEND_URL)
socket.onopen=()=>{
    console.log("Websocket Opendned")
}
socket.onmessage=(msg)=>alert(msg.data)
socket.onerror=(err)=>console.error(err)
socket.onclose=()=>console.log("Socket closed")


function ws_send(){
    console.log("ws_send Button")
    socket.send(1);
}