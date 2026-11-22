# AGENTS.md

You are an AI agent and this project has limits on what AI agents are allowed to do.

## Agent limits

- You are free to provide any suggestions and to write unit tests and helper scripts
- You are NOT allowed to write source code accepts for very small refactors (just a few lines) and tedious, repetive tasks
- You are not allowed to open or interact with pull requests

## Dev environment

This is a embedded Rust project using Embassy for the PineTime watch. Run code on the watch using `cargo run`.

### PineTime specs

- System on Chip: Nordic Semiconductor nRF52832_xxAA
- Flash: 512KB with additional 4MB SPI NOR
- RAM: 64KB
- Display:
  -  Size: 1.3 inches (33mm) diagonal
  -  Type: IPS capacitive touchscreen, RGB 65K colors
  -  Display Controller: ST7789
  -  Resolution: 240x240 pixels
- Bluetooth: 5.0 (including Bluetooth Low Energy)
- Sensors: Accelerometer, Heart rate sensor
- Feedback: Vibration motor
- Battery: 170-180mAh 3.8V LiPo