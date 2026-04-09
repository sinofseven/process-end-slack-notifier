# process-end-slack-notifier (`pesn`)

PIDでプロセスを監視し、終了時にSlack通知を送るCLIツール。

## 動作要件

- Linux（`/proc` ファイルシステムに依存）
- Slack Incoming Webhook URL

## インストール

```bash
cargo install --path .
```

## 使い方

### 基本的な流れ

1. Slack通知先（destination）を登録する
2. 監視したいプロセスのPIDを登録する
3. `pesn check` を定期実行する（cronなど）

### 1. 通知先（destination）の管理

#### 通知先を追加する

```bash
pesn destination add --name <name> --url <webhook-url> [--memo <memo>]
```

| オプション | 短縮形 | 必須 | 説明 |
|-----------|--------|------|------|
| `--name`  | `-n`   | Yes  | 通知先の名前（識別子） |
| `--url`   | `-u`   | Yes  | Slack Incoming Webhook URL |
| `--memo`  | `-m`   | No   | メモ |

**例:**

```bash
pesn destination add -n my-channel -u https://hooks.slack.com/services/T.../B.../xxx
```

#### 通知先の一覧を表示する

```bash
pesn destination list
```

#### 通知先の詳細を表示する

```bash
pesn destination show --name <name>
```

#### 通知先を削除する

```bash
pesn destination remove --name <name>
```

> 監視中のプロセスに使用されている通知先は削除できません。

---

### 2. 監視プロセスの管理

#### プロセスを監視対象に追加する

```bash
pesn process add --pid <pid> --destination <name> [--memo <memo>]
```

| オプション      | 短縮形 | 必須 | 説明 |
|----------------|--------|------|------|
| `--pid`        | `-p`   | Yes  | 監視するプロセスのPID |
| `--destination`| `-d`   | Yes  | 通知先の名前（登録済みのもの） |
| `--memo`       | `-m`   | No   | メモ |

登録時に `/proc/{pid}/cmdline` と `/proc/{pid}/cwd` からコマンドラインと作業ディレクトリを自動取得し、保存します。

**例:**

```bash
# ML学習ジョブを監視する
pesn process add -p 12345 -d my-channel -m "ML training job"
```

#### 監視中のプロセス一覧を表示する

```bash
pesn process list
```

#### 監視中プロセスの詳細を表示する

```bash
pesn process show --pid <pid>
```

#### 監視対象からプロセスを削除する

```bash
pesn process remove --pid <pid>
```

---

### 3. 終了チェックを実行する

```bash
pesn check
```

監視中の全プロセスを確認し、終了済みのプロセスに対してSlack通知を送信します。通知済みプロセスは監視リストから自動削除されます。

#### cronでの定期実行例

1分ごとにチェックする場合:

```cron
* * * * * /path/to/pesn check
```

---

## Slack通知の内容

プロセス終了が検知されると、以下の情報を含む通知が送信されます。

- `<!channel>` メンション + 終了検知時刻（JST）
- **PID** — プロセスID
- **Command** — 登録時に取得したコマンドライン
- **Cwd** — 登録時に取得した作業ディレクトリ
- **Memo** — メモ（設定した場合のみ）

---

## 設定ファイル

設定は以下のTOMLファイルに自動保存されます。初回実行時に自動作成されます。

| ファイル | 内容 |
|---------|------|
| `~/.config/pesn/config.toml` | 通知先（destination）の定義 |
| `~/.config/pesn/processes.toml` | 監視対象プロセスの一覧 |

### `~/.config/pesn/config.toml` の例

```toml
[[destination]]
name = "my-channel"
url = "https://hooks.slack.com/services/T.../B.../xxx"
memo = "optional memo"
```

### `~/.config/pesn/processes.toml` の例

```toml
[[process]]
pid = 12345
cwd = "/home/user/project"
command = "python train.py --epochs 100"
destination = "my-channel"
memo = "ML training job"
```

---

## ライセンス

MIT
