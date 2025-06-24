# Ariel OS BLE-blinky demo

Two independent tasks: one advertising as a BLE device, the other one blinking a LED.

Supported boards:

- `nrf52dk`
- `nrf52840dk`

## Building

Set up you system for Ariel OS: [Getting started guide](ariel-os.github.io/ariel-os/dev/docs/book/getting-started.html).

```sh
laze build -b <board>
```

Where `<board>` is the board you want to build for, either `nrf52dk` or `nrf52840dk`.

## Flashing

```sh
laze build -b <board> run
```
