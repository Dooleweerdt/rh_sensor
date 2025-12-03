Zephyr and Rust notes:
=======================

`Zephyr Rust Language Support guide <https://docs.zephyrproject.org/latest/develop/languages/rust/index.html>`_ does not include the following step:
Update west.yml to include module for zephyr-lang-rust:

.. code-block:: yaml

    - name: zephyr-lang-rust
      revision: main
      path: modules/lang/rust
      remote: zephyrproject-rtos


Additionally the clang compiler is required - if you see this error will occur during build:

.. WARNING::
    ~/Projects/zephyr_workspace/zephyr/include/zephyr/kernel_includes.h:20:10: fatal error: 'stddef.h' file not found


To fix it run:
.. code-block:: sh

    sudo apt install clang

