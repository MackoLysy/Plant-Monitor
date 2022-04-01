#pragma once
#include <SPI.h>
#include <BLEPeripheral.h>
class BleHandler
{

public:
    BleHandler(/* args */);
    ~BleHandler();
    void poll();
    void setupPeripherial();

private:
    static BleHandler *mInstance;
    BLEPeripheral *mblePeripherial;
    BLEService *mLedServie;
    BLECharCharacteristic *mLedChar;
    
    static void blePeripheralConnectHandler(BLECentral& central);
    static void blePeripheralDisconnectHandler(BLECentral& central);
    static void switchCharacteristicWritten(BLECentral& central, BLECharacteristic& characteristic);
};
