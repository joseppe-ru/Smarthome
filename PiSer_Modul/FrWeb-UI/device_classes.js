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
        let buf = new ArrayBuffer(8);
        for (let i=0;i<8;i++){
            buf[i]=i
        }
        ws.send(buf)
    }
}