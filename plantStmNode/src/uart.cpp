#include "uart.h"
#include "config.h"
#include "handlers/dataHandler.h"

uint8_t recivedData = 0;

void uart_init(UART_HandleTypeDef *huart)
{
    huart->Instance = USART1;
    huart->Init.BaudRate = 9600;
    huart->Init.WordLength = UART_WORDLENGTH_8B;
    huart->Init.StopBits = UART_STOPBITS_1;
    huart->Init.Parity = UART_PARITY_NONE;
    huart->Init.Mode = UART_MODE_TX_RX;
    huart->Init.HwFlowCtl = UART_HWCONTROL_NONE;
    huart->Init.OverSampling = UART_OVERSAMPLING_16;
    if (HAL_UART_Init(huart) != HAL_OK)
    {
        Error_Handler();
    }
}

void HAL_UART_RxCpltCallback(UART_HandleTypeDef *huart)
{
    auto instance = DataHandler::getInstance();
    HAL_UART_Receive_IT(huart, &recivedData, 1);
    instance->addToBuffer(recivedData);
    if (recivedData == '\n')
    {
        instance->setMessageReady(true);
    }
    // buffer[rxCounter] = (char)recivedData;
}