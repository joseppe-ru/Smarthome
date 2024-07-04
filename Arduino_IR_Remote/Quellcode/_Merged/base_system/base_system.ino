#include <stdint.h>
#include <Arduino.h>

#include "base_definitions.h"                   //Pin-definitionen, Enum für Displayschnittstelle, Enum und Strucktur für Menüeinträge, Enum und Struckturen für IR-Daten
#include "IRremote_RE_Input_Control.hpp"        //Steuerung/Eingabe
#include "IRremote_Display_Control_System.hpp"  //Display anzeige Funktionen und Spezielle Display initialisierungen
#include "IRremote_Menu_Control_Layer.hpp"      //Menü initialisierungen und zugriffsfunktionen
#include "IRremote_Sensor_Actor_System.hpp"     //IR Sensor und Empfänger

//Klassen initialisierungen
RE_Input_Control Drehschalter(Pin_RE_CLK, Pin_RE_DT, Pin_RE_SW);                                      //Drehschalter-Klasse Initialisieren -> zum Steuern der Ein-/Ausgabe
Display_Control_System lcd_display(Pin_LCD_RS,Pin_LCD_E,Pin_LCD_D4,Pin_LCD_D5,Pin_LCD_D6,Pin_LCD_D7); //Display-Klasse Initialisieren
Menu_Control_Layer Menu;                                                                              //Menü
Sensor_Actor_System IR_System(Pin_IR_RECV,Pin_IR_RECV_FEEDBACK,Pin_IR_LED,Pin_IR_LED_FEEDBACK);       //Sender & Empfänger

//globale Variablen                                     
int curr_menu_index = 0;                               //Zähler -> Aktueller Menüeintrag
enum Menu_Titles curr_Title = MENU_TITLE_GREETINGS_e;  //aktueller Menütitel (start mit Greetings)
struct Menu_Entry_s *curr_entry;                       //Speichervariable für aktuellen Menüeintrag
struct IRData_s *received_data_ptr = nullptr;          //empfangene Daten vom reciever als Pointer (ist besser zum auswerten)


//Initialisierungsfunktion
void setup() {
  Serial.begin(9600);      //Serielle Kommunikation;
  Serial.println("Setup");
  lcd_display.Init();      //Display
  Drehschalter.Init();     //Drehschlater
  IR_System.Init();        //IR-Sender und Empfänger
  display_menu();          //Display aktualisieren
}

/**
 * # display_menu
 * @brief Aktualisiert das Display
 * @note ermittelt über aktuellen Menü-Titel und aktuellen Index den aktuellen und nachfolgenden Menüeintrag und übergibt beie bezeichnungendem Display 
*/
void display_menu(){     //Wenn DISPLAY_UP_e || DISPLAY_DOWN_e
  String entry_text[2];  //Anzeigetext für das Display

  curr_entry = Menu.Get_Entry(curr_Title,curr_menu_index);  
  entry_text[0] = "->"+curr_entry->name;
  curr_entry = Menu.Get_Entry(curr_Title,curr_menu_index+1);
  entry_text[1] = curr_entry->name;
  
  lcd_display.Update_Display_Text(entry_text[0],entry_text[1]);

  //TODO: Testausgabe:
  Serial.println("Text 1: "+entry_text[0]);
  Serial.println("Text 2: "+entry_text[1]);
  Serial.print("Index: "); Serial.println(curr_menu_index);
  Serial.print("Titel: "); Serial.println(curr_Title);
  
  if(received_data_ptr==nullptr){
    Serial.println("RecvPTR = 0");
  }
  else{
    Serial.println("RecvPTR = Data");
  }
  
  return;
}

/**
 * # check_menue_entry 
 * @brief Überprüft den Nachfolger (nächstes Menü anzeigen oder Funktion ausführen)
 * @note Menue_Entry_s.followed_by ... FUNKTION, MENÜ_TITlE_...
 * @return 33 Neue Daten wurden abgespeichert;
 * @return -69 Wenn du hier raus kommst, dann hast du vergessen, ein Menü hinzuzufügen im switch-case;
 * @return 5 empfangen abgebrochen durch Drehknopf;
 * @return 66 receivevorgang normal beendet;
 * @return -77, wenn Protokoll nicht erkannt oder Unbekannt;
 * @return 12 gesendet per NEC protokoll;
 * @return -38 Menüeintrag konnte nicht korrekt abgerufen werden
*/
int check_menue_entry(){
  int ret=0;
  curr_entry = Menu.Get_Entry(curr_Title,curr_menu_index);  //Aktuellen Menüeintrag hohlen

  if(curr_entry == nullptr)
    return -38;

  if(curr_entry->followed_by==FUNKTION_e){  //Menüeintrag: soll als Nachfolger eine Funktion ausführen 
    switch (curr_Title){                    //Switch über den Aktuellen Menü-Titel
      case MENU_TITLE_SEND_e:{
        if(received_data_ptr == nullptr){    //es wurden keine neuen Daten empfangen -> normales Senden          
          ret = send_irremote(curr_entry);   //Senden Eines IR-Signals
        }  
        else{                                                                   //ich will neue Daten (kürzlich empfangen) abspeichern  
          ret = Menu.Manipulate_Entry_from_Data(MENU_TITLE_SEND_e,curr_menu_index,received_data_ptr);
          delete received_data_ptr;                                             //Speicherplatz wieder freigeben
          curr_Title = MENU_TITLE_START_e;                                      //dann ins Start-Menü übergehen                                                          
        }
        received_data_ptr = nullptr;                                            //Zeiger auf Empfangene Daten zurücksetzen (sonst undefiniertes verhalten)
        break;    
      }
      case (MENU_TITLE_RECEIVE_e):{
        curr_Title=MENU_TITLE_SEND_e;  //dann ins Sende-Menü übergehen
        ret = recive_irremote();       //empfang-Schleife aufrufen
        break;
      }
      default:{
        curr_Title = MENU_TITLE_START_e;  //dann ins Start-Menü übergehen
        ret = -69;                        //Wenn du hier raus kommst, dann hast du vergessen, ein Menü hinzuzufügen
        break;
      }
    }
  }
  else{                                    //es folgt der Wechsel in ein anderes Menü 
    curr_Title = curr_entry->followed_by;  //Menü-Titel Aktualisieren 
  }
  return ret;
}

//Diese Funktion wird als callback an den Drehschalter übergeben
void base_system(Display_Commands cmd){
  int checker = Menu.Check_Index(curr_Title,curr_menu_index);
  int err;

  switch (cmd) {
    case DISPLAY_UP_e:{
      if(!(checker==1))     //Index ist nicht auf Maximal 
        curr_menu_index++;  //-> Der index darf incrementiert werden
      break;
    }
    case DISPLAY_DOWN_e:{
      if(!(checker==2))     //Index ist nicht auf Minimal
        curr_menu_index--;  //-> Der index darf decrenemtiert werden
      break;  
    }
    case DISPLAY_OK_e:{
        err = check_menue_entry();
        curr_menu_index = 0;  //index zurückstezten
        break;
    }
    default:
      break;
  }

  if(err)  //Fehlermeldung ausgeben
    Serial.print("Fehlerhaft beendet; err: "); Serial.println(err);

  display_menu();  //display aktualisieren
  return;
}

/**
 * # IR-Sendefunktion ausführen 
 * @return -77, wenn Protokoll nicht erkannt oder Unbekannt
 * @return 12 gesendet per NEC protokoll
*/
int send_irremote(Menu_Entry_s *curr_entry){
  //TODO: Protokoll auswertung fehlt noch (NEC_e,Sony...)
  digitalWrite(Pin_IR_LED_FEEDBACK, HIGH);            //Feedback LED anschalten
  lcd_display.Update_Display_Text("Senden",". . .");  //Ladeanzeige
  Serial.println("Signal wird gesendet");
  int ret= IR_System.Send(NEC_e,0x00,curr_entry->data.command,5);
  delay(100);
  digitalWrite(Pin_IR_LED_FEEDBACK, LOW);  //Feedback LED anschalten
  return ret;
}

/**
 * @return 5 empfangen abgebrochen durch Drehknopf
 * @return 66 receivevorgang normal beendet
*/
int recive_irremote(){
  
  digitalWrite(Pin_IR_RECV_FEEDBACK, HIGH);  //Feedback LED anschalten

  lcd_display.Update_Display_Text("Empfangen",". . .");  //Ladeanzeige
  
  while (IR_System.Recive(received_data_ptr)) //Schleife zum Empfangen der Daten
  {

    if(Drehschalter.Checkup(nullptr)){          //Unterbrechung durch Drehschalter möglich
      digitalWrite(Pin_IR_RECV_FEEDBACK, LOW);  //Feedback LED ausschalten
      return 5;                                 //Abbruch durch Eingabe am Drehknopf   
    }
  }

  Serial.println("==Neue Daten: ==");
  Serial.print("Protokoll: ");Serial.println(received_data_ptr->protocol);
  Serial.print("Command: ");Serial.println(received_data_ptr->command);
  digitalWrite(Pin_IR_RECV_FEEDBACK, LOW);  //Feedback LED ausschalten  
  return 66;
}

void loop() {
  //Überprüft die Eingabe des Drehschalters -> führt bei Eingabe die übergebene Callbackfunktion aus
  Drehschalter.Checkup(base_system);
}
