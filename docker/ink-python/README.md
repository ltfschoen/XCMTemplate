* Run from parent directory:
```
DOCKER_BUILDKIT=0 docker compose -f ./docker/ink-python/docker-compose.yml up --build -d
docker exec -it ink-python /bin/bash
cd src
python app.py
```

* References:
    * https://github.com/polkascan/py-substrate-interface
