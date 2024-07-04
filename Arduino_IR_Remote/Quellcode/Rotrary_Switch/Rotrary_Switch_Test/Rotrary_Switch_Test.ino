/*
 * Aufbau:
 *  
*/

#include <Arduino.h>
#import <Encoder.h> //Encoder-Bibliothek im Quellcode-Ordner

//Heraus finden, welcher Arduino verwendet wird
//-> gibt dann unterschiedliche Pinzuweisungen
#if defined(__AVR_ATmega328P__)  // Arduino Uno
  #define Pin_CLK_RSW 4   // Clock-Pin -> Generiert einen Interrupt
  #define Pin_DT_RSW 3    // Drehschalter
  #define Pin_SW_RSW 2   // Knopf (Drückschalter)
  #pragma message("Pins for Arduino UNO enabled")
#elif defined(__AVR_ATmega32U4__)  // Arduino Leonardo
  #define Pin_CLK_RSW 2  // Clock-Pin -> Generiert einen Interrupt
  #define Pin_DT_RSW 3    // Drehschalter
  #define Pin_SW_RSW 4    // Knopf (Drückschalter)
  #pragma message("Pins for Arduino Leonardo enabled")
#else
  #error "Unbekanntes Board"
#endif

#define INPUT_PULLUP

Encoder RSW_Encoder(Pin_DT_RSW,Pin_CLK_RSW);

//RSW = Rotrary Switch
volatile bool RSW_Motion_Detected=false;
volatile int RSW_Turn_Direction; //0 -> links | -1 -> stillstand | 1 -> rechts

void setup() {
  Serial.begin(9600); 
  pinMode(Pin_SW_RSW,INPUT);
  digitalWrite(Pin_SW_RSW,HIGH);//Pullup, damit nicht ständig triggert
}


//diese Schleife kontroliert regelmäßig die Eingabeeinheit (Rotary-Encoder)
//Diese Funktion könnte auch duch einen Interrupt ausgelößt werden, um dem Controller die dauerhafte Belastung zu ersparen
void loop() {
  
  //Zum rauf und runter des Encoder:
  //  - mit einem Step inkementiert der Encoder (drehschalter) um 4 
  //  - dadurch wird mit einem Schritt 4mal inkrementiert
  //  - dadurch würde es 4mal auslösen, wenn man nicht schnell ganug (schneller als der Controller) ist. 
  //  - deshalb wird hier immer mit 4/-4 Verglichen -> das entspricht genau einem Schritt
  //  - Anschließend wird der Zähler zurückgesetzt.
  if(RSW_Encoder.read()==4){
    Serial.println("rotation left");//links=rauf
    //nach_oben-Funktion in der Steuerung/Display-Anwendung aufrufen
    RSW_Encoder.readAndReset();
  }
  else if(RSW_Encoder.read()==-4){
    Serial.println("rotation right");//rechts=runter
    //nach_unten-Funktion in der Steuerung/Display-Anwendung aufrufen
    RSW_Encoder.readAndReset();
  }
  
  if(!digitalRead(Pin_SW_RSW)){
    Serial.println("Button_Pressed");
    //
    delay(200);
  }

}
