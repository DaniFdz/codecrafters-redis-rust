run:
    bash spawn_redis_server.sh

test:
    bash tests/run_all.sh

test_verbose:
    bash tests/run_all.sh --verbose

push_step:
    git push origin master