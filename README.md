# Rgewe API

对[Gewechat](https://github.com/Devo919/Gewechat?tab=readme-ov-file) RESTFul API的 Rust 语言二次封装。附有较为完整的 API 文档。

## 拉取 Gewe 镜像

使用 `docker compose`

```shell
docker-compose up -d
```

或者单独拉取运行

```shell
docker pull docker build -t ghcr.io/momo402/rgewe:1.0
docker run -d --name rgewe --restart always -p 2531:2531 -p 2532:2532 ghcr.io/momo402/rgewe:1.0
```

## 测试

## 镜像测试

```shell
docker exec -it rgewe /bin/sh
```

进入容器后，查看`gewe`日志。

```txt
~ # cat gewe/api/gewe.log  | grep GeWe
```

最下面出现`(♥◠‿◠)ﾉﾞ  GeWe启动成功   ლ(´ڡ`ლ)ﾞ`即为成功。

### API 测试

初始`token` 为空，配置在`src/util/mod.rs`中。

### API 文档

原始的 [Gewe API 文档](https://apifox.com/apidoc/shared-69ba62ca-cb7d-437e-85e4-6f3d3df271b1)，Rust 二次封装的 API 可直接通过 Rust Doc 查看，修复了Java API的一些typo。

### LICNSE

Apache-2.0
