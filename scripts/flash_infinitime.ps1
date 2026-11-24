$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$MCU_BOOTLOADER_PATH = $env:MCU_BOOTLOADER_PATH ?? (Join-Path $PSScriptRoot "..\other_images\bootloader-1.0.2.bin")
$INFINITIME_IMAGE_PATH = $env:INFINITIME_IMAGE_PATH ?? (Join-Path $PSScriptRoot "..\other_images\pinetime-mcuboot-app-image-1.16.1.bin")

probe-rs erase --chip nrf52832_xxAA
probe-rs download --chip nRF52832_xxAA --binary-format Binary $MCU_BOOTLOADER_PATH
probe-rs download --chip nRF52832_xxAA --binary-format Binary --base-address 0x8000 $INFINITIME_IMAGE_PATH
