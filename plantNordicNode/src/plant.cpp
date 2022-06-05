#include "plant.h"
#include <Arduino.h>

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