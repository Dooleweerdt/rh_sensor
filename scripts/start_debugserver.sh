# Notes:
# After connecting in VS Code, remember to run:
# - "-exec monitor reset halt" to halt the CPU.
# - "-exec load" to load the program symbols (again?).
# - "-exec si" to single step once...

#openocd -c "set ESP_RTOS none" -f board/esp32c3-builtin.cfg -c "gdb memory_map disable" -c 'tcl port 6333' -c 'telnet port 4444' -c 'gdb port 3333'

openocd \
  -c "set ESP_RTOS Zephyr" \
  -f board/esp32c3-builtin.cfg \
  -c "gdb memory_map disable" \
  -c 'tcl port 6333' \
  -c 'telnet port 4444' \
  -c 'gdb port 3333' \
  -c "init" \
  -c "halt" \
  -c "esp appimage_offset 0x20000" \
#  -c "esp32c3 configure -event gdb-attach { halt; reset halt; }" \
#  -c "esp32c3 configure -event gdb-detach { reset run; }"
