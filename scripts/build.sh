PROJECT_ROOT="$(pwd)"
source "$PROJECT_ROOT/third_party/zephyr/zephyr/zephyr-env.sh"
cd $PROJECT_ROOT/plantNordicNode
west build --build-dir build --board=nrf51_ble400