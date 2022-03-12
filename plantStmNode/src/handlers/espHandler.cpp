#include "handlers/espHandler.h"
#include "handlers/dataHandler.h"
#include <string.h>
extern uint8_t recivedData;

EspHandler::EspHandler(UART_HandleTypeDef *huart) : mhuart(huart)
{
    HAL_UART_Receive_IT(mhuart, &recivedData, 1);
    mState = State::INIT;
}

EspHandler::~EspHandler()
{
}

void EspHandler::reset()
{
    sendCmd("AT+RST");
    HAL_Delay(10000);
}

void EspHandler::ok()
{
    sendCmd("AT");
}

void EspHandler::getIP()
{
    sendCmd("AT+CIFSR");
}
void EspHandler::setIPInfo()
{
    sendCmd("AT+CIPDINFO=1");
}
void EspHandler::handleMessage(const std::string &msg)
{
}

void EspHandler::handleInit(const std::string &msg)
{
}

void EspHandler::init()
{
    DataHandler::getInstance()->clearBuffer();
    mState = State::INIT;
    ok();
}
void EspHandler::sendCmd(const std::string &data)
{
    std::string tmpCmd = data + "\r\n";
    char cmd[tmpCmd.size() + 1];
    strcpy(cmd, tmpCmd.c_str());
    HAL_UART_Transmit(mhuart, (uint8_t *)(cmd), strlen(cmd), 500);
}
