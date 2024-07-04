// #Arduino Empfänger Test
/*
 * Anschlussplan am UNO R3:
 *  5V -> auf Steckbrett
 *  GND -> auf Steckbrett
 *  IR_RECIVER:
 *    - VCC (IR_Reciver) -> 5V (Steckbrett)
 *    - GND (IR_Reciver) -> GND (Steckbrett)
 *    - DatenPin (IR_Reciver) -> D2 (UNO)
 *  Feedback LED:
 *    - D5 (UNO) -> +LED- -> 220 Ohm Widerstand -> GND
 *  
*/
#include <IRremote.h>

#define IR_RECEIVE_PIN 2 //Pin am Arduino UNO für den IR Receiver
#define IR_RECEIVE_FEEDBACK_LED 5

IRrecv irrecv;

void setup() {
 
  irrecv.enableIRIn(); //Den IR Pin aktivieren
  Serial.begin(9600); //Serielle kommunikation mit 9600 Baud beginnen.

  irrecv.begin(IR_RECEIVE_PIN, true,IR_RECEIVE_FEEDBACK_LED); //Pin zum enpfangen und für signal led initialisieren
  printActiveIRProtocols(&Serial);
}

void loop() {
  if (irrecv.decode()) { //Wenn etwas gelesen wurde dann...
      
      //Ausgabe über serielle Schnittstelle
      irrecv.printIRResultShort(&Serial);
      irrecv.printIRSendUsage(&Serial);
      
      if (irrecv.decodedIRData.protocol == UNKNOWN) {
        Serial.println(F("Received noise or an unknown (or not yet enabled) protocol"));
        // We have an unknown protocol here, print more info
        irrecv.printIRResultRawFormatted(&Serial, true);
      }
      
      Serial.println();

      
    //if (irrecv.getProtocolString() == "SONY") {}  //auf Protokoll überprüfen
    
    irrecv.resume(); // auf den nächsten Wert warten
    
  }
}
