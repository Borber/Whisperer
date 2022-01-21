## CLI

使用`clap v3` `打包 `Whisperer` `lib` 中的方法 实现的命令行程序

#### 使用方法

```shell
// 加密
./whisperer.exe 123
低语:央慈奉资
// 解密
./whisperer.exe -d 低语:央慈奉资
123
// 字典校验及排序
./whisperer.exe --check-dict
验证通过!
```

字典校验及排序, 检验的是 `config.toml` 中的 `dict` 数组

感谢以下项目:

[clap](https://github.com/clap-rs/clap)