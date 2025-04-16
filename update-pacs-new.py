#!/usr/bin/env python3

import os
import sys
import shutil
import subprocess
import glob
import tempfile
import re
import traceback
import xml.etree.ElementTree as ET
from pathlib import Path
from collections import defaultdict, Counter
import argparse

# Global constants should be in uppercase
RUST_BACKTRACE = "1"
RUST_FULLTRACE = "full"
RUST_LOG = "info"

# SVD preprocessing options
FIX_ACCESS_TYPES_NESTED = True
FIX_ACCESS_TYPES = True
FIX_DERIVED_FROM = True
FIX_ENUMERATED_VALUES = True
FIX_ENUMERATED_VALUE_RANGES = True
FIX_WRITE_CONSTRAINTS = True
FIX_EMPTY_NAME_ENUMERATED_VALUES = True
FIX_MISSING_PLACEHOLDERS = True
FIX_RESET_VALUES = True
FIX_MISSING_ACCESS_TAGS = True
FIX_NAME_ENUMERATED = True  # Add new option to control the new fix

# Cargo version
CARGO_VERSION = "0.3.0"

# Define dependencies
DEPS = '''
[package.metadata.docs.rs]
features = ["all-peripherals", "critical-section", "rt"]

[dependencies]
critical-section = { version = "1.1", optional = true }
cortex-m = "0.7"
cortex-m-rt = { version = "0.7.5", optional = true }
vcell = "0.1.3"
portable-atomic = { version = "1.11.0"}

[features]
rt = ["cortex-m-rt/device"]
critical-section = ["dep:critical-section"]
'''

# Define manifest template
manifest_template = '''
[package]
name = "@package_name@"
version = "@version@"
edition = "2024"
description = "Peripheral Access Crate (PAC) for @DEVICE_NAME@."
authors = ["Nathan Larsen <n8tlarsen@gmail.com>", "Addison Heavner <addisonheavner@gmail.com>", "Tri Nguyen <trongtribk06@gmail.com>"]
keywords = ["@device_name@", "@family_name@", "arm", "cortex-m", "renesas"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/ra-rs/ra"
documentation = "https://docs.rs/crate/@device_name@-pac/latest"
categories = ["embedded", "hardware-support", "no-std"]
readme = "README.md"

'''

def format_cargo_toml(content):
    """
    Format the Cargo.toml content:
    - Add space between word and '='
    - Remove trailing spaces at the end of each line
    - Remove continuous blank lines
    - Ensure only one blank line between sections
    - Add a blank line before `tracing`, `tracing_dummy`, and `rt` sections
    - Add a space after commas in lists
    """
    # Add space between word and '='
    content = re.sub(r'(\w+)=', r'\1 =', content)

    # Add a space after commas in lists (e.g., features = ["all","rt"] -> features = ["all", "rt"])
    content = re.sub(r',(\S)', r', \1', content)

    # Remove trailing spaces at the end of each line
    content = re.sub(r'[ \t]+$', '', content, flags=re.MULTILINE)

    # Replace multiple blank lines with a single blank line
    content = re.sub(r'\n\s*\n+', '\n\n', content)

    # Add a blank line before tracing, tracing_dummy, and rt sections
    content = re.sub(r'(?<=\n)(tracing|tracing_dummy|rt)\s*=.*\n', r'\n\g<0>', content)

    return content

def generate_cargo_toml(pac_dir, package_name):
    """
    Create or update the Cargo.toml file with the manifest template and features.

    Args:
        pac_dir: Path to the PAC directory (Path object)
        package_name: Name of the package (e.g., "ra0e1")

    Returns:
        bool: True if successful, False otherwise
    """
    cargo_toml_path = pac_dir / "Cargo.toml"
    lib_rs_path = pac_dir / "lib.rs"
    src_dir = pac_dir / "src"
    features_toml_path = pac_dir / "features.toml"

    try:
        # If src directory doesn't exist, create it
        if not src_dir.exists():
            src_dir.mkdir(parents=True, exist_ok=True)
            print(f"Created directory: {src_dir}")

        # If lib.rs exists at the PAC root, move it to src/lib.rs
        if lib_rs_path.exists():
            # Make sure target directory exists
            src_dir.mkdir(parents=True, exist_ok=True)

            # Move lib.rs to src/lib.rs
            shutil.move(str(lib_rs_path), str(src_dir / "lib.rs"))
            print(f"Moved {lib_rs_path} to {src_dir / 'lib.rs'}")

        # Get device name and family name from package_name
        # Example: "ra0e1" -> device_name="ra0e1", family_name="ra"
        device_name = package_name.split('-')[0]
        family_name = device_name[:2] if len(device_name) >= 2 else device_name

        # For Renesas devices, device_name is usually the full device name in uppercase
        if package_name.startswith("ra"):
            DEVICE_NAME = "R7FA" + device_name[2:].upper()
        else:
            DEVICE_NAME = device_name.upper()

        # Replace all placeholders in the template
        updated_manifest = manifest_template.replace("@package_name@", package_name)
        updated_manifest = updated_manifest.replace("@version@", CARGO_VERSION)
        updated_manifest = updated_manifest.replace("@DEVICE_NAME@", DEVICE_NAME)
        updated_manifest = updated_manifest.replace("@device_name@", device_name)
        updated_manifest = updated_manifest.replace("@family_name@", family_name)

        # Add dependencies
        content = updated_manifest + DEPS

        # Check if features.toml exists and extract features
        additional_features = {}
        if features_toml_path.exists():
            with open(features_toml_path, 'r') as f:
                features_content = f.read()

                # Extract the features section content (excluding the [features] header)
                features_match = re.search(r'^\[features\]\s*(.*?)(?=^\[|\Z)',
                                          features_content,
                                          re.DOTALL | re.MULTILINE)

                if features_match:
                    features_text = features_match.group(1).strip()

                    # Extract individual features and their values
                    feature_lines = features_text.split('\n')
                    for line in feature_lines:
                        line = line.strip()
                        if line and '=' in line:
                            # Split by first equals sign
                            parts = line.split('=', 1)
                            if len(parts) == 2:
                                feature_name = parts[0].strip()
                                feature_value = parts[1].strip()
                                additional_features[feature_name] = feature_value

                    print(f"Extracted {len(additional_features)} features from {features_toml_path}")
                else:
                    print(f"Warning: Could not find [features] section in {features_toml_path}")

            # Try to delete the features.toml file with retry logic
            import time
            import gc

            MAX_RETRIES = 3
            retry_count = 0
            delete_success = False

            while retry_count < MAX_RETRIES and not delete_success:
                try:
                    gc.collect()  # Force garbage collection
                    time.sleep(0.5)  # Wait a bit for file handles to be released

                    if features_toml_path.exists():
                        features_toml_path.unlink()
                        delete_success = True
                        print(f"Deleted {features_toml_path} after extracting features")
                except (PermissionError, OSError) as e:
                    retry_count += 1
                    print(f"Attempt {retry_count}: Could not delete {features_toml_path}: {str(e)}")
                    time.sleep(1)  # Wait a bit longer before retrying

            if not delete_success:
                print(f"Warning: Could not delete {features_toml_path} after {MAX_RETRIES} attempts. You may need to delete it manually.")

        # Now we need to merge the additional features with the existing [features] section in content
        # Find the existing [features] section
        features_section_match = re.search(r'(^\[features\]\s*)(.*?)(?=^\[|\Z)',
                                          content,
                                          re.DOTALL | re.MULTILINE)

        if features_section_match:
            # Extract the existing features section and features
            features_header = features_section_match.group(1)
            existing_features_text = features_section_match.group(2).strip()

            # Build the new features section
            new_features_section = features_header + existing_features_text

            # Add additional features if they don't already exist
            if additional_features:
                new_features_section += "\n\n"  # Add space after existing features

                # Add each additional feature
                for feature_name, feature_value in additional_features.items():
                    new_features_section += f"{feature_name} = {feature_value}\n"

            # Replace the old features section with the new one
            content = re.sub(r'^\[features\].*?(?=^\[|\Z)',
                            new_features_section,
                            content,
                            flags=re.DOTALL | re.MULTILINE)

        # Format the content
        formatted_content = format_cargo_toml(content)

        # Write to Cargo.toml
        with open(cargo_toml_path, 'w') as f:
            f.write(formatted_content)

        print(f"Successfully created/updated Cargo.toml for {package_name}")
        return True

    except Exception as e:
        print(f"Error creating/updating Cargo.toml for {package_name}: {str(e)}")
        traceback.print_exc()  # Print the full traceback for debugging
        return False

def show_usage():
    """Display script usage information."""
    print("Usage: gen_pac.py [options] device_name")
    print("")
    print("Options:")
    print("  -h, --help         Show this help message and exit")
    print("  -s, --svd          Generate patched SVD files only")
    print("  -p, --pac          Generate PACs from patched SVD files only")
    print("  -a, --all          Generate full process (SVD and PAC) - default if no option specified")
    print("")
    print("Arguments:")
    print("  device_name        Mandatory. Process only the specified device.")
    print("                     Use 'ALL' to process all SVD files in the svd directory.")
    print("")
    print("Examples:")
    print("  gen_pac.py ALL               # Process all SVD files (full process)")
    print("  gen_pac.py R7FA4M1AB         # Process only R7FA4M1AB.svd (full process)")
    print("  gen_pac.py -s ALL            # Generate patched SVD files only for all devices")
    print("  gen_pac.py -p R7FA4M1AB      # Generate PAC only for R7FA4M1AB")
    print("")

def check_command_exists(command):
    """Check if a command exists in the system path."""
    if shutil.which(command) is None:
        print(f"{command} could not be found. Install it with the following command:")
        print("")
        if command == "svd2rust":
            print("    cargo install --git https://github.com/rust-embedded/svd2rust) --locked")
        elif command == "svd":
            print("    pip install svdtools")
        print("")
        sys.exit(1)

def run_command(command, check=True):
    """Run a shell command and check for errors."""
    try:
        # Let the command output directly to the console instead of capturing it.
        result = subprocess.run(command, check=check, shell=True, text=True)
        print("Command executed successfully.")
        return result
    except subprocess.CalledProcessError as e:
        print(f"Command failed: {e.cmd}")
        if check:
            sys.exit(1)
        return e

# Updated function to target the <access> tag within <register> and <field> elements
def fix_access_types_nested(root):
    """
    Fix access types in SVD XML tree.
    Converts all 'read-writeonce' to 'read-write' within <register> and <field> elements.
    """
    print("Fixing access types...")

    fixed_count = 0
    for register in root.findall(".//register"):
        for field in register.findall(".//field"):
            access_elem = field.find("access")
            if access_elem is not None and access_elem.text == "read-writeonce":
                access_elem.text = "read-write"
                print(f"  Replaced 'read-writeonce' with 'read-write' in field: {field.find('name').text}")
                fixed_count += 1
            if access_elem is not None and access_elem.text == "writeonce":
                access_elem.text = "read-write"
                print(f"  Replaced 'writeonce' with 'read-write' in field: {field.find('name').text}")
                fixed_count += 1

    print(f"Total fixed: {fixed_count}")
    return root

def add_missing_access_tags(root):
    """
    Adds missing 'access' tags with 'read-write' as the default for all fields in the SVD XML.
    """
    print("Checking for missing access tags...")

    # Iterate through all 'field' elements in the SVD XML
    for field in root.findall(".//field"):
        access_elem = field.find("access")

        # If the 'access' tag is missing, add it with 'read-write' as default
        if access_elem is None:
            print(f"Adding 'access' tag to field: {field.find('name').text}")
            access_tag = ET.SubElement(field, "access")
            access_tag.text = "read-write"

    print("Finished adding missing 'access' tags.")
    return root


def fix_access_types(root):
    """Fix access types in SVD file (read,write -> read-write)"""
    print("Fixing access types...")
    for access_elem in root.findall(".//*[@access]"):
        if access_elem.get("access") == "read,write":
            access_elem.set("access", "read-write")

def fix_derived_from(root):
    """Remove derived_from attributes from registers"""
    print("Removing derived_from attributes...")
    for derived in root.findall(".//register[@derivedFrom]"):
        derived.attrib.pop('derivedFrom')

def fix_enumerated_values(root):
    """Fix SVD file by adding explicit values to enumerated values with isDefault attribute."""
    print("Fixing enumerated values with isDefault attribute...")

    # Track how many changes we make
    changed_count = 0

    # Find all fields containing enumeratedValues
    for field in root.findall(".//field"):
        # Look for enumeratedValues within this field
        enum_values_list = field.findall("./enumeratedValues")

        for enum_values in enum_values_list:
            # Check if any enumeratedValue in this section has isDefault=true
            default_values = enum_values.findall("./enumeratedValue[isDefault='true']")

            if default_values:
                # Get lsb and msb from the field
                lsb_elem = field.find("lsb")
                msb_elem = field.find("msb")

                if lsb_elem is not None and msb_elem is not None:
                    lsb = int(lsb_elem.text)
                    msb = int(msb_elem.text)
                    bit_width = msb - lsb + 1

                    # Default value should be all zeros with the proper width
                    default_value = "#" + "0" * bit_width

                    # Check if other enum values use a different format
                    bin_format = False
                    dec_format = False
                    max_width = bit_width

                    for enum_value in enum_values.findall("./enumeratedValue/value"):
                        if enum_value.text:
                            if (enum_value.text.startswith('#') and len(enum_value.text) > 1):
                                bin_format = True
                                max_width = max(max_width, len(enum_value.text[1:]))
                            else:
                                try:
                                    int(enum_value.text)
                                    dec_format = True
                                except ValueError:
                                    pass

                    # Add explicit value to the default value(s)
                    for default_value_elem in default_values:
                        # If it already has a value, don't modify
                        if default_value_elem.find("value") is not None:
                            continue

                        # Create the value element with proper formatting
                        value_elem = ET.Element("value")

                        # Use the appropriate format for the value
                        if bin_format:
                            value_elem.text = f"#{'0' * max_width}"
                        elif dec_format:
                            value_elem.text = "0"
                        else:
                            value_elem.text = f"#{'0' * bit_width}"

                        # Simply append the value element to default_value_elem
                        # We'll handle formatting during XML writing
                        default_value_elem.append(value_elem)

                        name_elem = default_value_elem.find("name")
                        name_text = name_elem.text if name_elem is not None else "default"
                        print(f"  Added value {value_elem.text} to default enumerated value '{name_text}'")
                        changed_count += 1
                else:
                    # If we can't find lsb/msb, use a default value of "#0"
                    for default_value_elem in default_values:
                        if default_value_elem.find("value") is not None:
                            continue

                        value_elem = ET.Element("value")
                        value_elem.text = "#0"

                        # Simply append the value element
                        default_value_elem.append(value_elem)

                        name_elem = default_value_elem.find("name")
                        name_text = name_elem.text if name_elem is not None else "default"
                        print(f"  Added value {value_elem.text} to default enumerated value '{name_text}' (no lsb/msb info found)")
                        changed_count += 1

    # Handle any enumeratedValues that are not directly under a field (if any exist)
    for enum_values in root.findall(".//enumeratedValues"):
        # Skip those that we've already processed (those under fields)
        if enum_values.find("..") is not None and enum_values.find("..").tag == "field":
            continue

        # Check if any enumeratedValue in this section has isDefault=true
        default_values = enum_values.findall("./enumeratedValue[isDefault='true']")

        if default_values:
            # For these cases, we don't have field context, so use a default value of "#0"
            for default_value_elem in default_values:
                if default_value_elem.find("value") is not None:
                    continue

                value_elem = ET.Element("value")
                value_elem.text = "#0"

                # Simply append the value element
                default_value_elem.append(value_elem)

                name_elem = default_value_elem.find("name")
                name_text = name_elem.text if name_elem is not None else "default"
                print(f"  Added value {value_elem.text} to default enumerated value '{name_text}' (no field context)")
                changed_count += 1

    print(f"Fixed {changed_count} enumerated values with isDefault attribute")

def fix_enumerated_value_ranges(root):
    """Fix SVD file by ensuring enumerated values are within valid range based on field bit width."""
    print("Fixing enumerated values with out-of-range values...")

    # Track how many changes we make
    fixed_count = 0

    # First pass: Handle standard fields with enumeratedValues
    for field in root.findall(".//field"):
        # Get field name for logging
        field_name_elem = field.find("name")
        field_name = field_name_elem.text if field_name_elem is not None else "unnamed"

        # Get bit width from lsb and msb
        lsb_elem = field.find("lsb")
        msb_elem = field.find("msb")

        # If lsb and msb are available, calculate bit width
        if lsb_elem is not None and msb_elem is not None:
            lsb = int(lsb_elem.text)
            msb = int(msb_elem.text)
            bit_width = msb - lsb + 1
            max_value = (1 << bit_width) - 1  # Calculate maximum value: 2^bit_width - 1
        # Try alternative: check for bitWidth element
        else:
            bit_width_elem = field.find("bitWidth")
            if bit_width_elem is not None:
                bit_width = int(bit_width_elem.text)
                max_value = (1 << bit_width) - 1
            # Check for bitOffset element with bitWidth
            elif field.find("bitOffset") is not None and field.find("bitWidth") is not None:
                bit_width = int(field.find("bitWidth").text)
                max_value = (1 << bit_width) - 1
            # Fallback for fields with just bitField elements
            elif field.find("bitField") is not None:
                # Get all bitFields and determine max bit position
                bit_fields = field.findall("bitField")
                max_bit = 0
                for bit_field in bit_fields:
                    pos_elem = bit_field.find("position")
                    if pos_elem is not None and pos_elem.text:
                        max_bit = max(max_bit, int(pos_elem.text))
                bit_width = max_bit + 1
                max_value = (1 << bit_width) - 1
            else:
                # If we can't determine bit width, assume 1-bit (conservative)
                bit_width = 1
                max_value = 1
                print(f"  Warning: Couldn't determine bit width for field '{field_name}', assuming 1-bit")

        # Look for enumeratedValues within this field
        enum_values_list = field.findall("./enumeratedValues")

        for enum_values in enum_values_list:
            # Check all enumeratedValue items
            for enum_value in enum_values.findall("./enumeratedValue"):
                value_elem = enum_value.find("value")
                name_elem = enum_value.find("name")

                if value_elem is not None and value_elem.text:
                    value_text = value_elem.text
                    name = name_elem.text if name_elem is not None else "unnamed"

                    try:
                        # Handle binary format (#10101)
                        if value_text.startswith('#'):
                            # Convert binary string to integer
                            int_value = int(value_text[1:], 2)
                        # Handle hexadecimal format (0x...)
                        elif value_text.lower().startswith('0x'):
                            int_value = int(value_text, 0)  # Base 0 auto-detects hex/octal/decimal
                        else:
                            # Regular decimal value
                            int_value = int(value_text)

                        # Check if the value exceeds the maximum allowed
                        if int_value > max_value:
                            print(f"  Field '{field_name}': Value {int_value} for '{name}' exceeds max {max_value} (bit width: {bit_width})")

                            # Truncate the value to fit within the bit width
                            new_value = int_value & max_value

                            # Update the value in the appropriate format
                            if value_text.startswith('#'):
                                # Keep binary format with same length padding
                                binary_width = len(value_text) - 1
                                binary_str = bin(new_value)[2:]  # Remove '0b' prefix
                                value_elem.text = f"#{'0' * (binary_width - len(binary_str))}{binary_str}"
                            elif value_text.lower().startswith('0x'):
                                # Keep hex format with same prefix
                                value_elem.text = hex(new_value)
                            else:
                                value_elem.text = str(new_value)

                            print(f"    Fixed: Changed value to {value_elem.text}")
                            fixed_count += 1

                    except ValueError:
                        # Skip values that can't be parsed (invalid format)
                        print(f"  Warning: Could not parse value '{value_text}' for '{name}' in field '{field_name}'")
                        continue

    # Second pass: Handle standalone enumeratedValues (not under a field)
    for enum_values in root.findall(".//enumeratedValues"):
        # Skip those under fields (already processed)
        parent = enum_values.getparent() if hasattr(enum_values, 'getparent') else enum_values.find("..")
        if parent is not None and parent.tag == "field":
            continue

        # Find all enumeratedValue elements
        for enum_value in enum_values.findall("./enumeratedValue"):
            value_elem = enum_value.find("value")
            name_elem = enum_value.find("name")

            if value_elem is not None and value_elem.text:
                value_text = value_elem.text
                name = name_elem.text if name_elem is not None else "unnamed"

                # For standalone enum values, we need to make an educated guess about bit width
                # Look for patterns in other values in the same enumeratedValues group
                bit_width = None
                max_val = 0

                # Find max value to determine required bit width
                for ev in enum_values.findall("./enumeratedValue/value"):
                    if ev is not None and ev.text:
                        try:
                            val = int(ev.text, 0) if ev.text.lower().startswith('0x') else \
                                  int(ev.text[1:], 2) if ev.text.startswith('#') else int(ev.text)
                            max_val = max(max_val, val)
                        except ValueError:
                            continue

                # Calculate bit width based on max value
                if max_val > 0:
                    bit_width = max_val.bit_length()
                    max_value = (1 << bit_width) - 1
                else:
                    # Default to 32-bit if we can't determine
                    bit_width = 32
                    max_value = 0xFFFFFFFF

                try:
                    # Parse the value
                    if value_text.startswith('#'):
                        int_value = int(value_text[1:], 2)
                    elif value_text.lower().startswith('0x'):
                        int_value = int(value_text, 0)
                    else:
                        int_value = int(value_text)

                    # Check if value is valid (simple check for standalone enums)
                    if int_value > max_value:
                        print(f"  Standalone enumeratedValue: Value {int_value} for '{name}' exceeds max {max_value}")

                        # Truncate the value
                        new_value = int_value & max_value

                        # Update in the appropriate format
                        if value_text.startswith('#'):
                            binary_width = len(value_text) - 1
                            binary_str = bin(new_value)[2:]  # Remove '0b' prefix
                            value_elem.text = f"#{'0' * (binary_width - len(binary_str))}{binary_str}"
                        elif value_text.lower().startswith('0x'):
                            value_elem.text = hex(new_value)
                        else:
                            value_elem.text = str(new_value)

                        print(f"    Fixed: Changed value to {value_elem.text}")
                        fixed_count += 1

                except ValueError:
                    print(f"  Warning: Could not parse standalone value '{value_text}' for '{name}'")
                    continue

    print(f"Fixed {fixed_count} out-of-range enumerated values")

def fix_write_constraints(root):
    """Fix SVD file by ensuring writeConstraint ranges are within valid bit width limits for their fields."""
    print("Fixing write constraints with out-of-range values...")

    # Track how many changes we make
    fixed_count = 0

    # Find all fields containing writeConstraint elements
    for field in root.findall(".//field"):
        # Get field name for logging
        field_name_elem = field.find("name")
        field_name = field_name_elem.text if field_name_elem is not None else "unnamed"

        # Find writeConstraint element
        write_constraint = field.find("./writeConstraint")
        if write_constraint is None:
            continue

        # Check for range constraints
        range_elem = write_constraint.find("./range")
        if range_elem is None:
            continue

        # Get minimum and maximum values
        minimum_elem = range_elem.find("./minimum")
        maximum_elem = range_elem.find("./maximum")

        if minimum_elem is None or maximum_elem is None:
            continue

        # Get bit width from lsb and msb
        lsb_elem = field.find("lsb")
        msb_elem = field.find("msb")

        # Calculate bit width
        if lsb_elem is not None and msb_elem is not None:
            lsb = int(lsb_elem.text)
            msb = int(msb_elem.text)
            bit_width = msb - lsb + 1
            max_allowed = (1 << bit_width) - 1  # Calculate maximum value: 2^bit_width - 1
        else:
            # Try alternative: check for bitWidth element
            bit_width_elem = field.find("bitWidth")
            if bit_width_elem is not None:
                bit_width = int(bit_width_elem.text)
                max_allowed = (1 << bit_width) - 1
            # Check for bitOffset element with bitWidth
            elif field.find("bitOffset") is not None and field.find("bitWidth") is not None:
                bit_width = int(field.find("bitWidth").text)
                max_allowed = (1 << bit_width) - 1
            else:
                # If we can't determine bit width, log a warning and skip
                print(f"  Warning: Couldn't determine bit width for field '{field_name}' with writeConstraint, skipping")
                continue

        # Parse minimum and maximum values
        try:
            min_text = minimum_elem.text
            max_text = maximum_elem.text

            # Handle different number formats (hex, decimal)
            if min_text.lower().startswith('0x'):
                min_val = int(min_text, 0)  # Base 0 for auto-detection
            else:
                min_val = int(min_text)

            if max_text.lower().startswith('0x'):
                max_val = int(max_text, 0)
            else:
                max_val = int(max_text)

            # Check if values exceed the allowed range
            need_fix = False

            if min_val > max_allowed:
                print(f"  Field '{field_name}': Minimum value {min_val} exceeds max allowed {max_allowed} (bit width: {bit_width})")
                min_val = min_val & max_allowed
                need_fix = True

            if max_val > max_allowed:
                print(f"  Field '{field_name}': Maximum value {max_val} exceeds max allowed {max_allowed} (bit width: {bit_width})")
                max_val = max_val & max_allowed
                need_fix = True

            # Update values if needed while preserving format
            if need_fix:
                if min_text.lower().startswith('0x'):
                    minimum_elem.text = hex(min_val)
                else:
                    minimum_elem.text = str(min_val)

                if max_text.lower().startswith('0x'):
                    maximum_elem.text = hex(max_val)
                else:
                    maximum_elem.text = str(max_val)

                print(f"    Fixed: Changed range to [{minimum_elem.text} - {maximum_elem.text}]")
                fixed_count += 1

        except ValueError:
            print(f"  Warning: Could not parse writeConstraint values for field '{field_name}', skipping")
            continue

    print(f"Fixed {fixed_count} out-of-range write constraints")

def remove_empty_name_enumerated_values(root):
    """Remove enumerated values that have empty name tags."""
    print("Removing enumerated values with empty name tags...")

    # Track how many elements we remove
    removed_count = 0
    removed_parent_count = 0

    # Find all enumeratedValues elements first
    for enum_values in root.findall(".//enumeratedValues"):
        # We'll collect the indices of children to remove
        to_remove = []

        # Check each enumeratedValue child
        for i, enum_value in enumerate(enum_values.findall("./enumeratedValue")):
            # Try to find the name element
            name_elem = enum_value.find("name")

            # Empty name tag can be either:
            # 1. <name /> (self-closing tag)
            # 2. <name></name> (empty content)
            # 3. <name>   </name> (whitespace only)
            if name_elem is not None and (name_elem.text is None or name_elem.text.strip() == ""):
                # Get description and value for logging
                desc_elem = enum_value.find("description")
                desc_text = desc_elem.text if desc_elem is not None and desc_elem.text else "No description"

                value_elem = enum_value.find("value")
                value_text = value_elem.text if value_elem is not None and value_elem.text else "No value"

                print(f"  Found enumerated value with empty name: description='{desc_text}', value='{value_text}'")
                to_remove.append(i)

        # Remove the elements from last to first to avoid index shifting
        for idx in sorted(to_remove, reverse=True):
            # Use direct index access since we're working with a parent element
            try:
                # enumeratedValues elements typically contain multiple enumeratedValue children
                children = list(enum_values)
                if idx < len(children):
                    enum_values.remove(children[idx])
                    removed_count += 1
                    print(f"  Removed enumerated value at index {idx}")
            except Exception as e:
                print(f"  Warning: Could not remove enumerated value: {str(e)}")

        # Check if the enumeratedValues element is now empty
        # If it contains no more enumeratedValue children, remove it too
        if len(enum_values.findall("./enumeratedValue")) == 0:
            # Find the parent of enumeratedValues
            parent = None
            for potential_parent in root.findall(".//*"):
                if enum_values in potential_parent:
                    parent = potential_parent
                    break

            if parent is not None:
                print(f"  Removing empty enumeratedValues element")
                parent.remove(enum_values)
                removed_parent_count += 1
            else:
                print(f"  Warning: Could not find parent for empty enumeratedValues element")

    print(f"Removed {removed_count} enumerated values with empty name tags and {removed_parent_count} empty enumeratedValues elements")

    # If no elements were removed but we found some, it might be due to a different XML structure
    # Let's try an alternative approach for more complex XML structures
    if removed_count == 0:
        print("Trying alternative approach to remove enumerated values with empty names...")

        # This more direct approach modifies the XML tree by replacing enumeratedValues elements
        for parent in root.findall(".//*[enumeratedValues]"):
            for enum_values in parent.findall("./enumeratedValues"):
                # Create a new enumeratedValues element
                new_enum_values = ET.Element("enumeratedValues")

                # Copy only valid enumeratedValue elements
                valid_count = 0
                total_count = 0
                for enum_value in enum_values.findall("./enumeratedValue"):
                    total_count += 1
                    name_elem = enum_value.find("name")
                    if name_elem is None or (name_elem.text is not None and name_elem.text.strip() != ""):
                        # This is a valid element, copy it
                        new_enum_values.append(enum_value)
                        valid_count += 1
                    else:
                        # Log the skipped element
                        desc_elem = enum_value.find("description")
                        desc_text = desc_elem.text if desc_elem is not None and desc_elem.text else "No description"

                        value_elem = enum_value.find("value")
                        value_text = value_elem.text if value_elem is not None and value_elem.text else "No value"

                        print(f"  Skipping enumerated value with empty name: description='{desc_text}', value='{value_text}'")
                        removed_count += 1

                # If all enumeratedValue elements were removed, remove the entire enumeratedValues element
                if valid_count == 0:
                    parent.remove(enum_values)
                    removed_parent_count += 1
                    print(f"  Removed empty enumeratedValues element")
                # Otherwise, replace the old enumeratedValues with the new one if we removed any elements
                elif valid_count < total_count:
                    idx = list(parent).index(enum_values)
                    parent.remove(enum_values)
                    parent.insert(idx, new_enum_values)
                    print(f"  Replaced enumeratedValues element with cleaned version")

        print(f"Alternative approach: Removed {removed_count} enumerated values with empty name tags and {removed_parent_count} empty enumeratedValues elements")

def fix_missing_placeholders(root):
    """Fix SVD file by correcting register names with dimension attributes but missing %s placeholder."""
    print("Fixing register names with missing %s placeholders...")

    # Track how many names we fix
    fixed_count = 0

    # Find all registers with dimension attributes
    for register in root.findall(".//register"):
        dim_elem = register.find("dim")
        if dim_elem is None or not dim_elem.text:
            continue

        name_elem = register.find("name")
        if name_elem is None or not name_elem.text:
            continue

        register_name = name_elem.text

        # Check if the register name has a %s placeholder
        if "%s" not in register_name:
            print(f"  Found register with dimension attributes but no %s placeholder: {register_name}")

            # We'll try to identify a character that should be replaced with %s
            # Let's check the alternateRegister attribute first
            alt_reg = register.find("alternateRegister")
            if alt_reg is not None and alt_reg.text and "%s" in alt_reg.text:
                # Extract the pattern from alternateRegister
                alt_pattern = alt_reg.text

                # Find where the %s is in the alternate register name
                alt_parts = alt_pattern.split("%s")

                # Try to locate the corresponding section in the actual register name
                if len(alt_parts) == 2:
                    prefix = alt_parts[0]
                    suffix = alt_parts[1]

                    # If the register name starts with the prefix and ends with the suffix,
                    # the character(s) in between should be replaced with %s
                    if register_name.startswith(prefix) and register_name.endswith(suffix):
                        middle = register_name[len(prefix):-len(suffix) if len(suffix) > 0 else None]
                        if middle:
                            # Replace the middle part with %s
                            new_name = f"{prefix}%s{suffix}"
                            print(f"    Fixing name: {register_name} → {new_name}")
                            name_elem.text = new_name
                            fixed_count += 1
                            continue

            # If we couldn't fix using alternateRegister, try looking for a pattern in sibling registers
            parent = register.getparent() if hasattr(register, 'getparent') else None
            if parent is not None:
                # Find sibling registers with similar names
                siblings = []
                for sibling in parent.findall("./register"):
                    # Skip the current register
                    if sibling == register:
                        continue

                    sibling_name = sibling.find("name")
                    if sibling_name is not None and sibling_name.text:
                        siblings.append(sibling_name.text)

                # Look for registers with similar names but with %s
                similar_name_found = False
                for sibling_name in siblings:
                    if "%s" in sibling_name:
                        # Check if the sibling name has a similar pattern
                        # by replacing %s with a character and comparing
                        for char in "0123456789abcdefghijklmnopqrstuvwxyz":
                            test_name = sibling_name.replace("%s", char)

                            # Calculate similarity based on character substitution
                            if len(register_name) == len(test_name):
                                differing_positions = [(i, register_name[i], test_name[i])
                                                      for i in range(len(register_name))
                                                      if register_name[i] != test_name[i]]

                                # If there's only one difference and it matches our test character,
                                # we've found the character to replace with %s
                                if len(differing_positions) == 1:
                                    pos, reg_char, test_char = differing_positions[0]
                                    if test_char == char:
                                        # Replace the character at this position with %s
                                        new_name = register_name[:pos] + "%s" + register_name[pos+1:]
                                        print(f"    Fixing name by similarity: {register_name} → {new_name}")
                                        name_elem.text = new_name
                                        fixed_count += 1
                                        similar_name_found = True
                                        break

                        if similar_name_found:
                            break

            # If we still couldn't fix the name, check for specific patterns we know about
            if register_name == "SQCH1DSCmDR_H":
                new_name = "SQCH1DSC%sDR_H"
                print(f"    Fixing name by known pattern: {register_name} → {new_name}")
                name_elem.text = new_name
                fixed_count += 1

    print(f"Fixed {fixed_count} register names with missing %s placeholders")

def fix_reset_values(root):
    """Fix SVD file by ensuring resetValue is within valid range according to resetMask."""
    print("Fixing out-of-range resetValues...")

    # Track how many values we fix
    fixed_count = 0

    # Find all registers with resetValue and resetMask
    for register in root.findall(".//register"):
        reset_value_elem = register.find("resetValue")
        reset_mask_elem = register.find("resetMask")

        # Skip if either element is missing
        if reset_value_elem is None or reset_mask_elem is None:
            continue

        # Get register name for logging
        name_elem = register.find("name")
        register_name = name_elem.text if name_elem is not None and name_elem.text else "unnamed"

        try:
            # Parse values (support decimal, hex, binary)
            if reset_value_elem.text.lower().startswith("0x"):
                reset_value = int(reset_value_elem.text, 16)
            elif reset_value_elem.text.startswith("#"):
                reset_value = int(reset_value_elem.text[1:], 2)
            else:
                reset_value = int(reset_value_elem.text)

            if reset_mask_elem.text.lower().startswith("0x"):
                reset_mask = int(reset_mask_elem.text, 16)
            elif reset_mask_elem.text.startswith("#"):
                reset_mask = int(reset_mask_elem.text[1:], 2)
            else:
                reset_mask = int(reset_mask_elem.text)

            # Check if the resetValue has bits set outside the resetMask
            if (reset_value & ~reset_mask) != 0:
                # Fix by masking the resetValue with resetMask
                original_value = reset_value
                fixed_value = reset_value & reset_mask

                # Preserve the original format (hex, decimal) when updating
                if reset_value_elem.text.lower().startswith("0x"):
                    reset_value_elem.text = f"0x{fixed_value:X}"
                elif reset_value_elem.text.startswith("#"):
                    # Convert to binary and preserve width
                    binary_width = len(reset_value_elem.text) - 1
                    binary_str = bin(fixed_value)[2:]  # Remove '0b' prefix
                    reset_value_elem.text = f"#{'0' * (binary_width - len(binary_str))}{binary_str}"
                else:
                    reset_value_elem.text = str(fixed_value)

                print(f"  Register '{register_name}': Fixed resetValue from {hex(original_value)} to {hex(fixed_value)} (mask: {hex(reset_mask)})")
                fixed_count += 1

        except ValueError:
            print(f"  Warning: Could not parse resetValue or resetMask for register '{register_name}'")
            continue

    print(f"Fixed {fixed_count} out-of-range resetValues")

def fix_name_enumerated(root):
    """
    Fix SVD file by prefixing enumerated value names with their field name.
    This prevents svd2rust errors with duplicate enumerated values like '_0', '_1'.

    For example, converts:
    <name>0</name> to <name>FIELD_NAME_0</name>
    """
    print("Fixing enumerated value names by prefixing with field name...")

    # Track how many names we fix
    fixed_count = 0

    # Find all fields containing enumeratedValues
    for field in root.findall(".//field"):
        field_name_elem = field.find("name")
        if field_name_elem is None or not field_name_elem.text:
            continue

        field_name = field_name_elem.text

        # Look for enumeratedValues within this field
        enum_values_list = field.findall("./enumeratedValues")

        for enum_values in enum_values_list:
            # Check all enumeratedValue items
            for enum_value in enum_values.findall("./enumeratedValue"):
                name_elem = enum_value.find("name")

                if name_elem is not None and name_elem.text:
                    original_name = name_elem.text

                    # Only fix numeric or simple names that are likely to cause conflicts
                    # These are typically "0", "1", etc. which svd2rust converts to _0, _1
                    if original_name.isdigit() or original_name in ['true', 'false'] or len(original_name) <= 2:
                        new_name = f"{field_name}_{original_name}"

                        # Update the name
                        name_elem.text = new_name
                        print(f"  Prefixed enumerated value name: {original_name} -> {new_name}")
                        fixed_count += 1

    print(f"Fixed {fixed_count} enumerated value names")
    return root

def preprocess_svd_file(svd_file_path):
    """
    Preprocess SVD file to fix issues that might cause problems with svd2rust.
    Returns the path to the preprocessed file.
    """
    # Create a temporary directory for preprocessed files
    temp_dir = tempfile.mkdtemp()

    try:
        # Parse the SVD file
        tree = ET.parse(svd_file_path)
        root = tree.getroot()

        # Apply fixes based on configuration
        if FIX_DERIVED_FROM:
            fix_derived_from(root)

        if FIX_ENUMERATED_VALUES:
            fix_enumerated_values(root)

        if FIX_ENUMERATED_VALUE_RANGES:
            fix_enumerated_value_ranges(root)

        if FIX_WRITE_CONSTRAINTS:
            fix_write_constraints(root)

        if FIX_EMPTY_NAME_ENUMERATED_VALUES:
            remove_empty_name_enumerated_values(root)

        if FIX_MISSING_PLACEHOLDERS:
            fix_missing_placeholders(root)

        if FIX_RESET_VALUES:
            fix_reset_values(root)

        if FIX_ACCESS_TYPES:
            fix_access_types(root)

        if FIX_ACCESS_TYPES_NESTED:
            fix_access_types_nested(root)

        if FIX_MISSING_ACCESS_TAGS:
            add_missing_access_tags(root)

        if FIX_NAME_ENUMERATED:
            fix_name_enumerated(root)

        # Get the filename
        filename = os.path.basename(svd_file_path)

        # Write the modified XML to a temporary file with proper formatting
        temp_file_path = os.path.join(temp_dir, filename)

        # Use our custom XML writing function that preserves formatting
        write_formatted_xml(root, temp_file_path)

        return temp_file_path, temp_dir

    except Exception as e:
        print(f"Error preprocessing XML in {svd_file_path}: {str(e)}")
        sys.exit(1)

def write_formatted_xml(root, file_path):
    """
    Write XML with proper formatting for value elements.
    This specifically ensures proper formatting for value elements added to enumeratedValue elements.
    """
    import re

    # First convert the XML to a string
    xml_string = ET.tostring(root, encoding='utf-8').decode('utf-8')

    # Use regex to find the pattern where a value tag follows an isDefault tag
    # This pattern looks for <isDefault>true</isDefault> followed by <value> tag
    pattern = r'(<isDefault>true</isDefault>)(<value>[^<]+</value>)'

    # Replace with properly formatted version (add a line break and indentation)
    def format_value_tag(match):
        is_default_tag = match.group(1)
        value_tag = match.group(2)

        # Find the indentation of the isDefault tag by looking at the characters before it
        start_idx = match.start(1)
        line_start = xml_string.rfind('\n', 0, start_idx)
        if line_start == -1:
            line_start = 0

        # Calculate the indentation (spaces before isDefault tag)
        indent = xml_string[line_start:start_idx]

        # Return the formatted string with line break and indentation
        return f"{is_default_tag}\n{indent}{value_tag}"

    # Apply the formatting
    formatted_xml = re.sub(pattern, format_value_tag, xml_string)

    # Write with XML declaration
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write('<?xml version="1.0" encoding="utf-8"?>\n')
        f.write(formatted_xml)

def process_device(device_name, patch_only=False, pac_only=False):
    """
    Process a single device SVD file.

    Args:
        device_name: Name of the device (without .svd extension)
        patch_only: If True, only generate patched SVD files, skip PAC generation
        pac_only: If True, skip SVD preprocessing and use existing patched files

    Returns:
        bool: True if processing was successful
    """
    svd_file = f"svd/vendor/{device_name}.svd"
    patched_svd_file = f"patched_svd/{device_name}.svd"

    # Check if the SVD file exists (unless we're only generating PACs from existing files)
    if not pac_only and not os.path.exists(svd_file):
        print(f"SVD file {svd_file} not found.")
        return False

    print(f"\n===== Processing device: {device_name} =====")

    # Skip SVD preprocessing if we're only generating PACs from existing patched files
    temp_dir = None
    if not pac_only:
        # Preprocess the SVD file
        print(f"Preprocessing SVD file {device_name}...")
        preprocessed_svd, temp_dir = preprocess_svd_file(svd_file)

        # Ensure patched_svd directory exists
        os.makedirs("patched_svd", exist_ok=True)

        # Copy the preprocessed SVD file to the patched_svd directory
        dst = patched_svd_file
        try:
            shutil.copyfile(preprocessed_svd, dst)
            print(f"Copied {preprocessed_svd} to {dst}")
        except Exception as e:
            print(f"Error copying preprocessed SVD file: {str(e)}")
            return False
    else:
        print(f"PAC-only mode: Using existing patched SVD file for {device_name}")
        if not os.path.exists(patched_svd_file):
            print(f"Error: Patched SVD file {patched_svd_file} not found.")
            return False

    # If patch_only mode, we're done
    if patch_only:
        print(f"SVD-only mode: Skipping PAC generation for {device_name}")
        # Clean up temporary directory before exiting
        if temp_dir and os.path.exists(temp_dir):
            shutil.rmtree(temp_dir)
        return True

    try:
        # Create directories (package name)
        if (device_name.startswith("R7FA")):
            package_name = device_name.lower()[0] + device_name.lower()[3:7]
        elif (device_name.startswith("DA")):
            package_name = device_name.lower()
        elif (device_name.startswith("U5")):
            package_name = device_name.lower()
        else:
            print(f"Unknown device name format: {device_name}")
            return False

        pac_dir = Path(f"pac/{package_name}")
        pac_dir.mkdir(parents=True, exist_ok=True)

        # If there's a patched SVD file available, try using that
        patched_svd = os.path.join("patched_svd", f"{device_name.upper()}.svd")
        if os.path.exists(patched_svd):
            print(f"Found patched SVD file: {patched_svd}")
            print(f"Attempting to run svd2rust with the patched file {patched_svd} to create {package_name}...")
            # Use quotes for the file paths to correctly handle backslashes on Windows
            patched_cmd = f'svd2rust -i "{patched_svd}" -o "{pac_dir}" --target cortex-m --atomics --impl-debug --field-names-for-enums --reexport-core-peripherals --feature-peripheral --ident-formats-theme legacy -f type:prefix:case:suffix --log info'
            print(f"Running command: {patched_cmd}")
            result = run_command(patched_cmd, check=False)

            if result.returncode != 0:
                print(f"Warning: svd2rust failed for {device_name}")
                print(f"Output: {result.stdout}")
                print(f"Error: {result.stderr}")
                if not pac_only:
                    # Save the problematic SVD file for debugging
                    debug_dir = Path("debug_svd")
                    debug_dir.mkdir(exist_ok=True)
                    debug_file = debug_dir / f"{device_name}_debug.svd"
                    shutil.copyfile(preprocessed_svd, debug_file)
                    print(f"Saved problematic SVD file to {debug_file} for debugging")
                return False
            else:
                print(f"Successfully generated files using patched SVD file")
        else:
            print(f"No patched SVD file found for {device_name}")
            print(f"You may need to manually create a patched SVD file at: {patched_svd}")
            return False

        # Format files in src directory
        src_dir_path = pac_dir / "src"
        if src_dir_path.exists():
            print(f"Formatting all Rust files in {src_dir_path}...")
            rs_files = list(src_dir_path.glob("**/*.rs"))
            if rs_files:
                formatted_files = [str(file) for file in rs_files]
                print(f"Found {len(formatted_files)} Rust files to format")
                run_command(f"rustfmt {' '.join(formatted_files)}", check=False)
            else:
                print(f"No Rust files found in {src_dir_path}")
        else:
            print(f"Warning: {src_dir_path} not found for formatting")

        # Check and generate docs for the device PAC
        if pac_dir.exists():
            # Update Cargo.toml with the manifest template and features
            if generate_cargo_toml(pac_dir, package_name):
                # Change to the pac directory
                os.chdir(pac_dir)
                try:
                    run_command("cargo fix --all --allow-dirty --features all-peripherals", check=False)
                    run_command("cargo fmt --all", check=False)
                    # Remove old documentation if it exists
                    doc_path = pac_dir / "target" / "doc"
                    if doc_path.exists():
                        shutil.rmtree(doc_path)
                    run_command("cargo doc --no-deps", check=False)
                    # Print path to documentation
                    DOC_PATH = doc_path / f"{package_name.replace('-', '_')}" / "index.html"
                    if DOC_PATH.exists():
                        print(f"Documentation generated at: {DOC_PATH}")
                finally:
                    os.chdir("../..")  # Go back to the original directory
            else:
                print(f"Failed to generate Cargo.toml for {package_name}")
                return False

        return True

    finally:
        # Clean up temporary directory
        if temp_dir and os.path.exists(temp_dir):
            shutil.rmtree(temp_dir)

def main():
    # Parse command-line arguments
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("-h", "--help", action="store_true", help="Show usage information and exit")
    parser.add_argument("-s", "--svd", action="store_true", help="Generate patched SVD files only")
    parser.add_argument("-p", "--pac", action="store_true", help="Generate PACs from patched SVD files only")
    parser.add_argument("-a", "--all", action="store_true", help="Generate full process (SVD and PAC)")
    parser.add_argument("device_name", help="Process the specified device or 'ALL' for all devices")

    args = parser.parse_args()

    # Show usage information if requested
    if args.help:
        show_usage()
        sys.exit(0)

    # Check if required commands are installed
    check_command_exists("svd2rust")
    check_command_exists("svd")

    # Create directories if they don't exist
    os.makedirs("pac", exist_ok=True)
    os.makedirs("patched_svd", exist_ok=True)

    # Set environment variables
    os.environ["RUST_BACKTRACE"] = RUST_BACKTRACE
    os.environ["RUST_LOG"] = RUST_LOG

    # Determine processing mode
    svd_only = args.svd
    pac_only = args.pac

    # If both are specified, do full process
    if svd_only and pac_only:
        svd_only = False
        pac_only = False
    # If --all is specified, do full process
    elif args.all:
        svd_only = False
        pac_only = False
    # If no mode is specified, default to full process
    elif not svd_only and not pac_only:
        svd_only = False
        pac_only = False

    # Process all devices if ALL is specified
    if args.device_name.upper() == "ALL":
        print("Processing all SVD files...")
        if svd_only:
            print("Mode: Generate patched SVD files only")
        elif pac_only:
            print("Mode: Generate PACs from patched SVD files only")
        else:
            print("Mode: Full process (generate patched SVD files and PACs)")

        # For PAC-only mode with ALL, we need to get the list of available patched SVD files
        if pac_only:
            svd_files = glob.glob("patched_svd/*.svd")
            if not svd_files:
                print("No patched SVD files found in patched_svd directory.")
                sys.exit(1)
        else:
            svd_files = glob.glob("svd/vendor/*.svd")
            if not svd_files:
                print("No SVD files found in svd directory.")
                sys.exit(1)

        successes = 0
        failures = 0

        for svd_file in svd_files:
            # Extract device name from filename
            device_name = os.path.basename(svd_file).split('.')[0]
            if process_device(device_name, patch_only=svd_only, pac_only=pac_only):
                successes += 1
            else:
                failures += 1

        print(f"\nProcessed all SVD files. Successes: {successes}, Failures: {failures}")

        if failures > 0:
            print(f"Warning: {failures} device(s) failed to process properly.")
    else:
        # Process a single device
        device_name = args.device_name

        # Determine which steps to perform
        if svd_only:
            print(f"Mode: Generate patched SVD file only for {device_name}")
            success = process_device(device_name, patch_only=True)
        elif pac_only:
            print(f"Mode: Generate PAC only for {device_name}")
            # We need the patched SVD file first, but we won't regenerate it if it exists
            patched_svd = os.path.join("patched_svd", f"{device_name}.svd")
            if not os.path.exists(patched_svd):
                print(f"Patched SVD file {patched_svd} not found. Generating it first...")
                process_device(device_name, patch_only=True)
            success = process_device(device_name, patch_only=False, pac_only=True)
        else:
            print(f"Mode: Full process for {device_name}")
            success = process_device(device_name, patch_only=False)

        if not success:
            print(f"Failed to process device {device_name}")
            sys.exit(1)


    print("Done.")

if __name__ == "__main__":
    main()
