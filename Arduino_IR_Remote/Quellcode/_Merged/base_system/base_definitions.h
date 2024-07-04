
#ifndef BASE_DEFINITIONS_H
#define BASE_DEFINITIONS_H

#include <Arduino.h>

/**
 * # Pindefinitionen
 * @note Pindefinitionen werden vor dem Compilieren für das entsprechende Board freigeschaltet
*/
#if defined(__AVR_ATmega328P__)  // Arduino Uno
  #define Pin_RE_CLK  4   // Drehschalter (RE) Clock Pin 
  #define Pin_RE_DT   3   // Drehschalter (RE) Daten Pin
  #define Pin_RE_SW   2   // Drehschalter (RE) Knopf Pin
  #define Pin_LCD_RS  5   // Display (LCD) 
  #define Pin_LCD_E   6   // Display (LCD)
  #define Pin_LCD_D4  7   // Display (LCD)
  #define Pin_LCD_D5  8   // Display (LCD)
  #define Pin_LCD_D6  9   // Display (LCD)
  #define Pin_LCD_D7  10  // Display (LCD)
  #define Pin_IR_RECV 11  // IR Sensor (RECV)
  #define Pin_IR_RECV_FEEDBACK A0 // Signal-LED für das Empfangen von Daten
  #define Pin_IR_LED  12  // IR Sender (LED)
  #define Pin_IR_LED_FEEDBACK A1 // Signal-LED für das Senden von Daten
  #pragma message("Pins for Arduino UNO enabled") //Info in Kompielerausgabe, welches Board erkannt wurde
#elif defined(__AVR_ATmega32U4__)  // Arduino Leonardo
  //#error "Leonardo-Board: Pins noch nicht definiert"
  #define Pin_RE_CLK  0   // Drehschalter (RE) Clock Pin 
  #define Pin_RE_DT   0   // Drehschalter (RE) Daten Pin
  #define Pin_RE_SW   0   // Drehschalter (RE) Knopf Pin
  #define Pin_LCD_RS  0   // Display (LCD) 
  #define Pin_LCD_E   0   // Display (LCD)
  #define Pin_LCD_D4  0   // Display (LCD)
  #define Pin_LCD_D5  0   // Display (LCD)
  #define Pin_LCD_D6  0   // Display (LCD)
  #define Pin_LCD_D7  0  // Display (LCD)
  #define Pin_IR_RECV 3  // IR Sensor (RECV)
  #define Pin_IR_RECV_FEEDBACK 0 // Signal-LED für das Empfangen von Daten
  #define Pin_IR_LED  0  // IR Sender (LED)
  #define Pin_IR_LED_FEEDBACK 0 // Signal-LED für das Senden von Daten
  #pragma message("Pins for Arduino Leonardo enabled") //Info in Kompielerausgabe, welches Board erkannt wurde
#else
  #error "Pins für asugewühltes Board noch nicht definiert"
#endif

/**
 * # Display_Commands
 * @note zum Idendifizieren der Eingabe des Drehschalters
*/
enum Display_Commands{
  DISPLAY_UP_e =0,
  DISPLAY_DOWN_e=1,
  DISPLAY_OK_e=2
};

/**
 * # Menu_Titles
 * @note Titel/Überschriften zum Idendifizieren der einzelnen Menüs
*/
enum Menu_Titles{
  FUNKTION_e=0,
  MENU_TITLE_GREETINGS_e,
  MENU_TITLE_START_e,
  MENU_TITLE_SEND_e,
  MENU_TITLE_RECEIVE_e
};

/**
 * # protokoll_type_t
 * @note Infrarot Protokolle (aus der IR-Bibliothek)
*/
enum protokoll_type_t {
    UNKNOWN_e = 0,
    NEC_e,
} ;

/**
 * # IRData_s
 * @note Datenstrucktur, Wichtigste Daten zu Senden/Empfangen
*/
struct IRData_s {
    protokoll_type_t protocol;  // UNKNOWN, NEC, SONY, RC5, PULSE_DISTANCE, ...
    uint16_t command;           // Kommando
};

/**
 * # Menu_Entry_s
 * @note Datestrucktur für einen Menüeintrag
*/
struct Menu_Entry_s{
  String name;
  IRData_s data;
  enum Menu_Titles followed_by;
};

#endif //BASE_DEFINITIONS_H
