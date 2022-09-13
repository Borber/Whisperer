## Server

`Whisperer` 的 服务器版本, 使用 `poem` 库, 提供 `api`.

运行 `server` 程序, 根据 `server.toml`配置, 将监听对应地址端口, 项目样例中为 `127.0.0.1:3000`, 加密解密`api`地址也可在 `server.toml`中配置.

### 加密

`post` 请求 `localhost:3000/v1/api/e`

```json
{
  "s": "123"
}
```

响应:

```json
{
  "s": "低语:央慈奉资"
}
```

### 解密

`post`请求 `localhost:3000/v1/api/d`

```json
{
  "s": "低语:央慈奉资"
}
```

响应:

```json
{
  "s": "123"
}
```

## 提供免费api以供测试:

`https://whisperer.run-eu-central1.goorm.io/v1/api`


感谢以下项目:

[poem](https://github.com/poem-web/poem)