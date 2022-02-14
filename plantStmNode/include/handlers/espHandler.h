#pragma once
#include <string>
#include "config.h"

class EspHandler
{

public:
    EspHandler(UART_HandleTypeDef *huart);
    ~EspHandler();
    void reset();
    void init();
    void handleMessage(const std::string &msg);

private:
    enum class State
    {
        RESET,
        INIT,
        IDLE,
    };
    State mState;
    UART_HandleTypeDef *mhuart;
    void handleInit(const std::string &msg);
    void ok();
    void getIP();
    void setIPInfo();
    void sendCmd(const std::string &data);
};
