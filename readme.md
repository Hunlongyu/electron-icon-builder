## Electron icon build

先 生成 不同尺寸的 png
再把 512 png 转成 icons
然后把 256 png 转成 ico

最后根据 flatten 放到不同的目录下

https://github.com/Hunlongyu/eib/blob/main/src/commands/run.ts

png2icns
https://github.com/mdsteele/rust-icns/blob/master/examples/png2icns.rs

### RUST BUILD 
以下配置，可以减少二进制文件包的大小
```toml
[profile.release]
panic = "abort"
lto = true
codegen-units = 1
incremental = false
opt-level = "z"
```
