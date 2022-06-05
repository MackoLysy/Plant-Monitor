#pragma once

namespace PlantMonitor
{
    class Plant
    {

    public:
        Plant(/* args */);
        void setup(int waterPin, int lightPin);
        ~Plant();
        void readValues();
        int getWaterValue() const
        {
            return mWaterValue;
        }
        int getLightValue() const
        {
            return mLightValue;
        }

    private:
        int mWaterPin;
        int mLightPin;
        int mLightValue;
        int mWaterValue;
    };
}
