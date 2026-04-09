# コードレビュー指摘事項

## 1. バグ (要修正)

### [重大] `src/slack.rs:46` — memo表示で cwd を使用

```rust
if let Some(memo) = &process.memo {
    blocks.push(generate_section_block(format!("*Memo*: `{}`", process.cwd)))
                                                                 ^^^^^^^^^^^
}
```

- `memo` を束縛しているのに `process.cwd` を表示している
- 正しくは `format!("*Memo*: `{}`", memo)`
- 未使用変数 `memo` のコンパイラ警告も出ているはず
- セミコロンも欠落している（式文の末尾）

---

## 2. Rust イディオム / Clippy 指摘 (推奨修正)

| 箇所 | 問題 | 修正 |
|------|------|------|
| `src/slack.rs:14` | `url: &String` | `url: &str` に変更 |
| `src/cmd/check.rs:25` | `all_terminated_pids: &Vec<u32>` | `&[u32]` に変更 |
| `src/cmd/check.rs:37` | `all_errors.len() > 0` | `!all_errors.is_empty()` |
| `src/cmd/check.rs:67`, `destination/remove.rs:55`, `process/remove.rs:47` | `.map(\|p\| p.clone())` | `.cloned()` |
| `src/cmd/destination/remove.rs:39` | `if let Some(_) = ...find(...)` | `.any()` を使用 |
| `src/cmd/destination/remove.rs:5` 等 | `const KEY_NAME: &'static str` | `&str` で十分（`const` では `'static` は暗黙） |
| `src/main.rs:25` 等 | `unreachable!("")` | メッセージを有意義にするか省略 |

---

## 3. エラーハンドリング

- **`Result<(), String>` を全体で使用**: 小規模CLIなら許容範囲だが、`anyhow::Result` にすれば `.context()` 等でエラーに文脈を付けやすくなる
- **`src/cmd/check.rs:49` — `unwrap()`**: `get_process_info(&pid).unwrap()` はロジック上安全だが、`ok_or_else(|| format!("..."))` に置き換える方が防御的
- **clap引数の `unwrap()`** (各サブコマンド): `required(true)` なので安全だが、`expect("required argument")` の方が意図が明確

---

## 4. 設計 / アーキテクチャ

### `check.rs` のループ内 `save()` — `src/cmd/check.rs:63-70`

- 終了プロセスごとにファイル書き込みが発生（N個終了 → N回のディスクI/O）
- 改善: ループ後にまとめて1回 save() する
- 注意: 途中で通知失敗した場合のリカバリ設計が必要

### `base/` モジュールの冗長性

- `base/mod.rs` → `base/cmd.rs` → `Cmd` トレイト1つだけ
- `src/cmd.rs` に直接置くか、`src/cmd/mod.rs` 内に定義すれば十分

### `process.rs` と `cmd/process/` の命名衝突

- `process.rs`(procfs層) と `cmd/process/`(サブコマンド) で "process" が曖昧
- procfs層は `procfs.rs` や `monitor.rs` にリネーム検討

### XDG Base Directory 非準拠 — `src/models/configure.rs:21`, `processes.rs:23`

- `home_dir().join(".config")` とハードコード
- `dirs::config_dir()` を使えば `$XDG_CONFIG_HOME` を尊重できる

### `slack-morphism` クレートの過剰依存

- 使用しているのは Block Kit の4型のみ (`SlackBlock`, `SlackSectionBlock`, `SlackBlockMarkDownText`, `SlackDividerBlock`)
- slack-morphism は非同期Slack APIクライアント全体を含む大きなクレート
- `serde_json::json!()` で手動構築すれば依存を大幅に削減可能

### `<!channel>` のハードコード — `src/slack.rs:37`

- 全通知で全員にメンションが飛ぶ
- destination ごとにメンション設定を持たせる等の検討余地

---

## 5. プラットフォーム互換性

- `/proc` ファイルシステムに依存するが、条件コンパイルガードがない
- macOS で `cargo build` は通るが実行時にエラー
- 最低限 `src/process.rs` に `#[cfg(not(target_os = "linux"))] compile_error!("...")` を追加すべき

---

## 6. テスト

- テストコードが一切存在しない（`#[cfg(test)]` モジュールなし、`tests/` ディレクトリなし）
- 特にテストすべき箇所:
  - `process.rs` の cmdline パース
  - `slack.rs` の `build_blocks`（上記バグの検出にもなる）
  - `models/` の TOML シリアライズ/デシリアライズ

---

## 7. その他

- **ログ出力なし**: `tracing` / `env_logger` 等の導入検討。cron運用時のトラブルシューティングに有用
- **ファイルロック機構なし**: 複数 `pesn` インスタンスの同時実行で設定ファイルが競合する可能性
- **`Vec::retain` 未使用**: `check.rs:63-68`, `destination/remove.rs:51-56` で新 Vec を作っているが、`retain()` でインプレース削除の方が効率的

---

## 優先度まとめ

1. **P0 (即修正)**: slack.rs の memo バグ
2. **P1 (早期修正)**: Clippy 指摘事項 (`cargo clippy` で検出可能なもの)
3. **P2 (改善推奨)**: check.rs のループ内 save、XDG準拠、compile_error ガード
4. **P3 (将来検討)**: テスト追加、エラー型改善、slack-morphism 依存削減、ログ導入
