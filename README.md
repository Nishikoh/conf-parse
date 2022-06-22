# conf-parse
conf(ini)ファイルを読み取ってデータ型に格納する

## 実行
docker-compose, docker, cargoそれぞれの実行方法は以下の通り
```sh
docker-compose up
```
```sh
docker build -t app --target production .
docker run app
# docker run app test.conf 実行時にファイル指定できる
```
```sh
cargo run
# cargo run test.conf 引数で読み取るファイルを指定する
```