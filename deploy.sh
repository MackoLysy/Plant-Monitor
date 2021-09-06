#!/bin/bash


cd plantServer 
rm -rf build-arm
mkdir build-arm
cd build-arm
cmake .. -DBUILD_ARM=ON && make && scp PlantServer pi@192.168.0.150:/home/pi/plantMonitor/