#include <Arduino.h>
#include "base_definitions.h"
#include "IRremote_Display_Control_System.hpp"

Display_Control_System::Display_Control_System(int rs_pin, int e_pin, int d4_pin, int d5_pin, int d6_pin, int d7_pin)
  :lcd(rs_pin, e_pin, d4_pin, d5_pin, d6_pin, d7_pin) {
}

void Display_Control_System::Init() {
  // set up the LCD's number of columns and rows:
  lcd.begin(16, 2);
}

void Display_Control_System::Update_Display_Text(String text_1, String text_2) {
  //clear Display
  lcd.clear();
  //Text anzeigen (text1 -> Zeile 1) (text2 -> Zeile 2)
  lcd.setCursor(0, 0);
  lcd.print(text_1);
  lcd.setCursor(0, 1);
  lcd.print(text_2);
  return;
}
