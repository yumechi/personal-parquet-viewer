# personal-parquet-viewer

WebブラウザでParquetファイルを閲覧するためのツール

## デモ

https://www.yumechi.work/personal-parquet-viewer/

## 特徴

- ブラウザ上でParquetファイルを直接閲覧
- サーバーにファイルをアップロードせず、クライアントサイドで処理
- WebAssembly (Rust) による高速なParquet処理
- Svelte + Tailwind CSSによる軽量なUI

## 技術スタック

- **フロントエンド**: SvelteKit + TypeScript
- **スタイリング**: Tailwind CSS
- **Parquet処理**: Rust + WebAssembly
- **ツール管理**: mise
- **Python環境**: uv

## セットアップ

### 1. 必要なツールのインストール

```bash
# miseでツールをインストール
mise install
```

### 2. WebAssemblyのビルド

```bash
cd wasm
wasm-pack build --target web
```

### 3. フロントエンドのセットアップ

```bash
cd frontend
npm install
```

## 開発サーバーの起動

```bash
cd frontend
npm run dev
```

ブラウザで http://localhost:5173 を開いてください。

## テストデータの生成

```bash
cd scripts
mise exec -- uv sync
mise exec -- uv run python generate_test_parquet.py
```

生成されたテストファイルは `scripts/test_data/` に配置されます。

## プロジェクト構成

```
personal-parquet-viewer/
├── frontend/          # Svelteアプリケーション
│   ├── src/
│   │   ├── lib/       # 共通コンポーネント・ユーティリティ
│   │   ├── routes/    # SvelteKitルート
│   │   └── app.css    # Tailwindスタイル
│   └── package.json
├── wasm/              # Rust WebAssemblyコード
│   ├── src/
│   │   └── lib.rs     # Parquet読み込みロジック
│   ├── Cargo.toml
│   └── pkg/           # ビルド済みWASMパッケージ
├── scripts/           # テストデータ生成スクリプト
│   ├── generate_test_parquet.py
│   ├── pyproject.toml
│   └── README.md
├── mise.toml          # ツールバージョン管理
├── CLAUDE.md          # Claude Code連携情報
└── README.md
```

## 使い方

1. 開発サーバーを起動
2. ブラウザでアプリケーションを開く
3. 「Click to upload」ボタンからParquetファイルを選択
4. テーブル形式でデータが表示されます

## ライセンス

MIT
