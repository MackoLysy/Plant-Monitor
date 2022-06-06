#include <Arduino.h>

#include "bleHandler.h"
#include "config.h"
using namespace PlantMonitor;
BleHandler *BleHandler::mInstance = NULL;
BleHandler::BleHandler(/* args */)
{
    mInstance = this;
    mblePeripherial = new BLEPeripheral();
    mLedServie = new BLEService("19b10000e8f2537e4f6cd104768a1214");
    mLedChar = new BLECharCharacteristic("19b10000e8f2537e4f6cd104768a1214", BLERead | BLEWrite);
}

BleHandler::~BleHandler()
{
    delete mInstance;
}

void BleHandler::poll()
{
    mblePeripherial->poll();
}

void BleHandler::setupPeripherial()
{
    mblePeripherial->setLocalName(BLE_NAME);
    mblePeripherial->setAdvertisedServiceUuid(mLedServie->uuid());
    mblePeripherial->addAttribute(*mLedServie);
    mblePeripherial->addAttribute(*mLedChar);
    mblePeripherial->setEventHandler(BLEConnected, BleHandler::blePeripheralConnectHandler);
    mblePeripherial->setEventHandler(BLEDisconnected, BleHandler::blePeripheralDisconnectHandler);
    mLedChar->setEventHandler(BLEWritten, BleHandler::switchCharacteristicWritten);
    mblePeripherial->begin();
    Serial.println(F("BLE LED Peripheral"));
}

void BleHandler::blePeripheralConnectHandler(BLECentral &central)
{
    Serial.println("Connected!");
}
void BleHandler::blePeripheralDisconnectHandler(BLECentral &central)
{
    Serial.println("Diconnected!");
}
void BleHandler::switchCharacteristicWritten(BLECentral &central, BLECharacteristic &characteristic)
{
    Serial.println("Write");
}