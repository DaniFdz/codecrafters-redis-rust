#!/usr/bin/env bash

if [[ $1 = "-h" || $1 = "--help" ]]; then
  echo -e "Usage: $0 [--verbose|-v]"
  exit
fi

if [[ $1 = "--verbose" || $1 = "-v" ]]; then
  VERBOSE=1
else
  VERBOSE=0
fi

DIRNAME="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTS_DIR="$DIRNAME/."
START_SERVER="$DIRNAME/../spawn_redis_server.sh"

function stop_server {
  echo -e "Stopping the server..."
  kill $SERVER_PID &>/dev/null
}

function handle_CTRL_C {
  # shellcheck disable=SC2317
  stop_server
  # shellcheck disable=SC2317
  exit 1
}

# Start the server in background
function start_server {
  echo -e "Starting the server..."
  if [ "$VERBOSE" -eq 1 ]; then
    $1 &
  else
    $1 &>/dev/null &
  fi
  SERVER_PID=$!

  # shellcheck disable=SC2015
  netstat -an &>/dev/null && while [ "$(netstat -an | grep 6379 | grep -c LISTEN)" -eq 0 ] && [ "$#" -ne 0 ]; do
      sleep 1
    done || sleep 5

  echo -e "Server started with PID: $SERVER_PID\n"
}

function main {
  # Start the server
  start_server "$START_SERVER"
  # Run the tests
  ALL_PASSED=1
  for test in "$TESTS_DIR"/*.test.sh; do
    $test
    if ! $test; then
      ALL_PASSED=0
    fi
    echo -e
  done
}

trap handle_CTRL_C INT

main || stop_server && exit 1

stop_server
if [ "$ALL_PASSED" -eq 1 ]; then
  echo -e "All tests passed!"
  exit 0
else
  echo -e "Some tests failed!"
  exit 1
fi