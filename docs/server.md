# Server

## Docker

Use docker to deploy

```sh
docker run -dit --name kimika -p 3941:3941 yixiaojiu1/kimika-server-ts:latest
```

## Source

deploy from source code

```sh
git clone https://github.com/yixiaojiu/kimika

cd kimika-server-ts

pnpm install

pnpm run start
```
