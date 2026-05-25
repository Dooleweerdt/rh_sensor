Debugging with OpenOCD:
=======================

Debugging with OpenOCD and the ESP32C3 requires the ESP fork of OpenOCD:

`ESP OpenOCD Fork on GitHub: <https://github.com/espressif/openocd-esp32>`_

When building OpenOCD from source, the jimtcl component may give trouble.
A fix which will be deprecated was to use the ``--enable-internal-jimtcl`` flag on the ``./configure`` step.

Additional guidance for Zephyr and ESP32 MCU's can be found in these links:

`ESP OpenOCD guide with Tips and Quirks: <https://docs.espressif.com/projects/esp-idf/en/v5.0/esp32c3/api-guides/jtag-debugging/tips-and-quirks.html>`_

`Zephyr OpenOCD guide: <https://docs.zephyrproject.org/latest/boards/espressif/common/openocd-debugging.html>`_

The essential changes and notes are:

* The default ``west flash`` and ``west debugserver`` for flashing and debugging does not work with the ESP32C3.

    * ``OPENOCD`` and ``OPENOCD_DEFAULT_PATH`` symbols should be added to all west commands... (currently added to build...)
    * *Flashing currently fails due to a wrong interface file - this may be fixable in the future...*

* The ``esp appimage_offset 0xyyyyy`` parameter is essential to map flash instructions to the internal address space for the ESP32C3.

    * *This is required when using MCUBOOT and multiple images.*

* To get Zephyr Thread information, add the ``ESP RTOS Zephyr`` parameter to OpenOCD on startup.
* OpenOCD doesn't yet support SW breakpoints for the ESP32C3, so it has to be forced to use HW breakpoints (and only 2 are awailable!).
* The ESP32C3 needs a cache flush after connect, to align OpenOCD and internal states.

The VSCode OpenOCD launch configuration in use looks like this:

.. code-block:: json

    {
        "name": "Debug ESP32-C3",
        "type": "cppdbg",
        "request": "launch",
        //"program": "${input:elfPath}",
        "program": "${workspaceFolder}/build/app/zephyr/zephyr.elf",
        "args": [],
        "stopAtEntry": false,
        "hardwareBreakpoints": {
            "require": true
        },
        "cwd":  "${workspaceFolder}",
        "MIMode": "gdb",
        "miDebuggerPath": "/home/brian/zephyr-sdk-1.0.1/riscv64-zephyr-elf/bin/riscv64-zephyr-elf-gdb",
        //"miDebuggerServerAddress": "127.0.0.1:3333",
        "setupCommands": [
            {
                "description": "Connect to OpenOCD via extended-remote mode",
                "text": "-target-select extended-remote 127.0.0.1:3333",
                "ignoreFailures": false
            },
        ],
        "postRemoteConnectCommands": [
            { "text": "set remote hardware-watchpoint-limit 2 "},
            { "text": "monitor reset halt" },
            { "text": "maintenance flush register-cache"},
            { "text": "thb main" },
        ],
        "logging": {
            "engineLogging": false
        }
    }


Note the difference in setupCommmands, as this will use target extended-remode vs. the miDebugger call above which uses the old gdb remote command.

Flash script and start debugserver scripts can be found here and in the repository:

* Flash script: ``app/scripts/flashtarget.sh``

.. code-block:: bash

    # Use this for flashing both MCUBoot and the application (with verification):
    openocd -f board/esp32c3-builtin.cfg \
        -c "gdb memory_map disable" \
        -c "init" \
        -c "halt" \
        -c "esp appimage_offset 0x20000" \
        -c "program_esp build/mcuboot/zephyr/zephyr.bin 0x00000000 verify" \
        -c "program_esp build/app/zephyr/zephyr.signed.bin 0x00020000 verify" \
        -c "reset run" \
        -c "shutdown"


* Start debugserver script: ``app/scripts/start_debugserver.sh``

.. code-block:: bash

    openocd \
        -c "set ESP_RTOS Zephyr" \
        -f board/esp32c3-builtin.cfg \
        -c "gdb memory_map disable" \
        -c 'tcl port 6333' \
        -c 'telnet port 4444' \
        -c 'gdb port 3333' \
        -c "init" \
        -c "halt" \
        -c "esp appimage_offset 0x20000"

