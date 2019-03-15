# Cross Compiling

This project is currently cross-compiled for Windows 10 using `mingw`.
See [this blog post](https://crankydev.wordpress.com/2016/11/13/rust-cross-compiling-in-linux-for-windows/)
for more details. Set target to windows with

```bash
 cargo build --target=x86_64-pc-windows-gnu
```
