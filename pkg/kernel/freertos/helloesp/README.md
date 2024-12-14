# Hello ESP32

## prereq

```bash
# install esp-idf
./modules/esp/install.sh esp32c6

# load esp-idf
source ./modules/esp/export.sh
```

## build

```bash
cd pkg/kernel/freertos/helloesp
idf.py set-target esp32c6
idf.py build
```

## upload

```bash
# ./tools/flash <project_directory> <serial_port>
# replace /dev/cu.usbmodem1401 accordingly
./tools/flash pkg/kernel/freertos/helloesp /dev/cu.usbmodem1401
# Hello from Swift on ESP32-C6!
```
