run:
    bash spawn_redis_server.sh

test:
    bash tests/run_all.sh

push_step:
    git push origin master