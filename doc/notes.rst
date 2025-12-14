Zephyr and Rust notes:
=======================

`Zephyr Rust Language Support guide <https://docs.zephyrproject.org/latest/develop/languages/rust/index.html>`_ does not include the following step:

Update west.yml to include module for zephyr-lang-rust:

.. code-block:: yaml

    - name: zephyr-lang-rust
      revision: main
      path: modules/lang/rust
      remote: zephyrproject-rtos


Additionally the clang compiler is required - if you see this error during build:

.. code-block:: text

    ~/Projects/zephyr_workspace/zephyr/include/zephyr/kernel_includes.h:20:10: fatal error: 'stddef.h' file not found


To fix it run:

.. code-block:: sh

    sudo apt install clang


Required modifications to access device and sensor APIs from Rust:
------------------------------------------------------------------------

Make sure to enable the use of zephyr-sys in Cargo.toml the application (rh_sensor):
.. code-block:: toml

    [dependencies]
    zephyr-sys = { version = "0.1.0", path = "../../modules/lang/rust/zephyr-sys" }


Make sure to add sensor includes to zephyr-sys wrapper.h and build.rs:

In wrapper.h add:
.. code-block:: c

    // Added by BDR to include sensor API
    #include <zephyr/drivers/sensor.h>


In build.rs add:
.. code-block:: rust

    .allowlist_function("sensor_.*")
    .allowlist_item("SENSOR_.*")

