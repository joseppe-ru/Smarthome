# Arduino_IR_Remote

Entwurf einer Universellen IR-Fernbedienung mit dem Arduino

 - Steuerung über Rotary Encoder,
 - Menue/auswahl über LCD-Display,
 - Empfangen des IR-Signals einer Fernbedienung 
   -> Speichern zur Laufzeit (temporär)
 - Senden eines IR-Signals an Gerät
   -> vorher bzw. zur Laufzeit gespeicherte Signale wiedergeben

   Der Quellcode besteht zunächst aus Testprojekten zu den eizelnen Komponenten.
   Das Hauptprogramm befindet sich unter "_merged".

Ansteuerung über Raspberry (*NICHT* teil dieser Belegarbeit)
 - Datenbus: I2C
 - Abspeicherung der Möglichen Sende-Signale
 - übergeben dieser Codes zum ansteuren der Geräte
