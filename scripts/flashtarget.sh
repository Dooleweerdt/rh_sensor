# Partition map for ESP32-C3 with MCUBoot:
# - See also /home/brian/Projects/zephyr_workspace/zephyr/dts/vendor/espressif/partitions_0x0_default_4M.dtsi
# MCUBoot at 0x00000 - 0x0FFFF (64KB)
# Reserved (Sys partition) at 0x10000 - 0x1FFFF (64KB) - eFuses etc...
# Application slot 0 at 0x20000 - 0x1DFFFF (1792KB)
# Application slot 1 at 0x1E0000 - 0x39FFFF (1792KB)

/home/brian/Tools/openocd-esp32/src/openocd -s /home/brian/Projects/zephyr_workspace/zephyr/boards/espressif/esp32c3_devkitm/support -s /home/brian/Tools/openocd-esp32/tcl -f /home/brian/Tools/openocd-esp32/tcl/board/esp32c3-builtin.cfg -c "gdb memory_map disable" -c "program_esp build/mcuboot/zephyr/zephyr.bin 0x00000000 verify" -c "program_esp build/app/zephyr/zephyr.signed.bin 0x00020000 verify reset exit"

# Use this for single file uploads (no MCUBoot):
#/home/brian/Tools/openocd-esp32/src/openocd -s /home/brian/Projects/zephyr_workspace/zephyr/boards/espressif/esp32c3_devkitm/support -s /home/brian/Tools/openocd-esp32/tcl -f /home/brian/Tools/openocd-esp32/tcl/board/esp32c3-builtin.cfg -c "gdb memory_map disable" -c "program_esp build/zephyr/zephyr.bin 0x00000000 verify reset exit"

# Use this for erasing pages in flash (e.g. 0x10000 - 0x1FFFF):
#/home/brian/Tools/openocd-esp32/src/openocd -s /home/brian/Projects/zephyr_workspace/zephyr/boards/espressif/esp32c3_devkitm/support -s /home/brian/Tools/openocd-esp32/tcl -f /home/brian/Tools/openocd-esp32/tcl/board/esp32c3-builtin.cfg -c "gdb memory_map disable" -c "init; reset halt; flash erase_address 0x10000 0x10000; exit" 

# Use this for reading flash pages:
#/home/brian/Tools/openocd-esp32/src/openocd -s /home/brian/Projects/zephyr_workspace/zephyr/boards/espressif/esp32c3_devkitm/support -s /home/brian/Tools/openocd-esp32/tcl -f /home/brian/Tools/openocd-esp32/tcl/board/esp32c3-builtin.cfg -c "gdb memory_map disable" -c "init; reset halt; flash read_bank 0 flash_dump.bin 0x8000 0x1000; exit"