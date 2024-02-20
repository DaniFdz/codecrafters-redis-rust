run:
    bash spawn_redis_server.sh

watch:
    cargo watch -q -c -w src/ -x "run -q -r --target-dir=/tmp/codecrafters-redis-target --manifest-path '$(dirname '$0')'/Cargo.toml -- '$@'"

test:
    cargo test
    bash tests/run_all.sh

test_watch:
    cargo watch -q -c -w src/ -w tests/ -x "test; bash tests/run_all.sh"

test_verbose:
    cargo test --verbose
    bash tests/run_all.sh --verbose

push_step:
    git push origin master