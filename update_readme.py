import os
import sys
import argparse

# Base directory for all PAC packages
PAC_BASE_DIR = "pac"

def is_non_empty_dir(path):
    """Check if a directory exists and is not empty."""
    if not os.path.isdir(path):
        return False
    return len(os.listdir(path)) > 0

def generate_readme(package_name):
    """Generate README.md for a specific package."""
    # Full path to the package directory
    package_dir = os.path.join(PAC_BASE_DIR, package_name)

    # Skip if the package directory doesn't exist or is empty
    if not is_non_empty_dir(package_dir):
        print(f"Warning: {package_dir} doesn't exist or is empty. Skipping.")
        return False

    # Generate DEVICE_NAME
    device_name = package_name.upper()

    # Template for README.md
    readme_template = f"""
# {package_name}

This is the Peripheral Access Crate (PAC) for the {device_name} device series.

The crate is automatically generated from the device SVD file in the [packs](https://www.keil.arm.com/packs) using [sdv2rust](https://github.com/rust-embedded/svd2rust).

## Overview

The `{package_name}` crate provides low-level access to device registers, enabling developers to interact with the hardware safely and efficiently.

## Usage

Include this crate in your `Cargo.toml`:

```toml
[dependencies]
 {package_name} = "0.3.0"
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
"""

    # Path to the README.md file within the package directory
    readme_path = os.path.join(package_dir, "README.md")


    # Write the README.md file
    with open(readme_path, "w") as file:
        file.write(readme_template)

    print(f"README.md generated for {package_name}")
    return True

def generate_all_readmes():
    """Generate README.md files for all non-empty packages in the PAC directory."""
    if not os.path.isdir(PAC_BASE_DIR):
        print(f"Error: {PAC_BASE_DIR} directory not found.")
        return False

    success_count = 0
    for package_name in os.listdir(PAC_BASE_DIR):
        package_dir = os.path.join(PAC_BASE_DIR, package_name)
        if is_non_empty_dir(package_dir):
            if generate_readme(package_name):
                success_count += 1

    print(f"Generated README.md files for {success_count} packages.")
    return success_count > 0

def main():
    parser = argparse.ArgumentParser(description="Generate README.md files for PAC packages.")
    parser.add_argument("package", nargs="?", help="Specific package name to generate README.md for.")
    args = parser.parse_args()

    if args.package:
        # Generate README.md for the specific package
        if not generate_readme(args.package):
            return 1
    else:
        # Generate README.md files for all packages
        if not generate_all_readmes():
            return 1

    return 0

if __name__ == "__main__":
    sys.exit(main())
