#!/usr/bin/env python3
"""
テストデータパターン2: 商品在庫データ（日本語カラム名、null値含む）
"""

import pyarrow as pa
import pyarrow.parquet as pq
import random
from datetime import datetime, timedelta
import os

# 書き出し先ディレクトリ
OUTPUT_DIR = 'test_data2'

def get_output_dir(script_dir):
    """出力ディレクトリのパスを取得"""
    return os.path.join(script_dir, OUTPUT_DIR)

def generate_product_data(num_rows=100):
    """商品在庫データを生成（異なる構造）"""
    categories = ['Electronics', 'Clothing', 'Food', 'Books', 'Toys', 'Home', 'Sports']
    statuses = ['in_stock', 'out_of_stock', 'discontinued', 'pre_order']
    
    data = {
        # 商品ID（文字列ベース）
        '商品ID': [f'PROD-{i:06d}' for i in range(1, num_rows + 1)],
        
        # 商品名
        '商品名': [
            f'Product {chr(65 + (i % 26))}{i}' for i in range(1, num_rows + 1)
        ],
        
        # カテゴリ
        'カテゴリ': [random.choice(categories) for _ in range(num_rows)],
        
        # 価格（null値を含む）
        '価格': [
            round(random.uniform(10.0, 9999.99), 2) if random.random() > 0.1 else None
            for _ in range(num_rows)
        ],
        
        # 在庫数（整数、null値を含む）
        '在庫数': [
            random.randint(0, 1000) if random.random() > 0.15 else None
            for _ in range(num_rows)
        ],
        
        # ステータス
        'ステータス': [random.choice(statuses) for _ in range(num_rows)],
        
        # 評価（0.0-5.0、null値を含む）
        '評価': [
            round(random.uniform(0.0, 5.0), 1) if random.random() > 0.2 else None
            for _ in range(num_rows)
        ],
        
        # レビュー数
        'レビュー数': [random.randint(0, 10000) for _ in range(num_rows)],
        
        # 発売日（日付のみ）
        '発売日': [
            (datetime(2020, 1, 1) + timedelta(days=random.randint(0, 1825))).date()
            for _ in range(num_rows)
        ],
        
        # 更新日時（タイムスタンプ）
        '最終更新日時': [
            datetime.now() - timedelta(hours=random.randint(0, 8760))
            for _ in range(num_rows)
        ],
        
        # 割引率（パーセンテージ、null値を含む）
        '割引率': [
            round(random.uniform(0.0, 50.0), 1) if random.random() > 0.3 else None
            for _ in range(num_rows)
        ],
        
        # 在庫警告フラグ
        '在庫警告': [
            random.choice([True, False]) for _ in range(num_rows)
        ],
        
        # タグ（配列として扱う文字列、カンマ区切りで保存）
        'タグ': [
            ','.join(random.sample(['new', 'sale', 'popular', 'limited', 'eco-friendly', 'premium'], 
                                  k=random.randint(0, 3)))
            for _ in range(num_rows)
        ],
    }
    return data

def create_parquet_file(output_path, num_rows=100):
    """Parquetファイルを作成"""
    data = generate_product_data(num_rows)
    
    # スキーマを明示的に定義（null値を許可）
    schema = pa.schema([
        ('商品ID', pa.string()),
        ('商品名', pa.string()),
        ('カテゴリ', pa.string()),
        ('価格', pa.float64()),
        ('在庫数', pa.int64()),
        ('ステータス', pa.string()),
        ('評価', pa.float64()),
        ('レビュー数', pa.int64()),
        ('発売日', pa.date32()),
        ('最終更新日時', pa.timestamp('us')),
        ('割引率', pa.float64()),
        ('在庫警告', pa.bool_()),
        ('タグ', pa.string()),
    ])
    
    table = pa.table(data, schema=schema)
    pq.write_table(table, output_path)
    print(f"Created: {output_path} ({num_rows} rows)")

def generate_test_files(script_dir):
    """テストファイルを生成"""
    output_dir = get_output_dir(script_dir)
    
    # 出力ディレクトリを作成
    os.makedirs(output_dir, exist_ok=True)
    
    # 様々なサイズのテストファイルを生成（異なるファイル名）
    test_files = [
        ('small2.parquet', 10),
        ('medium2.parquet', 100),
        ('large2.parquet', 1000),
        ('xlarge2.parquet', 10000),
    ]
    
    for filename, num_rows in test_files:
        output_path = os.path.join(output_dir, filename)
        create_parquet_file(output_path, num_rows)
    
    print(f"\nAll test files created in: {output_dir}")

