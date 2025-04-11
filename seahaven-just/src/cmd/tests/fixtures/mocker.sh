#!/bin/env bash
# This script prints each argument on a new line.
# The output of the CLI can be tested against.

# If no arguments were passed, just print an special string, and exit
if [ $# -eq 0 ]; then
    echo "<no-args>"
    exit 0
fi

# Print each argument on a new line in the order they were received
printf "%s\n" "$@"
