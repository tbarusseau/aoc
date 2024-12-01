#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage:"
    echo "$0 YEAR [-f]"
    exit 1
fi

YEAR=$1
FORCE=0

if [[ ! -z "$2" && $2 = "-f" ]]; then
    FORCE=1
fi

echo "Creating day files"
for I in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25; do
    mkdir -p "src/y${YEAR}"
    DEST_FILE="src/y${YEAR}/day${I}.rs"

    if [[ ! -f "${DEST_FILE}" || ${FORCE} -eq 1 ]]; then
        cp src/day_template.rs "${DEST_FILE}"
        gsed -i "s/NotDone/${I}/g" "${DEST_FILE}"
    else
        echo "Skipping file: ${DEST_FILE}"
    fi
done

MOD_FILE="src/y${YEAR}/mod.rs"
if [[ ! -f "${MOD_FILE}" || ${FORCE} -eq 1 ]]; then
    echo "Creating mod file"
    echo "crate::days_gen!();" >"src/y${YEAR}/mod.rs"
else
    echo "Skipping file: ${MOD_FILE}"
fi

