#!/bin/bash
SPLITTER="/"

# declare associative array
declare -A labels=(
    ["Area: Application Protocol"]="solarxr"
    ["Area: Firmware"]="firmware"
    ["Area: Overlay"]="overlay"
    ["Area: Skeletal Model"]="skeletal_model"
)

prefixes=()


# read each line of stdin
# save it as variable label
while read -r label
do
    #index labels
    prefixes+=( "${labels[$label]}" )
done

# IFS is the delimiter for joining each value of the array
echo "$(IFS=$SPLITTER ; echo "${prefixes[*]}"): "
