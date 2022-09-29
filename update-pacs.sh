#!/usr/bin/env bash

deps=$'
[dependencies]
cortex-m = \"0.7\"
vcell = \"0.1.2\"

[dependencies.cortex-m-rt]
optional = true
version = \"0.6.13\"

[features]
rt = [\"cortex-m-rt/device\"]
'

# Copy SVDs to patch directory as lowercase names
mkdir -p svd/patch
for i in $(ls svd/vendor); do
    o=$(echo $i | tr '[:upper:]' '[:lower:]')
    cp "svd/vendor/${i}" "svd/patch/${o}"
done
# Apply patches
sed -i 's/A-B/A,B/' svd/patch/r7fa6m5bh.svd # Required so that svdtools can parse this properly
patches=$(ls --hide='periph' svd/device)
for p in $patches; do
    svdtools patch "svd/device/${p}"
done

# Generate PACs
svds=$(ls svd/patch/*.patched)
for s in $svds; do
    name_upper=$(echo ${s} | sed 's/7f//' | cut -c 11-15)
    name=$(echo ${name_upper} | tr '[:upper:]' '[:lower:]')
    pac=$(echo ${name} | sed 's/^/pac\//')
    if [ -d "${pac}" ]
    then
        rm -Rf ${pac}
        mkdir ${pac}
    else 
        mkdir ${pac}
    fi
    echo "Found device family ${name_upper}"
    svd2rust --nightly -i $s -o $pac
    if [ $? != 0 ]
    then
        rm -Rf ${pac}
        continue
    fi
    form -i "${pac}/lib.rs" -o "${pac}/src"
    rm "${pac}/lib.rs"
    
manifest=$'[package]
name = \"@name@\"
description = \"Peripheral access API for @NAME@ microcontrollers (generated using svd2rust)\"
version = \"@version@\"
authors = [\"Nathan Larsen <n8tlarsen@gmail.com>\"]
keywords = [\"no-std\", \"arm\", \"cortex-m\"]
categories = [\"embedded\", \"hardware-support\", \"no-std\"]
license = \"MIT OR Apache-2.0\"
repository = \"https://github.com/ra-rs/ra\"
readme = \"README.md\"
edition = \"2021\"
'
    echo "${manifest}${deps}" | sed s/@name@/${name}/ | sed s/@NAME@/${name_upper}/ | sed s/@version@/${1}/ > ${pac}/Cargo.toml
    echo "Peripheral access API for ${name_upper} microcontrollers (generated using svd2rust)" > ${pac}/README.md
    cargo +nightly fmt --manifest-path "${pac}/Cargo.toml"
    rustfmt +nightly "${pac}/build.rs"
done

# Clean up
rm -Rf svd/patch
