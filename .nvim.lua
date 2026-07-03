local dap = require('dap')

dap.configurations.c = {
  --{
    --name = "Zephyr: Native Posix / Emulation (Host)",
    --type = "codelldb",
    --request = "launch",
    ---- Points straight to the binary West built inside your build directory
    --program = function()
    --  return vim.fn.getcwd() .. '/build/zephyr/zephyr.exe'
    --end,
    --cwd = '${workspaceFolder}',
    --stopOnEntry = false,
  --},
  {
    name = "Zephyr: ESP32-C3 (Native GDB over OpenOCD using cppdbg)",
    type = "cppdbg",
    request = "launch",
    MIMode = 'gdb',
    miDebuggerPath = "/home/brian/zephyr-sdk-1.0.1/riscv64-zephyr-elf/bin/riscv64-zephyr-elf-gdb",
    cwd = '${workspaceFolder}',
    stopAtEntry = false,
    targetArchitecture = "arm",
    hardwareBreakpoints = {
      require = true,
      limit = 2,
    },

    sourceFileMap = {
      -- Maps standard relative build paths to your workspace root
      ["../"] = "${workspaceFolder}",
      ["../../modules/"] = "${workspaceFolder}/modules",
      ["../app/src/"] = "$(workspaceFolder)/src",
      ["../app/"] = "$(workspaceFolder)"
    },

    program = function()
      return vim.fn.getcwd() .. '/../build/app/zephyr/zephyr.elf'
    end,

    setupCommands = {
      {
        text = "-target-select extended-remote localhost:3333", -- Use 2331 for nRF J-Link!
        description = "Connect to GDB server via extended-remote link",
        ignoreFailures = false
      },
      {
        text = "set breakpoint auto-hw on",
        description = "Force automatic hardware breakpoints for flash space",
        ignoreFailures = false
      }
    },

    postRemoteConnectCommands = {
      { text = "set remote hardware-watchpoint-limit 2 " },
      { text = "monitor reset halt" },
      { text = "maintenance flush register-cache" },
      { text = "thb main" },
    },
    logging = {
      engineLogging = false
    },
  },
  {
    name = "Zephyr: Attach to Remote GDB Server (Hardware)",
    type = "codelldb",
    request = "launch",
    -- Path to your project's compiled ELF symbol binary file
    program = function()
      return vim.fn.getcwd() .. '/build/app/zephyr/zephyr.elf'
    end,
    -- Connects straight to an active background openocd/jlink instance running on port 2331
    cwd = '${workspaceFolder}',
    stopOnEntry = true,
    targetArchitecture = "arm",
    MIMode = "gdb",
    miDebuggerPath = "/home/brian/zephyr-sdk-1.0.1/arm-zephyr-eabi/bin/arm-zephyr-eabi-gdb",
    miDebuggerServerAddress = "localhost:3333",
    -- Essential reset commands passed down to the OpenOCD target
    postRemoteConnectCommands = {
      { text = "monitor reset halt", ignoreFailures = false },
      { text = "load", ignoreFailures = false },
    },
  }
}
-- Copy the C configuration directly over for C++ support
dap.configurations.rust = dap.configurations.c
dap.configurations.cpp = dap.configurations.c
