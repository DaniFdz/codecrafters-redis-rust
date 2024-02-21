#!/usr/bin/env bash

TEST_NAME="TEST: EXPIRY"

COLOR_RED="\033[0;31m"
COLOR_GREEN="\033[0;32m"
COLOR_RESET="\033[0m"

# Test the server
REDIS_CLI="redis-cli"


OVERALL_STATUS=0
response=$($REDIS_CLI set "test_ex" "test" px 100)
if [ "$response" != "OK" ]; then
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET} setting key 'test_ex' - Expected 'OK' got $response"
    OVERALL_STATUS=1
fi
sleep 1
response=$($REDIS_CLI get "test_ex")
if [ "$response" != "" ]; then
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET} getting key 'test_ex' - Expected '(nil)' got $response"
    OVERALL_STATUS=1
fi

if [ "$OVERALL_STATUS" -eq 0 ]; then
    echo -e "$TEST_NAME - ${COLOR_GREEN}PASSED${COLOR_RESET}"
    exit 0
else
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET}"
    exit 1
fi
