#PiSer Modul
Das Pi-Server-Modul soll auf einem Raspberry Pi Laufen und zum einen den Endpunkt des Webservers und zum anderen den MQTT-Broker darstellen.

Der HTTP-Web-Server:
 - Hostet das FrWeb-UI (meine Website)
 - über Websocket dynamischen Aufbau ermöglichen

Der MQTT-Broker:
 - Grundlage für die Kommunikation von Raspberrys oder anderen eigenen Smarthome-Komponenten untereinander
 - MQTT ist die einzige Schnittstelle zwischen Website (Client) und den andern Endgeräten 
  - (die könen wiederum über andere Prtokolle erweiterte Hardware ansteuern)
 
 nosql-ids:
  - JSON Datanbank
  - verfügt über Daten zu den Einzelnen Endgeräten im Smarthome-Komponenten
    - Koomunikations information
	- Position
	- Typ
	- ID
	- Name
	...