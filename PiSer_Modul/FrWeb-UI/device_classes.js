class Device_class_Parent {
    //alle Informationen aus JSON entnehmen und in Klasse speichern
    constructor(dev){
        //Die Membervariablen der Elternklassen müssen immer vorhanden sein (auch in JSON)
        this.Name = dev.name
        this.Type = dev.type
        this.ID = dev.dev_id
    }

    //funktion zum Erstellen der Anzeigeelemente
    build_frontend(position){
        return new HTMLElement()
    }

    //ausführen der Funktionalität
    event(ws){
    }

    //Konvertieren der zu sendenden Werte in ein Bytearray
    Data_to_Bytearray(){}
}

export class Schalter extends Device_class_Parent{
    constructor(dev) {
        super(dev);
    }

    //gibt mir das erstellte HTML-Element zurück
    build_frontend(position){
        console.log("Schalter '",this.Name,"' wird erstellt");
        //HTML-Element erzeugen
        const sw = document.createElement("Button");
        sw.innerText=this.Name
        sw.id=this.ID
        sw.name=this.Type

        //Rückgabe des HTML-Elementes
        return sw
    }

    event(ws) {
        console.log(this.Name, this.ID, this.Type);
        //daten als JSON versenden
        mqtt.publish()
        ws.send(this.Data_to_Bytearray())
    }

    Data_to_Bytearray(toggle_state) {
        let bytes=new Uint8Array(8)
        for (let i=0;i<bytes.length;i++){
            bytes[i]=i;
        }
        return bytes
    }
}