#ifndef  IRREMOTE_RE_INPUT_CONTROL_HPP
#define  IRREMOTE_RE_INPUT_CONTROL_HPP

#include <Encoder.h> //Encoder-Bibliothek im Quellcode-Ordner


/**
 * # RE_Input_Control Klasse
 * @brief Diese Klasse implementiert Funktionen des RotaryEncoder (aus Arduino Bibliothek)
 * @brief Der RE dient als Eingabe um durch das Menü zu navigieren 
 * @note RE = Rotary Encoder (Bauteilbezeichnung)
*/
class RE_Input_Control{
public:
  //Callback für Displaysteuerung (Funktion wird nach Eingabe über Drehschalter aufgerufen)
  using Controls_Callback = void (*)(int);

  //Konstruktor
  RE_Input_Control(int clk_pin, int dt_pin, int sw_pin); 
  void Init();
  int Checkup(Controls_Callback callback);
  
private:
  //Pin für Knopf des Drehschalters
  int Pin_SW;  
  //Drehschalter-Klasse aus Bibliothek (Encoder.h)
  Encoder Rotary_Encoder_PC1;
};

#endif //IRREMOTE_RE_INPUT_CONTROL_HPP
