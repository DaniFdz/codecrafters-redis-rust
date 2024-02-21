#!/usr/bin/env bash

TEST_NAME="TEST: SET/GET"

COLOR_RED="\033[0;31m"
COLOR_GREEN="\033[0;32m"
COLOR_RESET="\033[0m"

# Test the server
REDIS_CLI="redis-cli"


OVERALL_STATUS=0
response=$($REDIS_CLI set "test_set" "test")
if [ "$response" != "OK" ]; then
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET} setting key 'test_set' - Expected 'OK' got $response"
    OVERALL_STATUS=1
fi
response=$($REDIS_CLI get "test_set")
if [ "$response" != "test" ]; then
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET} getting key 'test_set' - Expected 'test' got $response"
    OVERALL_STATUS=1
fi
response=$($REDIS_CLI get "test_set_2")
if [ "$response" != "" ]; then
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET} getting key 'test_set_2' - Expected '(nil)' got $response"
    OVERALL_STATUS=1
fi

if [ "$OVERALL_STATUS" -eq 0 ]; then
    echo -e "$TEST_NAME - ${COLOR_GREEN}PASSED${COLOR_RESET}"
    exit 0
else
    echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET}"
    exit 1
fi
