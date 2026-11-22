# FreeTime Wearable for PineTime

This is a Rust-based firmware project for the [PineTime watch](https://pine64.org/devices/pinetime/) based on [Embassy](https://embassy.dev/).

# This is not "vibe-coded"

This project is not vibe-coded and does not except vibe-coded contributions. You can use agents all you like for suggestions,
intellisense, unit tests, and basic refactors, but avoid having it write source code. Full details are in the [AGENTS.md](./AGENTS.md)
file.

# Device

Connect to the device using the [official documentation](https://pine64.org/documentation/PineTime/Further_information/Devkit_wiring/).
Here is the SWD pinout for reference:

From left to right, where "left" is the side where the battery is connected:
1. SWDIO
2. SWDCLK
3. 3.3V VVC
4. GND

If using the battery, don't connect the 3.3V line. 