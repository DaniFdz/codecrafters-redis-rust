#!/usr/bin/env bash

TEST_NAME="TEST: PING"
TMP_OUTPUT="/tmp/output"

COLOR_RED="\033[0;31m"
COLOR_GREEN="\033[0;32m"
COLOR_RESET="\033[0m"

# Test the server
REDIS_CLI="redis-cli"
($REDIS_CLI ping | grep -q "PONG") &> $TMP_OUTPUT
STATUS=$?

# Check the result
if [ $STATUS -eq 0 ]; then
  echo -e "$TEST_NAME - ${COLOR_GREEN}PASSED${COLOR_RESET}"
  rm $TMP_OUTPUT
  exit 0
else
  echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET}"
  echo -e "\tProgram finished with status: $STATUS"
  echo -e "\tOutput: $(cat $TMP_OUTPUT)"
  rm $TMP_OUTPUT
  exit 1
fi
