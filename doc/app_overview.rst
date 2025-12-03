Application Overview:
=====================

Here is an overview of the RH Sensor application, which measures Relative Humidity (RH) and Temperature.
Results are sent to OpenHAB via Wifi & MQTT.

Sensor values are measured with the following sensors:

* RH values with an SHT30 humidity and temperature sensor
* Additional One-Wire (DS18B20) temperature sensor.



.. raw:: html
    :file: ./RhSensorInRust.drawio.html


Application targets are:

* Adafruit Feather nRF52840 Express board
* DFRobot Beetle ESP32-C3 board