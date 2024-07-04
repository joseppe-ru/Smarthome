/* Anschlussplan
 *  Sender mit Transistorschaltung (auf pro Micro - arduino leonardo):
 *   Transistor NPN 2222:
 *    - Emitter -> GND
 *    - Basis -> 1KOhm -> D3 (Arduino)
 *    - Kollektor -> 220Ohm -> -IR_LED+ -> 5V (VCC, Arduino)
 *    
 *   Feedback LED:
 *    D10 (arduino) -> +LED- -> 330Ohm (optional veränderbar) -> GND 
*/

#import <IRremote.hpp>

#define IR_SEND_PIN    3
#define INFR_FEEDBACK_PIN  10
IRsend irsend;

void setup() {
    pinMode(LED_BUILTIN, OUTPUT);
    irsend.begin(IR_SEND_PIN,true,INFR_FEEDBACK_PIN);
    
    Serial.begin(9600);

    // Just to know which program is running on my Arduino
    Serial.println(F("START " __FILE__ " from " __DATE__ "\r\nUsing library version " VERSION_IRREMOTE));
    Serial.print(F("Send IR signals at pin "));
    Serial.println(IR_SEND_PIN);

}

uint8_t sCommand = 0x34;
uint8_t sRepeats = 0;

void loop() {

    irsend.sendNEC(0x00, sCommand, sRepeats);

        /*
     * Increment send values
     */
    sCommand += 0x11;
    sRepeats++;
    // clip repeats at 4
    if (sRepeats > 4) {
        sRepeats = 4;
    }
    delay(1000);  // delay must be greater than 5 ms (RECORD_GAP_MICROS), otherwise the receiver sees it as one long signal

}
