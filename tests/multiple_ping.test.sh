#!/usr/bin/env bash

TEST_NAME="TEST: MULTIPLE PING COMMANDS"

COLOR_RED="\033[0;31m"
COLOR_GREEN="\033[0;32m"
COLOR_RESET="\033[0m"

# Test the server
REDIS_CLI="redis-cli"
STATUS=$(echo -e "ping\nping" | $REDIS_CLI | grep -c "PONG")

# Check the result
if [ "$STATUS" -eq 2 ]; then
  echo -e "$TEST_NAME - ${COLOR_GREEN}PASSED${COLOR_RESET}"
  exit 0
else
  echo -e "$TEST_NAME - ${COLOR_RED}FAILED${COLOR_RESET}"
  echo -e "\tProgram received $STATUS instead of 2 PONG responses."
  exit 1
fi

# Clean up