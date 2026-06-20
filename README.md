# Pico Controller

Embedded Rust firmware for a Raspberry Pi Pico 2W (RP2350) acting as a USB HID gamepad. Four buttons on the breadboard map to HID button inputs, visible in Windows via `joy.cpl`.

## Hardware

- Raspberry Pi Pico 2W (RP2350, ARM Cortex-M33)
- 4 tactile buttons on GP2, GP3, GP4, GP5 (internal pull-up — pressed = LOW)
- External LED + resistor on GP28

## Build & Flash

Hold BOOTSEL, plug in the Pico, then release BOOTSEL so it enumerates as a USB storage device.

```
cargo run --release
```

## Build Progress

### Components
![All components laid out before wiring — Pico 2W, breadboard, buttons, LEDs, and jumper wires](images/20260614_020057.jpg)

### First LED Wired
![First wiring milestone — Pico on breadboard with LED and resistor connected and lit](images/20260614_182106.jpg)

### All 4 Buttons Wired
![Final breadboard wiring — all four buttons connected to the Pico with jumper wires](images/20260615_011611.jpg)

## Testing

Press **Win + R**, type `joy.cpl`, open the gamepad properties, and press each button — the corresponding indicator should light up in the dialog.