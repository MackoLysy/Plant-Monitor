#pragma once
#include <plant.h>

class PlantHandler
{
public:
    PlantHandler(/* args */);
    ~PlantHandler();
    void read();
    void setup();
private:
    void addPlant(int waterPin, int lightPin);
    int plantNumber;
    Plant mPlants[5];
};
