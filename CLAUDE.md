# CLAUDE.md

このプロジェクトにおける Claude Code との連携に関する情報を記載します。

## プロジェクト概要

personal-parquet-viewer: Parquetファイルを閲覧するためのWebアプリケーション

### 目的

WebブラウザでParquetファイルを直接閲覧できるツールを提供する。

### 主要機能

- Parquetファイルのアップロード（クライアント側のみで処理、サーバーに保存しない）
- アップロードしたファイルをテーブル形式で表示
- 大容量ファイルの処理に対応

### 技術スタック

- **フロントエンドフレームワーク**: Svelte（軽量・高速）
- **スタイリング**: Tailwind CSS（軽量なデザイン）
- **Parquet処理**: WebAssembly（大容量ファイル対応）
- **開発言語**:
  - フロントエンド: JavaScript/TypeScript
  - WebAssembly: Rust

### アーキテクチャ方針

- **クライアントサイド処理**: アップロードされたファイルはブラウザ内で処理し、サーバーには送信しない
- **セキュリティ**: ファイルはメモリ上でのみ扱い、永続化しない
- **パフォーマンス**: WebAssemblyを活用して高速な処理を実現

## 開発環境

### ツールバージョン管理

このプロジェクトでは `mise` を使用してツールのバージョンを管理しています。

設定ファイル: `mise.toml`

管理対象:
- Node.js: v24
- Python: 3.12
- uv: latest

**注意**: Rustはローカルにインストールされているバージョンを使用するため、miseでは管理していません。

### セットアップ

```bash
# mise がインストールされていることを確認
mise --version

# ツールのインストール
mise install

# インストールされたバージョンの確認
mise current
```

## コミュニケーション

- このプロジェクトでは日本語でコミュニケーションを行います
- Claude Code との対話も日本語で行います

## プロジェクト構成

```
personal-parquet-viewer/
├── frontend/          # Svelteアプリケーション
│   ├── src/
│   │   ├── lib/       # 共通コンポーネント・ユーティリティ
│   │   ├── routes/    # SvelteKitルート
│   │   └── wasm/      # WebAssemblyバインディング
│   └── package.json
├── wasm/              # Rust WebAssemblyコード
│   ├── src/
│   └── Cargo.toml
├── scripts/           # テストデータ生成スクリプトなど
├── mise.toml
├── CLAUDE.md
└── README.md
```

## 開発ガイドライン

### 開発フロー

1. Parquetファイル処理ロジックをRustで実装
2. WebAssemblyにコンパイル
3. SvelteアプリケーションからWASMを呼び出し
4. Tailwind CSSでUIをスタイリング

### テストデータ

- `scripts/` ディレクトリにテスト用Parquetファイル生成スクリプトを配置
- 様々なデータサイズ・スキーマのサンプルを生成可能にする

### 今後の実装予定

- [ ] Svelteプロジェクトの初期化
- [ ] Tailwind CSSのセットアップ
- [ ] Rust WebAssemblyプロジェクトの初期化
- [ ] Parquet読み込み機能の実装
- [ ] ファイルアップロードUIの実装
- [ ] テーブル表示コンポーネントの実装
- [ ] テストデータ生成スクリプトの作成
