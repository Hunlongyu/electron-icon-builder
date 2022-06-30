## Electron icon build

先 生成 不同尺寸的 png
再把 512 png 转成 icons
然后把 256 png 转成 ico

最后根据 flatten 放到不同的目录下

https://github.com/Hunlongyu/eib/blob/main/src/commands/run.ts

png2icns
https://github.com/mdsteele/rust-icns/blob/master/examples/png2icns.rs