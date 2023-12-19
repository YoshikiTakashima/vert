#!/bin/zsh
#
# This script is meant to be run in the directories where the files are present.

CRASH_DIR="./proptest-regressions"
LCOV_FILE="coverage.lcov"
PARSE_SCRIPT="$(git rev-parse --show-toplevel)/summarize-lcov-stdout.awk"

# filter only for ones where no crash exists.
if [ ! -d "$CRASH_DIR" ] 
then
	lcov --summary $LCOV_FILE | $PARSE_SCRIPT
fi
