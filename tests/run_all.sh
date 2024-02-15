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

DIRNAME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TESTS_DIR="$DIRNAME/."
START_SERVER="$DIRNAME/../spawn_redis_server.sh" 

function stop_server {
  echo -e "Stopping the server..."
  kill $SERVER_PID &>/dev/null
}

function handle_CTRL_C {
  stop_server
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

  while [ "$(netstat -an | grep 6379 | grep -c LISTEN)" -eq 0 ] && [ "$#" -ne 0 ]; do
    sleep 1
  done
  echo -e "Server started with PID: $SERVER_PID\n"
}

function main {
  # Start the server
  start_server "$START_SERVER"
  # Run the tests
  for test in "$TESTS_DIR"/*.test.sh; do
    $test
    echo -e
  done
}

trap handle_CTRL_C INT

main || stop_server

stop_server
