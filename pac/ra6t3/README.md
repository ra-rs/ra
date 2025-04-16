
# ra6t3

This is the Peripheral Access Crate (PAC) for the RA6T3 device series.

The crate is automatically generated from the device SVD file in the [packs](https://www.keil.arm.com/packs) using [sdv2rust](https://github.com/rust-embedded/svd2rust).

## Overview

The `ra6t3` crate provides low-level access to device registers, enabling developers to interact with the hardware safely and efficiently.

## Usage

Include this crate in your `Cargo.toml`:

```toml
[dependencies]
 ra6t3 = "0.3.0"
```

## License

This crate is licensed under either the MIT License or the Apache License, Version 2.0.

The contents of this crate are auto-generated and licensed under the following proprietary terms of Renesas Electronics Corporation:
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products. No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all applicable laws, including copyright laws.

This software is provided "AS IS" without warranties of any kind, and Renesas makes no warranties regarding this software, including but not limited to
warranties of merchantability, fitness for a particular purpose, and non-infringement.

Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability of this software. By using this software,
you agree to the additional terms and conditions found at:
[Renesas Disclaimer](http://www.renesas.com/disclaimer)
