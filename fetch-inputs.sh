#!/usr/bin/env bash

# Usage: ./fetch-inputs.sh <output_directory> <year>

if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <cookie_file> <output_directory> <year>"
    exit 1
fi

COOKIE_FILE="$1"
INPUT_DIR="$2"
YEAR="$3"

# Create the input directory if it doesn't exist
mkdir -p "$INPUT_DIR"

# Retrieve cookie
COOKIE=$(cat $COOKIE_FILE)

for DAY in {1..25}; do
    NAME=$(printf "%02d" $DAY)
    INPUT_FILE="$INPUT_DIR/day_$NAME.txt"

    # Skip download if input file already exists
    if [ -e "$INPUT_FILE" ]; then
        echo "Day $DAY already exists. Skipping..."
        continue
    fi

    URL="https://adventofcode.com/$YEAR/day/$DAY/input"

    echo "Downloading Day $DAY input..."
    
    # Fetch the input using curl
    curl --fail --silent --show-error \
    --cookie "session=$COOKIE" \
    --output "$INPUT_FILE" "$URL"

    # Check if the download was successful
    if [ $? -eq 0 ]; then
        echo "Day $DAY input downloaded successfully."
    else
        echo "Error downloading input for Day $DAY. Check your session cookie or the year/day."

        # Clean up partial file
        rm -f "$INPUT_FILE" 
    fi
done
