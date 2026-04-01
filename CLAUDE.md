# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイダンスを提供します。

## プロジェクト概要

PIDでプロセスを監視し、終了時にSlack通知を送信するCLIツール。Rust (edition 2024) で記述。

## ビルド・テストコマンド

- **ビルド:** `cargo build`
- **実行:** `cargo run`
- **全テスト:** `cargo test`
- **単一テスト:** `cargo test <test_name>`
- **チェック（コード生成なし）:** `cargo check`
- **リント:** `cargo clippy`
- **フォーマット:** `cargo fmt`

## アーキテクチャ

- `src/main.rs` — エントリポイント
- `src/process.rs` — Linux `/proc` ファイルシステムリーダー: `/proc/{pid}/cmdline` と `/proc/{pid}/cwd` を通じてPIDからコマンドラインと作業ディレクトリを解決する

本ツールはLinuxをターゲットとする（procfsに依存）。
