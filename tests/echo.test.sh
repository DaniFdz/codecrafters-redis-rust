#!/usr/bin/env bash

TEST_NAME="TEST: HANDLE MULTIPLE CONNECTIONS"

COLOR_RED="\033[0;31m"
COLOR_GREEN="\033[0;32m"
COLOR_RESET="\033[0m"

# Test the server
REDIS_CLI="redis-cli"

OVERALL_STATUS=0

function test_echo() {
    RESPONSE=$($REDIS_CLI echo "$1")
		CORRECT=$(echo $RESPONSE | grep -c "$1")
    if [ "$CORRECT" -ne 1 ]; then
        echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET} echoing $1 - Expected '$1' got $RESPONSE"
        OVERALL_STATUS=1
    fi
}

PIDS=()
for i in {1..10}; do
    test_echo "$i" &
		PIDS+=($!)
done

# Wait for all the connections to finish
for pid in ${PIDS[*]}; do
		wait $pid
done

if [ "$OVERALL_STATUS" -eq 0 ]; then
    echo -e "$TEST_NAME - ${COLOR_GREEN}PASSED${COLOR_RESET}"
    exit 0
else
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET}"
    exit 1
fi
