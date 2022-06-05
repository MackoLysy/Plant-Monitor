// Copyright (c) Sandeep Mistry. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.

#include <Arduino.h>
#include <TaskScheduler.h>
#include <bleHandler.h>
#include <plantHandler.h>
#include "plant_message.pb.h"

Scheduler ts;
PlantMonitor::BleHandler ble;
PlantMonitor::PlantHandler plantHandler;
void readCallback();
Task task(1000 * TASK_MILLISECOND, TASK_FOREVER, &readCallback, &ts, true);

void setup()
{
  Serial.begin(9600);
  plantHandler.setup();
  ble.setupPeripherial();
}

void loop()
{
  ts.execute();
  ble.poll();
}

void readCallback()
{
  plantHandler.read();
  // digitalWrite(1, HIGH);
  // delay(100);
  // Serial.println("Analog!");
  // int light = analogRead(4);
  // int water = analogRead(3);
  // Serial.println(light);
  // Serial.println(water);
  // digitalWrite(1, LOW);
}