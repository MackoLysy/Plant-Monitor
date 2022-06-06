#include <Arduino.h>
#include "plant.h"

using namespace PlantMonitor;

Plant::Plant(/* args */)
{
}

Plant::~Plant()
{

}
void Plant::setup(int waterPin, int lightPin)
{
    mWaterPin = waterPin;
    mLightPin = lightPin;
}

void Plant::readValues()
{
    mWaterValue = analogRead(mWaterPin);
    mLightValue = analogRead(mLightPin);
}