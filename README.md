# Rust勉強用

- VSCodeで使う場合は、ルートディレクトリにCargo.tomlがくるディレクトリ配置にする必要がある


```
# 更新
rustup update

# コンパイル
rustc ファイル名
```

## cargoコマンド

```
# プログラム作成
cargo new プロジェクト名

# ビルド
cargo build

# Clean
cargo clean

# 実行
cargo run
```


# 構文解説

- String型は+演算子での連結が可能
- 変数に付く&が借用の印