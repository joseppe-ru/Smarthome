//#include <memory>

#include "IRremote_Sensor_Actor_System.hpp"
#include <IRremote.h>
#include "base_definitions.h"

IRrecv IRrecv_OS_1838B;
IRsend irsend;

Sensor_Actor_System::Sensor_Actor_System(int recv_pin, int recv_feedback_pin, int send_pin, int send_feedback_pin){
  //Pinzuweisung für membervariablen dieser Klasse
  Pin_Recv=recv_pin;
  Pin_Recv_Feedback=recv_feedback_pin;
  Pin_Send=send_pin;
  Pin_Send_Feedback=send_feedback_pin;
}

/**
 * # Sensor_Actor_System::Init
 * @brief Initialisierungsfunktion für Sensor und IR-Diode sowie Feedback LEDs
*/
void Sensor_Actor_System::Init(){
  printActiveIRProtocols(&Serial);
  //Empfänger init:
  IRrecv_OS_1838B.enableIRIn();
  pinMode(Pin_Recv_Feedback, OUTPUT);  //FeedbackLED auf OUtput schalten?? 
  IRrecv_OS_1838B.begin(Pin_Recv,true,Pin_Recv_Feedback); 
  //Sender init:
  pinMode(Pin_Send_Feedback, OUTPUT);  //FeedbackLED auf OUtput schalten?? 
  irsend.begin(Pin_Send,true,Pin_Send_Feedback);
}

/**
 * # Sensor_Actor_System::Recive
 * @note überprüft den Sensor
 * @return true: wenn daten erkannt wurden; -> bedeutet, schleife kann weiter gehen
 * @return false: wenn keine Daten empfangen werden konnten -> bedeutet, schleife beenden
*/
bool Sensor_Actor_System::Recive(IRData_s*& received_data){
    struct IRData_s recv_data; //Empfangene Daten (speicherplatz reservien)
    if (IRrecv_OS_1838B.decode()) {
      if(IRrecv_OS_1838B.decodedIRData.protocol==UNKNOWN){  //Falsche Daten
        return true;
      }
      //recived_data=IRrecv_OS_1838B.decodedIRData;//??
      //Ausgabe der Empfangenen Daten
      IRrecv_OS_1838B.printIRResultShort(&Serial);
      IRrecv_OS_1838B.printIRSendUsage(&Serial);
      IRrecv_OS_1838B.resume();

      //Daten übertragen 
      //TODO:  Protokollauswertung??? IRrecv_OS_1838B.decodedIRData.command
      received_data = new IRData_s{NEC_e,IRrecv_OS_1838B.decodedIRData.command}; //Zeiger auf empfanene daten; neuen Speicherplatz allocieren, wird nach abspeichern wieder gelöscht
      return false; //Rückgabewert mit Informationen zum Senden
    }
    else
    {
      received_data = nullptr;
      return true;
    }
}

//TODO: Implementierung für Weiter Protokolle ??
/**
 * # Send
 * @brief sendet das angegebene Commando 
 * @param protokoll Art des Protokolls
 * @param adr addressse ict irgendwie immer 0
 * @param cmd der Command ist das entscheidende
 * @param repeats häufigkeit der Wiederhohlungen
 * @return 12-gesendet per NEC protokoll;
 * @return  -77-wenn Protokoll nicht erkannt oder Unbekannt
*/
int Sensor_Actor_System::Send(protokoll_type_t protokoll, int adr, int cmd, int repeats){
  switch (protokoll){
    case NEC_e:{
      Serial.println("NEC_e Senden...");
      irsend.sendNEC(adr, cmd, repeats);
      return 12;
    }
    case UNKNOWN_e:
    default:
    Serial.println("nicht Senden...");
      return -77;
  }
}
