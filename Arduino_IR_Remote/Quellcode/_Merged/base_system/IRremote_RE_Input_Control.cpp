#include <Arduino.h>
#include "IRremote_RE_Input_Control.hpp"
#include "base_definitions.h"

RE_Input_Control::RE_Input_Control(int clk_pin, int dt_pin, int sw_pin):Rotary_Encoder_PC1(dt_pin,clk_pin){
  Pin_SW=sw_pin;
}

/**
 * # Init
 * @brief Initialisiert den Knopf des Drehschalters
*/
void RE_Input_Control::Init(){
  pinMode(Pin_SW,INPUT);
  digitalWrite(Pin_SW,HIGH); //Pullup für Knopf, damit nicht ständig triggert
}

/**
 * # Checkup
 * @brief Kontroliert den Status des Drehschalters
 * @note Drehen: mit einem Step inkementiert der RotaryEncoder (drehschalter) um 4;
 * @note ruft die Callback-Funktion mit entsprechendem Display-Kommando (enum Display_Commands) auf
 * @param callback Funktion von typ void (int)
 * @return 1-up; 2-down; 3-ok 0; nichts ist passiert
*/
int RE_Input_Control::Checkup(Controls_Callback callback){
  if(callback == nullptr){
    return 0;
  }
  if(Rotary_Encoder_PC1.read()==4){
    Rotary_Encoder_PC1.readAndReset();
    callback(DISPLAY_UP_e);   //Übergebene Funktion mit Display-Kommando aufrufen
    return 1;
  }
  else if(Rotary_Encoder_PC1.read()==-4){
    Rotary_Encoder_PC1.readAndReset();
    callback(DISPLAY_DOWN_e);  //Übergebene Funktion mit Display-Kommando aufrufen
    return 2;
  }
  
  if(!digitalRead(Pin_SW)){
    delay(200);  //kurze pause, da knöpfe sehr ungenau sind -> würde sonst mehrmals auslösen
    callback(DISPLAY_OK_e);  //Übergebene Funktion mit Display-Kommando aufrufen
    return 3;
  }

  return 0;
}
