## Electron icon build


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
