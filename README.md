# pca9557

Minimal `#![no_std]` Rust driver for the PCA9557 8-bit I²C GPIO expander.

The PCA9557 provides:

- 8 GPIO pins over I²C
- input register
- output register
- polarity inversion register
- configuration register
- per-pin input/output direction control

The chip powers up with all pins configured as inputs. The datasheet defines four 8-bit registers: input port `0x00`, output port `0x01`, polarity inversion `0x02`, and configuration `0x03`. Configuration bit `1` means input, bit `0` means output. :contentReference[oaicite:0]{index=0}

## Status

Experimental / work in progress.

## Requirements

```
[dependencies]
embedded-hal = "1"
```

## Supported environment

This crate is intended for embedded `no_std` use.

```
#![no_std]
```

## Basic usage

```
use embedded_hal::digital::PinState;
use pca9557::{Mode, Pca9557, Pin};

let mut io = Pca9557::new(i2c, 0x18);

io.set_mode(Pin::IO3, Mode::OUTPUT)?;
io.write_pin(Pin::IO3, PinState::High)?;

io.set_mode(Pin::IO4, Mode::INPUT)?;
let state = io.read_pin(Pin::IO4)?;
```

## I²C address

The PCA9557 uses three address pins: `A0`, `A1`, and `A2`.

Valid 7-bit I²C addresses are:

| A2 | A1 | A0 | Address |
|----|----|----|---------|
| 0  | 0  | 0  | `0x18`  |
| 0  | 0  | 1  | `0x19`  |
| 0  | 1  | 0  | `0x1A`  |
| 0  | 1  | 1  | `0x1B`  |
| 1  | 0  | 0  | `0x1C`  |
| 1  | 0  | 1  | `0x1D`  |
| 1  | 1  | 0  | `0x1E`  |
| 1  | 1  | 1  | `0x1F`  |

## Pins

Pins are represented as bit masks:

```
Pin::IO0 // 0b0000_0001
Pin::IO1 // 0b0000_0010
Pin::IO2 // 0b0000_0100
Pin::IO3 // 0b0000_1000
Pin::IO4 // 0b0001_0000
Pin::IO5 // 0b0010_0000
Pin::IO6 // 0b0100_0000
Pin::IO7 // 0b1000_0000
```

## Pin direction

Pin direction is controlled by the configuration register.

```
io.set_mode(Pin::IO0, Mode::INPUT)?;
io.set_mode(Pin::IO1, Mode::OUTPUT)?;
```

Meaning of configuration bits:

| Bit value | Mode   |
|----------:|--------|
| `1`       | Input  |
| `0`       | Output |

`set_mode()` performs a read-modify-write operation, so changing one pin does not overwrite the mode of the other pins.

## Reading pins

```
let state = io.read_pin(Pin::IO0)?;
```

Returns:

```
PinState::High
PinState::Low
```

## Writing pins

```
io.write_pin(Pin::IO0, PinState::High)?;
io.write_pin(Pin::IO0, PinState::Low)?;
```

`write_pin()` performs a read-modify-write operation on the output register.

## Register access

```
let input = io.read_input()?;
let output = io.read_output()?;
let config = io.read_config()?;
let polarity = io.read_polarity()?;
```

Full-register writes:

```
io.write_output(0b0000_1111)?;
io.write_config(0b1111_0000)?;
io.write_polarity(0b0000_0000)?;
```

## Polarity inversion

```
io.write_polarity(0b0000_1111)?;
```

- `1` → invert input
- `0` → keep original

## Raw input

```
let raw = io.read_input_raw()?;
```

Implementation:

```
input ^ polarity
```

## Reset / initialization

```
io.reset()?;
```

Writes:

```
config   = 0xff
output   = 0xff
polarity = 0x00
```

This is not hardware RESET.

## Notes

P0 differs from other pins: it is open-drain, while P1–P7 are push-pull outputs. External pull-up may be required.

## License

MIT License for pca9557-rs

Copyright (c) 2026 [Ramael]
