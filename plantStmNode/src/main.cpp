#include "config.h"
#include "handlers/espHandler.h"
#include "uart.h"
#include "gpio.h"
#include <string>
#include "string.h"
#include "handlers/dataHandler.h"
UART_HandleTypeDef huart1;
void sendCmd(const std::string &data);
int main(void)
{
  HAL_Init();
  SystemClock_Config();
  gpio_init();
  uart_init(&huart1);
  EspHandler espHander(&huart1);
  espHander.reset();
  espHander.init();
  while (1)
  {
    if (DataHandler::getInstance()->getMessageReady())
    {
      espHander.handleMessage(DataHandler::getInstance()->getBuffer());
    }
  }
}
