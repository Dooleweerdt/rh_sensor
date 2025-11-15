# Notes:
# After connecting in VS Code, remember to run:
# - "-exec monitor reset halt" to halt the CPU.
# - "-exec load" to load the program symbols (again?).
# - "-exec si" to single step once...

/home/brian/Tools/openocd-esp32/src/openocd -c "set ESP_RTOS none" -c "set ESP_FLASH_SIZE 0" -s /home/brian/Projects/zephyr_workspace/zephyr/boards/espressif/esp32c3_devkitm/support -s /home/brian/Tools/openocd-esp32/tcl -f /home/brian/Tools/openocd-esp32/tcl/board/esp32c3-builtin.cfg -c "gdb memory_map disable" -c 'tcl port 6333' -c 'telnet port 4444' -c 'gdb port 3333'
