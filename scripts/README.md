# テストデータ生成スクリプト

## セットアップ

このプロジェクトでは `uv` を使用してPython依存関係を管理します。
`mise` を使用してツールをインストールしてください。

```bash
# プロジェクトルートで
mise install

# scriptsディレクトリで依存関係をインストール
cd scripts
uv sync
```

## 使用方法

```bash
# scriptsディレクトリで
uv run python generate_test_parquet.py

# または、定義済みのコマンドを使用
uv run generate-test-data
```

このスクリプトは、`test_data/` ディレクトリに以下のテストファイルを生成します：

- `small.parquet` - 10行
- `medium.parquet` - 100行
- `large.parquet` - 1,000行
- `xlarge.parquet` - 10,000行

## データスキーマ

各ファイルには以下のカラムが含まれます：

- `id` (int64): ユーザーID
- `name` (string): ユーザー名
- `age` (int64): 年齢
- `score` (float64): スコア
- `active` (bool): アクティブフラグ
- `created_at` (string): 作成日時
