# 監視対象のプロセス一覧
## 保存場所
```
~/.config/pesn/processes.toml
```

## 形式
- process
  - type: array
  - properties
    - pid
      - type: u32
      - required: true
      - detail: 監視対象のプロセスのPID
    - cwd
      - type: string
      - required: true
      - detail: プロセスのカレントディレクトリ
    - command
      - type: string
      - required: true
      - detail: プロセスのコマンド
    - memo
      - type: string
      - required: false
      - detail: 備考

## Rust上の扱い
- AllProcesses Structが欲しい
  - 中身はprocessの配列
- Static関数としてLoad()が欲しい
  - ファイルを読み込んで返す
  - ファイルがなければ空配列
- get_all_pid()という関数が欲しい
  - pidの配列を返す
- get_process_info()という関数が欲しい
  - pidを引数で受け取り、processを返す
  - なければNone
- remove_process_info()という関数が欲しい
  - pidを引数で受け取り、配列から消す
- add_process_info()という関数が欲しい
  - pid, cwd, command, memoを引数で受け取る
  - 配列にprocessを追記する
- save関数が欲しい
  - このファイルを作成する
  - ディレクトリがなければ作成する