#include <iostream>
#include "wiringPi.h"

int main(int argc, char const *argv[])
{
    if (wiringPiSetup() < 0)
    {
        fprintf(stderr, "Unable to setup wiringPi: %s\n", strerror(errno));
        return 1;
    }
    std::cout << "Funga!" << std::endl;
    return 0;
}
