Debugging with OpenOCD notes:
=============================

Debugging with OpenOCD and the ESP32C3 requires the ESP fork of OpenOCD:

`ESP OpenOCD Fork on GitHub: <https://github.com/espressif/openocd-esp32>`_

Additional guidance for Zephyr and ESP32 MCU's can be found in these links:

`ESP OpenOCD guide with Tips and Quirks: <https://docs.espressif.com/projects/esp-idf/en/v5.0/esp32c3/api-guides/jtag-debugging/tips-and-quirks.html>`_

`Zephyr OpenOCD guide: <https://docs.zephyrproject.org/latest/boards/espressif/common/openocd-debugging.html>`_

The essential changes and notes are:

* The default ``west flash`` and ``west debugserver`` for flashing and debugging does not work with the ESP32C3.
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


Note the difference in setupCommmands, as this will use target extended-remode vs. the miDebugger call above which uses the old remote command.

Flash script and start debugserver scripts can be found here:

* Flash script: ``app/scripts/flashtarget.sh``
* Start debugserver script: ``app/scripts/start_debugserver.sh``
