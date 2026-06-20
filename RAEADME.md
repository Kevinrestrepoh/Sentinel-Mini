# Sentinel Mini

A small, battery-powered security gadget built around the ESP32-C3, written in bare-metal Rust.

Touch the sensor to arm it, walk away. If someone trips the sensor, the buzzer fires and the event gets written to flash. The log survives reboots. While armed and idle the chip drops into deep sleep, burning only a few microamps.

## Hardware

| Part | Notes |
|---|---|
| ESP32-C3 SuperMini | |
| SSD1306 OLED 128x64 | I2C — SDA GPIO20, SCL GPIO21 |
| TTP223 capacitive touch | Out GPIO1 |
| Passive buzzer | GPIO2 |

## How It Works

Short tap cycles through a three-item menu (Arm / History / Battery). Long press (800 ms) selects.

Once armed the display shows "ARMED" and the chip goes to sleep. Waking on a touch at GPIO1, it boots straight into the alarm sequence — three beeps and the event logged. Long press again to dismiss and return to the menu.

The state machine looks like this:

```
Boot (2 s) ──▶ Home ──▶ Armed ──▶ Alarm ──▶ Home
                  ▲                   │
                  └───────────────────┘
```

## Features

- Five-state machine: Boot, Home, Armed, Alarm, History
- Capacitive touch input — tap to navigate, hold to select
- Software-PWM buzzer at 2 kHz
- Up to 32 events persisted in NVS flash
- Deep sleep with GPIO1 wakeup (~5 µA idle)
- defmt logs over USB serial

## Dependencies

| Crate | Version | What for |
|---|---|---|
| `esp-hal` | 1.1 | HAL for ESP32-C3 |
| `ssd1306` | 0.10 | OLED driver |
| `embedded-graphics` | 0.8 | Text/drawing primitives |
| `embedded-hal` | 1.0 | Traits |
| `heapless` | 0.9 | Stack-allocated Vec and String |
| `esp-nvs` | 0.4 | NVS key-value storage |
| `esp-storage` | 0.8 | Flash backend for NVS |
| `esp-alloc` | 0.10 | Heap allocator for NVS internals |
| `defmt` | 1.0 | Structured logging |
| `esp-backtrace` | 0.19 | Panic handler |
| `esp-println` | 0.17 | Serial output |
| `esp-bootloader-esp-idf` | 0.5 | IDF bootloader compat |

## Building

```sh
rustup target add riscv32imc-unknown-none-elf
cargo install espflash
cargo build --release
```

Flash with:

```sh
espflash flash --monitor --chip esp32c3 target/riscv32imc-unknown-none-elf/release/sentinel-mini
```

Note: the serial monitor will report an error when the device enters deep sleep — that's normal, the UART shuts down with the rest of the chip.

## Usage

1. Power on, wait through the 2 s boot screen
2. Tap to move the cursor, hold to select
3. **Arm** — hold on Arm, device sleeps until touched
4. **Alarm** — buzzer sounds on touch, hold to dismiss
5. **History** — view stored event count
6. **Battery** — view voltage reading
