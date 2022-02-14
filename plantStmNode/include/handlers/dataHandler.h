#pragma once
#include "config.h"

class DataHandler
{

public:
    static DataHandler *getInstance();
    ~DataHandler();
    void setMessageReady(bool value);
    const bool &getMessageReady() const { return mMessageReady; }
    const char *getBuffer() const { return mMessageBuffer; }
    void clearBuffer();
    void addToBuffer(uint8_t value);

private:
    DataHandler();
    bool mMessageReady;
    char mMessageBuffer[MAX_SIZE];
    uint8_t mIndex;
    static DataHandler *mInstance;
};
