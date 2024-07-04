#ifndef IRREMOTE_SENSOR_ACTOR_SYSTEM_HPP
#define IRREMOTE_SENSOR_ACTOR_SYSTEM_HPP

#include "base_definitions.h"

/**
 * # Sensor_Actor_System Klasse
 * @brief Implementiert die IRremote Bibliothek für den Infrarot Sensor, sowie Aktor;
 * @brief enthält Funktionen zum Senden und Empfangen von IR-Signalen
*/
class Sensor_Actor_System {
public:
  //Konstruktor
  Sensor_Actor_System(int recv_pin, int recv_feedback_pin, int send_pin, int send_feedback_pin);  

  void Init();

  bool Recive(IRData_s*& received_data);
  int Send(protokoll_type_t protokoll,int adr, int cmd, int repeats);
  
private:
  int Pin_Recv;
  int Pin_Recv_Feedback;
  int Pin_Send;
  int Pin_Send_Feedback;
};

#endif  //IRREMOTE_SENSOR_ACTOR_SYSTEM_HPP
