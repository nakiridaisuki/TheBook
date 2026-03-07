# 安裝 Rust

[The book 1.1](https://rust-lang.tw/book-tw/ch01-01-installation.html)

要安裝 Rust，官方建議使用 rustup 來管理 Rust 版本與相關工具，官方提供了安裝腳本，這個安裝腳本會自動安裝 rustup 工具並接著安裝最新的穩定版 Rust。

```shell
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Archlinux 的包管理器有提供 rustup，所以我使用 pacman 安裝，並且使用 rustup 安裝 Rust 穩定版。

```shell
sudo pacman -S rustup
rustup default stable
```

最後檢查是否有正確安裝 Rust

```shell
rustc --version
```
