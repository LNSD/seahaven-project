#!/bin/env bash
# This script sorts all arguments alphabetically and prints each on a new line.
# This way, the output of the CLI is deterministic and can be tested against.

# If no arguments were passed, just print an special string, and exit
if [ $# -eq 0 ]; then
    echo "<no-args>"
    exit 0
fi

# Using `LC_ALL=C` enforces traditional ASCII sorting order where hyphens come before letters
printf "%s\n" "$@" | LC_ALL=C sort
