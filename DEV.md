## publish

Cargo.tomlのバージョンを上げてマージ→GitHubでリリースを切る→リリースをトリガーとしたGitHub Actionsでpublish(バージョンはCargo.tomlが見られる)

## ドキュメント生成のdev

```
cargo doc --open
```

## コードの問題確認

```
cd _example/helloworld
cargo build --release
```

でコンパイルエラーにならないかを確認
