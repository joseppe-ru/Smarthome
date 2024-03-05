# Das FrWeb-UI
Website für den Client zum interagieren mit dem Smarthome

dynamischer Aufbau über JSON:
 - JSON-Datenbank wird über Websocket Kommuniziert
 - (Datenbank siehe PiSer-Modul)
 - für jeden Geräte-Typ gibt es eine eigene Klasse
 - Die Klasse ist zuständig für:
   - darstellung im Frontend - erzeugt HTML-DOM-Element
   - Kommunikation über Websocket MQTT
   - Speichern der daten aus Datenbank