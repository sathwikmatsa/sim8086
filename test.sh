#!/bin/bash

# Function to check if the file has the .asm extension and get filename without extension
check_asm_file_extension() {
    local filename="$1"
    
    # Extract the file extension
    local extension="${filename##*.}"
    
    # Extract the filename without the extension
    local basename="${filename%.*}"
    
    # Check if the file has no extension
    if [[ "$filename" == "$extension" ]]; then
        echo "No file extension found. Only '.asm' files are supported. Please provide a file with the '.asm' extension."
        exit 1
    fi
    
    # Check if the file extension is .asm
    if [[ "$extension" != "asm" ]]; then
        echo "Unsupported file extension: '.${extension}'. Only '.asm' files are supported."
        exit 1
    fi
}

# Check if a filename is provided
if [ -z "$1" ]; then
    echo "No file provided. Please provide a file with the '.asm' extension."
    exit 1
fi

# Call the function with the provided filename
check_asm_file_extension "$1"

filename="$1"
basename="${filename%.*}"

nasm ${filename}

cargo run --quiet -- ${basename}

nasm "${basename}.8086.decoded"

diff ${basename} "${basename}.8086"
