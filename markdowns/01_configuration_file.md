# コマンドの設定ファイルについて
## 保存場所
```
~/.config/pesn/config.toml
```

## 形式
- destination
  - type: array
  - properties
    - name
      - type: string
      - required: true
      - detail: destinationの名前
    - url
      - type: string
      - required: true
      - detail: SlackのIncomingWebhookのURL
    - memo
      - type: string
      - required: false
      - detail: 備考

## Rust上の扱い
- Configure Structが欲しい
  - 中身はdestinationの配列
- Static関数としてLoadが欲しい
  - 設定ファイルを読み込んで返す
  - ファイルがなければ空配列のConfigureを返してほしい
- resolve_destination()関数が欲しい
  - 引数はname
  - 返値はdestination
  - なければNone
- Save()関数が欲しい
  - 設定ファイルを保存する
  - ディレクトリがなければ作成する