# Project Smarthome

Selbstprogrammiertes Smarthome soll auf Basis eines Raspberry Pi's laufen.
Workaround:
  - HTTP-Server mit Websocket in Rust
  - Datenbank (JSON-Datei), in der ich neue Geräte, die ich einrichten möchte hinzufüge
  - Website mit JavaScript, baut sich dynamisch nach den angeben aus der Datenbank auf
  - MQTT-Broker in Rust (parallel zum HTTP-Server)
  - Javascipt-MQTT-Client für Benutzer (Darstellung von Daten der Smarthomegeräte und senden von Steuerungssignalen)
  - MQTT-Client für Raspberry (oder andere WIFI fähige Geräte/Mikrocontroller)
  - MQTT-Client-Endgeräte haben andere Kommunikatinsprotokolle implementiert, die die Verbindung zur Hardware/den Steuerungsendgeräten herstellt.

MQTT soll als das Kommunikationsmittel für das Benutzerinterface dienen, alle anderen Funktionaitäten sind dann in den MQTT-Client-Endgeräten implementiert (Funkanbindung zu einem Schalter, SPI zum Arduino, BLE, BLE Mesh?, IR,...)
Alle Parameter, die zur Steuerung (über verschiedene Protokolle hinweg) des Smarthomes/anteuerung der Geräte notwendig sind, sollen in der JSON-Datenbank festgehalten werden.
Die Datenbank wird per Websocket an den HTTP-Client übermitteln und in JavaScript in Für die Geräte angepasste Klassen übergeben.
