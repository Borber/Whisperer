# whisperer-serverless-vercel

`Whisperer` 的 [vercel](https://vercel.com/) 无服务器函数版本, 感谢 `Vercel` 的免费额度

## DemoAPI

### 加密

`path`: `https://whisperer-serverless-vercel.vercel.app/api/v1/encode`

请求样例:

```json
{
  "s": "测试文本"
}
```

返回样例:

```json
{
  "code": 0,
  "result": "低语:住逝告朋术教遮爱西罗术粟族"
}
```

![e](https://i.pstorage.space/i/k8oDwK2vK/original_e.png)

### 解密

`path`: `https://whisperer-serverless-vercel.vercel.app/api/v1/decode`

请求样例:

```json
{
  "s": "低语:住逝告朋术教遮爱西罗术粟族"
}
```

返回样例:

```json
{
  "code": 0,
  "result": "测试文本"
}
```

![d](https://i.pstorage.space/i/abQMmDWxe/original_d.png)

感谢以下项目:

[vercel-community/rust](https://github.com/vercel-community/rust)

