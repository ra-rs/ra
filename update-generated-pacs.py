import os
import sys
import shutil
import subprocess
import re
from pathlib import Path
import argparse

# Global constants
RUST_BACKTRACE = "1"
RUST_LOG = "info"

def show_usage():
    """Display usage information for the script."""
    print("Usage: python fix_pac.py [OPTIONS] [DEVICE_NAME]")
    print("\nOptions:")
    print("  -h, --help     Show this help message and exit")
    print("  -a, --all      Fix ALL PACs")
    print("\nExamples:")
    print("  python fix_pac.py ra0e1     # Fix only ra0e1")
    print("  python fix_pac.py -a        # Fix all PACs")

def fix_lib_rs(file_path):
    """Fix specific Clippy warnings in lib.rs files."""
    try:
        # Read the contents of lib.rs
        with open(file_path, 'r') as file:
            content = file.read()

        # Make a backup of the original file
        backup_path = f"{file_path}.bak"
        shutil.copy2(file_path, backup_path)

        # Fix "extern blocks must be unsafe" - simplified pattern
        content = content.replace('extern "C" {', 'unsafe extern "C" {')

        # Fix "unsafe attribute used without unsafe" for link_section
        pattern1 = r'#\[link_section\s*=\s*"([^"]*)"\]'
        replacement1 = r'#[unsafe(link_section = "\1")]'
        content = re.sub(pattern1, replacement1, content)

        # Fix "unsafe attribute used without unsafe" for no_mangle
        pattern2 = r'#\[no_mangle\]'
        replacement2 = r'#[unsafe(no_mangle)]'
        content = re.sub(pattern2, replacement2, content)

        # Write the modified content back to lib.rs
        with open(file_path, 'w') as file:
            file.write(content)

        print(f"Successfully fixed Clippy warnings in {file_path}")

        # Delete backup if successful
        if os.path.exists(backup_path):
            os.remove(backup_path)
            print(f"Deleted backup file {backup_path}")

        return True

    except Exception as e:
        print(f"Error processing {file_path}: {str(e)}")
        # Restore backup if exists
        if os.path.exists(backup_path):
            shutil.copy2(backup_path, file_path)
            print(f"Restored backup of {file_path}")
        return False

def fix_common_rs(file_path):
    """Fix unused from_ptr warnings in common.rs files."""
    try:
        # Read the contents of common.rs
        with open(file_path, 'r') as file:
            content = file.read()

        # Make a backup of the original file
        backup_path = f"{file_path}.bak"
        shutil.copy2(file_path, backup_path)

        # Fix for the first from_ptr warning (around line 255)
        pattern1 = r'(pub\(crate\)\s+const\s+fn\s+from_ptr\(ptr:\s+\*mut\s+u8\)\s+->.*?\{)'
        replacement1 = r'#[allow(dead_code)]\n    \1'
        content = re.sub(pattern1, replacement1, content)

        # Fix for the second from_ptr warning (around line 798)
        pattern2 = r'(pub\(crate\)\s+const\s+unsafe\s+fn\s+from_ptr\(ptr:\s+\*mut\s+u8\)\s+->.*?\{)'
        replacement2 = r'#[allow(dead_code)]\n      \1'
        content = re.sub(pattern2, replacement2, content)

        # Write the modified content back to common.rs
        with open(file_path, 'w') as file:
            file.write(content)

        print(f"Successfully fixed unused from_ptr warnings in {file_path}")

        # Delete backup if successful
        if os.path.exists(backup_path):
            os.remove(backup_path)
            print(f"Deleted backup file {backup_path}")

        return True

    except Exception as e:
        print(f"Error processing {file_path}: {str(e)}")
        # Restore backup if exists
        if os.path.exists(backup_path):
            shutil.copy2(backup_path, file_path)
            print(f"Restored backup of {file_path}")
        return False

def process_device(device_name):
    """Process a single device PAC folder to fix Clippy warnings."""
    print(f"Processing {device_name}...")

    # Convert device name to lowercase for consistency
    device_name = device_name.lower()

    # Define the PAC directory
    pac_dir = os.path.join("pac", f"{device_name}")

    # Check if the PAC directory exists
    if not os.path.exists(pac_dir):
        print(f"Error: PAC directory for {device_name} not found at {pac_dir}")
        return False

    success = True

    # Fix lib.rs file
    lib_rs_path = os.path.join(pac_dir, "src", "lib.rs")
    if os.path.exists(lib_rs_path):
        if not fix_lib_rs(lib_rs_path):
            success = False
    else:
        print(f"Warning: lib.rs not found at {lib_rs_path}")

    # Fix common.rs file
    #common_rs_path = os.path.join(pac_dir, "src", "common.rs")
    #if os.path.exists(common_rs_path):
    #    if not fix_common_rs(common_rs_path):
    #        success = False
    #else:
    #    print(f"Warning: common.rs not found at {common_rs_path}")

    return success

def main():
    # Parse command-line arguments
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("-h", "--help", action="store_true", help="Show usage information and exit")
    parser.add_argument("-a", "--all", action="store_true", help="Fix ALL PACs")
    parser.add_argument("device_name", nargs="?", help="Fix PAC for the specified device")

    args = parser.parse_args()

    # Show usage information if requested
    if args.help:
        show_usage()
        sys.exit(0)

    # Create directories if they don't exist
    os.makedirs("pac", exist_ok=True)

    # Set environment variables
    os.environ["RUST_BACKTRACE"] = RUST_BACKTRACE
    os.environ["RUST_LOG"] = RUST_LOG

    # Determine processing mode
    process_all = args.all

    # Process all devices if ALL is specified
    if process_all:
        print("Processing all PAC files...")

        # Get list of PAC directories
        pac_dirs = [d for d in os.listdir("pac") if os.path.isdir(os.path.join("pac", d)) and d.endswith("")]
        if not pac_dirs:
            print("No PAC directories found.")
            sys.exit(1)

        successes = 0
        failures = 0

        for pac_dir in pac_dirs:
            if process_device(pac_dir):  # Use pac_dir instead of device_name
                successes += 1
            else:
                failures += 1

        print(f"\nProcessed all PAC files. Successes: {successes}, Failures: {failures}")

        if failures > 0:
            print(f"Warning: {failures} device(s) failed to process properly.")
    else:
        # Process a single device
        if not args.device_name:
            print("Error: No device name specified.")
            show_usage()
            sys.exit(1)

        device_name = args.device_name
        print(f"Mode: Fix Clippy warnings for {device_name}")

        success = process_device(device_name)

        if not success:
            print(f"Failed to process device {device_name}")
            sys.exit(1)

    print("Done.")

if __name__ == "__main__":
    main()
