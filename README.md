# e1skkserv

- 自分(emanon001)のためのskkserv
- Rustによるskkservのサンプル実装

## インストール方法

```
cargo install --git https://github.com/emanon001/e1skkserv
```

## 使い方

### skkservの起動

```
e1skkserv
```

オプションは `e1skkserv -h` で確認。

## 変換ルール

### `ぼく` → `emanon001`

```
echo -n '1ぼく ' | iconv -f 'UTF-8' -t 'EUC-JP' | nc  localhost 1178
1/emanon001/
```
