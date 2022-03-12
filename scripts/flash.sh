cd $(pwd)/plantNordicNode/build/zephyr
hex_firmware=$(ls -dt *.hex | head -1)

echo "hex firmware is $hex_firmware"

openocd -f interface/stlink-v2.cfg -f target/nrf51.cfg -c "init ; halt; sleep 500 ;sleep 500 ; program $hex_firmware verify ; sleep 500; reset; sleep 500; exit" 
echo "flashing complete"
echo "hex firmware was $hex_firmware"
echo "softdevice firmware was $softdevice_firmware"
cd $(pwd)