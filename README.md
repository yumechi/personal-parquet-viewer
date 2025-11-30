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

WASMをビルドするには、Rustとwasm-packが必要です。

#### 前提条件

- **Rust**: [rustup](https://rustup.rs/)でインストール
- **wasm-pack**: `cargo install wasm-pack`でインストール
- **wasm32-unknown-unknownターゲット**: `rustup target add wasm32-unknown-unknown`でインストール

#### ビルド手順

```bash
cd wasm
wasm-pack build --target web
```

ビルドが成功すると、`wasm/pkg/`ディレクトリに以下のファイルが生成されます：
- `parquet_viewer_wasm.js` - JavaScriptバインディング
- `parquet_viewer_wasm_bg.wasm` - WebAssemblyバイナリ
- `parquet_viewer_wasm.d.ts` - TypeScript型定義
- `package.json` - npmパッケージ情報

これらのファイルは、フロントエンドの`package.json`で`file:../wasm/pkg`として参照されています。

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
mise exec -- uv run python generate_test_parquet.py 1    # パターン1: ユーザーデータ
mise exec -- uv run python generate_test_parquet.py 2    # パターン2: 商品在庫データ（日本語カラム名、日付型含む）
mise exec -- uv run python generate_test_parquet.py 3    # パターン3: 様々な日付・時刻型を含むデータ
```

生成されたテストファイルは以下のディレクトリに配置されます：
- `scripts/test_data/` - パターン1のデータ
- `scripts/test_data2/` - パターン2のデータ
- `scripts/test_data3/` - パターン3のデータ

E2Eテストで使用するため、`small3.parquet`を`frontend/tests/fixtures/`にコピーしてください：

```bash
cp scripts/test_data3/small3.parquet frontend/tests/fixtures/
```

## テストの実行

### WASMのテスト

```bash
cd wasm
cargo test
```

実データを使った統合テストが実行されます。テストファイルは以下のディレクトリから読み込まれます：
- `scripts/test_data/`
- `scripts/test_data2/`
- `scripts/test_data3/`

### E2Eテスト

```bash
cd frontend
npm test
```

Playwrightを使ったE2Eテストが実行されます。テストファイルは`frontend/tests/fixtures/`から読み込まれます。

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
