#ifndef MENU_CONTROL_LAYER_HPP
#define MENU_CONTROL_LAYER_HPP

#include "base_definitions.h"

/**
 * # Diese Klasse verwaltete Menü-Einträge
 * Funktionen:
 *  - Init_Menue_Arrays -> initialisert die Menü-listen (im konstruktor aufgerufen)
 *  - Get_Entry -> hat als Rückgabe wert die Struktur eines Eintrages 
 *  - Manipulate_Entry -> kann einen Eintrag überschreiben
*/
class Menu_Control_Layer{

public:
  //konstruktor
  Menu_Control_Layer();

  Menu_Entry_s* Get_Entry(enum Menu_Titles title,int index);
  int Manipulate_Entry_from_Data(enum Menu_Titles title,int index,struct IRData_s *received_data_ptr);
  int Check_Index(enum Menu_Titles title,int index);

private:
  void Init_Menu_Arrays(/*FKT_Callback m_callback*/); //Initialisert die Menü-arraysd mit werten

  //Menü-Arrays (beinhalten Menüeinträge)
  Menu_Entry_s Start_Menu[3];
  Menu_Entry_s Send_Menu[10];
  Menu_Entry_s Receive_Menu[2];
  Menu_Entry_s Greetings_Menu[2];

  //Indikator für maximalen Index (berechnet je Menü-Array)
  int max_menu_index;

  //Pointer auf alte Einträge
  struct Menu_Entry_s* old_entry_ptr=nullptr;

};

#endif //MENUE_CONTROL_LAYER_HPP
