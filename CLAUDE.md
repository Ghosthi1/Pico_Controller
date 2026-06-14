# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Teaching Style

This is a **learning project**. The user is here to understand embedded Rust, not just get it working. Follow these rules in every response:

- **Explain before showing**: describe what needs to happen and why before any code appears
- **Prefer questions over answers**: ask the user what they think should happen next, then confirm or redirect
- **Avoid writing code directly**: guide the user to write it themselves — suggest what to type, explain the concept, and let them try first
- **When code is unavoidable** (e.g. a build config file): provide it but explain every line
- **Highlight the "why"**: always connect what we're doing to the underlying hardware or Rust concept
- **Surface tradeoffs**: when there are multiple valid approaches, explain the options so the user can choose

## Project

Embedded Rust firmware for a Raspberry Pi Pico 2W (RP2350, ARM Cortex-M33) acting as a USB HID game controller.

## Target Platform

- **MCU**: RP2350 (dual Cortex-M33)
- **Board**: Raspberry Pi Pico 2W
- **Rust target**: `thumbv8m.main-none-eabihf`
- **Paradigm**: `#![no_std]` + `#![no_main]`

## Build & Flash

The project is configured and compiles successfully.

**Toolchain setup (already done):**
- Target installed: `thumbv8m.main-none-eabihf`
- `.cargo/config.toml` sets the default target and runner (`picotool load -x -t elf`)
- `memory.x` linker script present (2048K FLASH at 0x10000000, 520K RAM at 0x20000000)
- `picotool` is installed

**Build:**
```
cargo build --release
```

**Flash via `cargo run`** (hold BOOTSEL, plug in Pico, then release BOOTSEL — Pico enumerates as USB device):
```
cargo run --release
```
This invokes the runner: `picotool load -x -t elf <elf-path>` automatically.

**Flash manually via picotool** (same BOOTSEL mode, but run picotool directly):
```
picotool load -x -t elf target/thumbv8m.main-none-eabihf/release/Pico_controller
```

## Current State

- `src/main.rs` compiles: bare `#![no_std]` + `#![no_main]` skeleton with `rp235x-hal`, infinite loop, no peripherals used yet
- `hal::block::ImageDef::secure_exe()` image header present (required by RP2350 bootrom)
- Dependencies: `panic-halt`, `cortex-m`, `cortex-m-rt`, `rp235x-hal`, `usbd-hid`
- Next step: flash the skeleton and confirm USB enumeration / LED blink to prove the toolchain works end-to-end

## Recommended Crates

For a USB HID game controller on Pico 2W:

- **`embassy-rp`** + **`embassy-usb`** — async HAL with built-in USB HID support (preferred for new projects)
- **`rp235x-hal`** — bare-metal HAL if async is not needed
- **`usbd-hid`** — USB HID descriptor/report generation
- **`cortex-m`** + **`cortex-m-rt`** — core Cortex-M runtime (required by both HAL approaches)

## Physical Documentation (Photo Checkpoints)

Prompt the user to take a photo at these moments — each one captures state that is hard to reconstruct later:

1. **Before first wiring** — bare Pico 2W and all components laid out, so you have a clean reference of what everything looks like unpowered and unconnected.
2. **After each wiring milestone** — every time a new component (button, joystick, resistor bank, etc.) is fully connected and tested, photograph it before moving on. Mistakes compound; a photo lets you diff what changed.
3. **First successful flash** — the moment the firmware runs on real hardware for the first time. Capture the Pico plugged in and any indicator (LED, USB enumeration on the host) visible.
4. **Before any rewire or destructive change** — if you're pulling wires or moving components, photo first.
5. **Final assembly** — completed wiring before any enclosure goes on, both top and underside if applicable.

When reminding the user, say something like: "Good checkpoint — worth a photo before we move on."

## Architecture Intent

The firmware will expose the Pico 2W as a USB HID gamepad. Key concerns:

- **Input polling**: Read GPIO pins for buttons/axes on a fixed schedule (e.g., 1 ms interval)
- **USB HID reports**: Pack button/axis state into a HID report descriptor and send via USB
- **Wireless (optional)**: The 2W has CYW43439 Wi-Fi/BT; BLE HID is an alternative to USB
- **`no_std`**: No heap allocator by default; use fixed-size buffers and `heapless` if collections are needed