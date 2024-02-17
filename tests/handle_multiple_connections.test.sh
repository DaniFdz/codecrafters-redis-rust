#!/usr/bin/env bash

TEST_NAME="TEST: HANDLE MULTIPLE CONNECTIONS"

COLOR_RED="\033[0;31m"
COLOR_GREEN="\033[0;32m"
COLOR_RESET="\033[0m"

# Test the server
REDIS_CLI="redis-cli"

OVERALL_STATUS=0

function test_connection() {
    STATUS=$($REDIS_CLI ping | grep -c "PONG")
    if [ "$STATUS" -ne 1 ]; then
        echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET} on connection $1 - Expected 1 PONG, got $STATUS PONG"
        OVERALL_STATUS=1
    fi
}
# Create multiple connections
PIDS=()
for i in {1..10}; do
    test_connection "$i" &
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
