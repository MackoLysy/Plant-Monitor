#include "handlers/dataHandler.h"
#include <string.h>

DataHandler *DataHandler::mInstance = nullptr;

DataHandler::DataHandler(/* args */) : mMessageReady(false), mIndex(0)
{
}

DataHandler::~DataHandler()
{
}

void DataHandler::setMessageReady(bool value)
{
    mMessageReady = value;
}

DataHandler *DataHandler::getInstance()
{
    if (mInstance == nullptr)
    {
        mInstance = new DataHandler();
    }
    return mInstance;
}

void DataHandler::addToBuffer(uint8_t value)
{
    if (mIndex == MAX_SIZE)
    {
        mIndex = 0;
    }
    mMessageBuffer[mIndex] = value;
    mIndex++;
}

void DataHandler::clearBuffer()
{
    mIndex = 0;
    memset(mMessageBuffer, 0, MAX_SIZE);
}