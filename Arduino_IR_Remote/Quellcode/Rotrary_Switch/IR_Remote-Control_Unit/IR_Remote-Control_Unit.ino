/*
 * Aufbau:
 *  
*/

#import <Encoder.h> //Encoder-Bibliothek im Quellcode-Ordner

//Heraus finden, welcher Arduino verwendet wird
//-> gibt dann unterschiedliche Pinzuweisungen
#if defined(__AVR_ATmega328P__)  // Arduino Uno
  #define Pin_CLK_RSW 3   // Clock-Pin -> Generiert einen Interrupt
  #define Pin_DT_RSW 4   // Drehschalter
  #define Pin_SW_RSW 2   // Knopf (Drückschalter) -> interrupt
  #pragma message("Pins for Arduino UNO enabled")
#elif defined(__AVR_ATmega32U4__)  // Arduino Leonardo
  #define Pin_CLK_RSW 2  // Clock-Pin -> Generiert einen Interrupt
  #define Pin_DT_RSW 4    // Drehschalter
  #define Pin_SW_RSW 3    // Knopf (Drückschalter)
  #pragma message("Pins for Arduino Leonardo enabled")
#else
  #error "Unbekanntes Board"
#endif

Encoder RSW_Encoder(Pin_DT_RSW,Pin_CLK_RSW);

void setup() {
  Serial.begin(9600); 
  pinMode(Pin_SW_RSW,INPUT);
  digitalWrite(Pin_SW_RSW,HIGH); //Pullup, damit nicht ständig triggert  
}

//diese Funktion kontrolliert die Eingabeeinheit (Rotary-Encoder)
//mit callback oder Rückgabewert ausstatten
void RSW_Checkup(){
  
  //Zum rauf und runter des Encoder:
  //  - mit einem Step inkementiert der Encoder (drehschalter) um 4 
  //  - dadurch wird mit einem Schritt 4mal inkrementiert
  //  - dadurch würde es 4mal auslösen, wenn man nicht schnell ganug (schneller als der Controller) ist. 
  //  - deshalb wird hier immer mit 4/-4 Verglichen -> das entspricht genau einem Schritt
  //  - Anschließend wird der Zähler zurückgesetzt.
  if(RSW_Encoder.read()==4){
    Serial.println("rotation left");//links=rauf
    RSW_Encoder.readAndReset();
    //nach_oben-Funktion in der Steuerung/Display-Anwendung aufrufen
  }
  else if(RSW_Encoder.read()==-4){
    Serial.println("rotation right");//rechts=runter
    RSW_Encoder.readAndReset();
    //nach_unten-Funktion in der Steuerung/Display-Anwendung aufrufen
  }
  
  if(!digitalRead(Pin_SW_RSW)){
    Serial.println("Button Pressed");
    //kurze pause, da knöpfe sehr ungenau sind 
    //-> würde sonst mehrmals auslösen
    delay(200);
  }
}


void loop() {
  //Funktion zum überprüfen des Drehschalters (RSW) -> eingabe
  RSW_Checkup();
}
