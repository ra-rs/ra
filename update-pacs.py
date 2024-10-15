import os
import shutil
import subprocess
import sys
import re
import argparse

# Define dependencies
deps = '''
[dependencies]
critical-section = { version = "1.1", optional = true }
cortex-m = "0.7"
cortex-m-rt = { version = "0.6", optional = true }
vcell = "0.1"
portable-atomic = { version = "0.3", default-features = false, optional = true }

[features]
rt = ["cortex-m-rt/device"]
atomics = ["portable-atomic"]
'''

# Define manifest template
manifest_template = '''
[package]
name = "@name@"
description = "Peripheral access API for @NAME@ microcontrollers (generated using svd2rust)"
version = "@version@"
authors = ["Nathan Larsen <n8tlarsen@gmail.com>", "Addison Heavner <addisonheavner@gmail.com>"]
keywords = ["no-std", "arm", "cortex-m", "renesas"]
categories = ["embedded", "hardware-support", "no-std"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/ra-rs/ra"
readme = "README.md"
edition = "2021"
'''

def build(version):
    # Create patch directory
    os.makedirs('svd/patch', exist_ok=True)

    # Copy SVDs to patch directory as lowercase names
    for filename in os.listdir('svd/vendor'):
        src = os.path.join('svd/vendor', filename)
        dst = os.path.join('svd/patch', filename.lower())
        shutil.copyfile(src, dst)

    # Apply patches
    with open('svd/patch/r7fa6m5bh.svd', 'r', encoding='utf-8') as file:
        data = file.read().replace('A-B', 'A,B')
    with open('svd/patch/r7fa6m5bh.svd', 'w', encoding='utf-8') as file:
        file.write(data)

    patches = [f for f in os.listdir('svd/device') if f != 'periph']
    for patch in patches:
        subprocess.run(['svdtools', 'patch', os.path.join('svd/device', patch)])

    # Generate PACs
    svds = [f for f in os.listdir('svd/patch') if f.endswith('.patched')]
    for svd in svds:
        name_upper = svd.replace('7f', '')[0:5]
        name = re.sub(r'[^a-zA-Z0-9_-]', '_', name_upper.lower())
        pac = os.path.join('pac', name)
        if os.path.exists(pac):
            shutil.rmtree(pac)
        os.makedirs(pac)

        print(f"Found device family {name_upper}")
        result = subprocess.run(['svd2rust', '--atomics', '-i', os.path.join('svd/patch', svd), '-o', pac])
        if result.returncode != 0:
            shutil.rmtree(pac)
            continue

        subprocess.run(['form', '-i', os.path.join(pac, 'lib.rs'), '-o', os.path.join(pac, 'src')])
        os.remove(os.path.join(pac, 'lib.rs'))

        manifest = manifest_template.replace('@name@', name).replace('@NAME@', name_upper).replace('@version@', version)
        with open(os.path.join(pac, 'Cargo.toml'), 'w') as file:
            file.write(manifest + deps)

        with open(os.path.join(pac, 'README.md'), 'w') as file:
            file.write(f"Peripheral access API for {name_upper} microcontrollers (generated using svd2rust)")

        subprocess.run(['cargo', 'fmt', '--manifest-path', os.path.join(pac, 'Cargo.toml')])
        subprocess.run(['rustfmt', os.path.join(pac, 'build.rs')])
        subprocess.run(['cargo', 'build', '--manifest-path', os.path.join(pac, 'Cargo.toml'), '--features', 'atomics'])
        print()  # Add blank line to separate status outputs

    # Clean up
    shutil.rmtree('svd/patch')

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
                    prog='update-pacs',
                    description='Rebuild Renesas PACs',
    )
    parser.add_argument('version', help="Version number of generated PACs")
    args = parser.parse_args()
    build(args.version)
