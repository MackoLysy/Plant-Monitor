#!/bin/bash
wget https://apt.kitware.com/kitware-archive.sh
sudo bash kitware-archive.sh
sudo apt install --no-install-recommends git cmake ninja-build gperf \
  ccache dfu-util device-tree-compiler wget \
  python3-dev python3-pip python3-setuptools python3-tk python3-wheel xz-utils file \
  make gcc gcc-multilib g++-multilib libsdl2-dev

pip3 install --user -U west
echo 'export PATH=~/.local/bin:"$PATH"' >> ~/.bashrc
ZEPHYR_SDK=https://github.com/zephyrproject-rtos/sdk-ng/releases/download/v0.13.1/zephyr-sdk-0.13.1-linux-x86_64-setup.run
source ~/.bashrc
rm kitware-archive.sh

if [ -e "$HOME/zephyr-sdk/" ]; then
  echo "$HOME/zephyr-sdk/ already exists, not installing it again!" >&2
else
  echo "Installing zephyr-sdk"
  wget "$ZEPHYR_SDK"
  chmod +x "$(basename "$ZEPHYR_SDK")"
  ./"$(basename "$ZEPHYR_SDK")" -- -y -rc -d $HOME/zephyr-sdk/
fi

mkdir -p $(pwd)/third_party/zephyr
cd $(pwd)/third_party/zephyr

python3 -m west init
python3 -m west update
  
python3 -m pip install -r zephyr/scripts/requirements.txt

rm -f zephyr-sdk-0.13.1-linux-x86_64-setup.run
cd "$(pwd)"
chmod +x "$(pwd)/scripts/build.sh"
chmod +x "$(pwd)/scripts/flash.sh"

