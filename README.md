Learning Rust and Docker by rewriting my site to use them.

## Building Image

```
docker build -t abenga_site -f ./build/images/abenga_site/Dockerfile .
```

## Running Image

```
docker run -e "ROCKET_ADDRESS=0.0.0.0" -e "DATABASE_ENV_NAME=local" -e "SITE_RUN_TYPE=DOCKER" --network host abenga_site
```


