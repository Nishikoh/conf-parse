# conf-parse

## 実行
```sh
docker-compose up
```
```sh
docker build -t app --target production .
docker run app
# docker run app test.conf 実行時にファイル指定できる
```
## 開発
```sh
# 引数で読み取るファイルを指定
cargo run test.conf
```