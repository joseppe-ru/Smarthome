#include "IRremote_Menu_Control_Layer.hpp"
#include "base_definitions.h"
#include <Arduino.h>

Menu_Control_Layer::Menu_Control_Layer(){
    Init_Menu_Arrays();
}

/**
 * # Init_Menu_Arrays
 * @brief füllt die Menüeinträge mit Startwerten
*/
void Menu_Control_Layer::Init_Menu_Arrays(){
    IRData_s null_data = {UNKNOWN_e,0x00};
    
    //Begrüßungs menuü anlegen
    Greetings_Menu[0]={"Hello Munke",null_data,MENU_TITLE_START_e};
    Greetings_Menu[1]={"copyright <R>",null_data,MENU_TITLE_START_e};

    //Startmenü anlegen:
    Start_Menu[0]={"Senden",null_data,MENU_TITLE_SEND_e};
    Start_Menu[1]={"Empfangen",null_data,MENU_TITLE_RECEIVE_e};
    Start_Menu[2]={"back",null_data,MENU_TITLE_GREETINGS_e};

    //SendenMenü anlegen:
    Send_Menu[0]={"Ein/Aus",{NEC_e,0x45},FUNKTION_e};
    Send_Menu[1]={"Rot",{NEC_e,0x46},FUNKTION_e};
    Send_Menu[2]={"Blau",{NEC_e,0x15},FUNKTION_e};
    Send_Menu[3]={"Grün",{NEC_e,0x47},FUNKTION_e};
    Send_Menu[4]={"Bunt",{NEC_e,0x44},FUNKTION_e};
    Send_Menu[5]={"Vor>>",{NEC_e,0x43},FUNKTION_e};
    Send_Menu[6]={"Play/Pause",{NEC_e,0x40},FUNKTION_e};
    Send_Menu[7]={"cmd7",{NEC_e,0x07},FUNKTION_e};
    Send_Menu[8]={"cmd8",{NEC_e,0x09},FUNKTION_e};

    Send_Menu[9]={"back",{NEC_e,0x00},MENU_TITLE_START_e};

    //Empfangsmenü
    Receive_Menu[0]={"Start  ->",null_data,FUNKTION_e};
    Receive_Menu[1]={"Abbruch X",null_data,MENU_TITLE_START_e};   
    return; 
}

/**
 * # Get_Entry
 * @brief gibt Pointer auf einen Menüeintrag zurück
 * @param title aktueller Menü-title
 * @param index aktueller index -> Menüeintrag
 * @return Pointer auf bestehenden Menüeintrag;
 * @return Null-Pointer, wenn eintrag nicht gefunden
*/
Menu_Entry_s* Menu_Control_Layer::Get_Entry(enum Menu_Titles title,int index){  
    switch (title){
        case MENU_TITLE_GREETINGS_e:{
          if (index > (sizeof(Greetings_Menu)/sizeof(Greetings_Menu[0]))-1){  //wenn der Index größer als die anzahl der elemente ist
            break;
          }
          return &Greetings_Menu[index];
        }
        case MENU_TITLE_START_e:{
          if (index > (sizeof(Start_Menu)/sizeof(Start_Menu[0]))-1){  //wenn der Index größer als die anzahl der elemente ist
            break;
          } 
          return &Start_Menu[index];
        }
        case MENU_TITLE_SEND_e:{
          if (index > (sizeof(Send_Menu)/sizeof(Send_Menu[0]))-1){  //wenn der Index größer als die anzahl der elemente ist
            break;
          } 
          return &Send_Menu[index];
        }
        case MENU_TITLE_RECEIVE_e:{
          if (index > (sizeof(Receive_Menu)/sizeof(Receive_Menu[0]))-1){  //wenn der Index größer als die anzahl der elemente ist
            break;
          }
          return &Receive_Menu[index];
        }
        case FUNKTION_e:
        default:{
          break;
        }
    }
    return nullptr;
}

/**
 * # Manipulate_Entry_from_Data
 * @brief überschreibt bestehenden Menüeintrag mit neuen Daten
 * @param title aktueller Menü-title
 * @param index aktueller index -> Menüeintrag
 * @param received_data_ptr Pointer auf Daten (IRData_s)
 * @return -22 beim fehlerhaften abfragen der bestehenden Strucktur;
 * @return 0 bei erfolgreichem Abschluss
*/
int Menu_Control_Layer::Manipulate_Entry_from_Data(enum Menu_Titles title,int index,struct IRData_s *received_data_ptr){
  //Zeiger auf alten eintrag
  old_entry_ptr = Get_Entry(title,index);
  if(old_entry_ptr==nullptr){
    return -22;
  }
  //Überschreiben des Eintrages
  old_entry_ptr->name="new"+(String)index;                   //neuer name mit index
  old_entry_ptr->data.protocol=received_data_ptr->protocol;  //Protokoll übertragen 
  old_entry_ptr->data.command=received_data_ptr->command;    //Komando übertragen
  old_entry_ptr->followed_by=FUNKTION_e;                     //als Funktion Markieren

  old_entry_ptr = nullptr;    //Pointer wieder Zurücksetzten
  return 0;
}

/**
 * # Check_Index
 * @brief überprüft, ob der anegebene Index sich innerhalb des Menü-Array-Bereiches
 * @note es kann eine Obere Grenze oder eine untere Grenze erreicht werden
 * @return 0, wenn kein Grenzfall erreicht ist (index in mitte des Arrays)
 * @return 1, wenn obere Grenze des Arrays erreicht
 * @return 2, unter Grenze des Arrays errreicht
 * (@return 3, beides trifft zu -> excludiert)
*/
int Menu_Control_Layer::Check_Index(enum Menu_Titles title,int index){

  //Maximalen Index für das jeweilige Menü berechnen
  switch (title){
    case MENU_TITLE_START_e:{
      max_menu_index = (sizeof(Start_Menu)/sizeof(Start_Menu[0]))-1;
      break;
    }
    case MENU_TITLE_SEND_e:{
      max_menu_index = (sizeof(Send_Menu)/sizeof(Send_Menu[0]))-1;
      break;
    }
    case MENU_TITLE_RECEIVE_e:{
      max_menu_index = (sizeof(Receive_Menu)/sizeof(Receive_Menu[0]))-1;
      break;
    }
    case MENU_TITLE_GREETINGS_e:{
      max_menu_index = (sizeof(Greetings_Menu)/sizeof(Greetings_Menu[0]))-1;
      break;
    }
    case FUNKTION_e:
    default:{
      max_menu_index = 0;
      break;
    }
  }

  if (index >= max_menu_index) //Obere Grenze
    return 1;

  if (index <= 0)  //Untere Grenze
    return 2;

  return 0;
}
