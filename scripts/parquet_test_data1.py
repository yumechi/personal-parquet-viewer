#!/usr/bin/env python3
"""
テストデータパターン1: ユーザーデータ
"""

import pyarrow as pa
import pyarrow.parquet as pq
import random
from datetime import datetime, timedelta
import os

# 書き出し先ディレクトリ
OUTPUT_DIR = 'test_data'

def get_output_dir(script_dir):
    """出力ディレクトリのパスを取得"""
    return os.path.join(script_dir, OUTPUT_DIR)

def generate_sample_data(num_rows=100):
    """サンプルデータを生成"""
    data = {
        'id': list(range(1, num_rows + 1)),
        'name': [f'User_{i}' for i in range(1, num_rows + 1)],
        'age': [random.randint(18, 80) for _ in range(num_rows)],
        'score': [round(random.uniform(0, 100), 2) for _ in range(num_rows)],
        'active': [random.choice([True, False]) for _ in range(num_rows)],
        'created_at': [
            (datetime.now() - timedelta(days=random.randint(0, 365))).isoformat()
            for _ in range(num_rows)
        ],
    }
    return data

def create_parquet_file(output_path, num_rows=100):
    """Parquetファイルを作成"""
    data = generate_sample_data(num_rows)
    table = pa.table(data)
    pq.write_table(table, output_path)
    print(f"Created: {output_path} ({num_rows} rows)")

def generate_test_files(script_dir):
    """テストファイルを生成"""
    output_dir = get_output_dir(script_dir)
    
    # 出力ディレクトリを作成
    os.makedirs(output_dir, exist_ok=True)
    
    # 様々なサイズのテストファイルを生成
    test_files = [
        ('small.parquet', 10),
        ('medium.parquet', 100),
        ('large.parquet', 1000),
        ('xlarge.parquet', 10000),
    ]
    
    for filename, num_rows in test_files:
        output_path = os.path.join(output_dir, filename)
        create_parquet_file(output_path, num_rows)
    
    print(f"\nAll test files created in: {output_dir}")

