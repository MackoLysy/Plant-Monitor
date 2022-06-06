#pragma once
#include <plant.h>

namespace PlantMonitor
{
    class PlantHandler
    {
    public:
        PlantHandler(/* args */);
        ~PlantHandler();
        void read();
        void setup();

    private:
        void addPlant(int waterPin, int lightPin);
        size_t plantNumber;
        Plant mPlants[5];
    };
}
