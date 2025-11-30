#!/usr/bin/env python3
"""
テストデータパターン3: 様々な日付・時刻型を含むデータ
"""

import pyarrow as pa
import pyarrow.parquet as pq
import random
from datetime import datetime, timedelta, time
import os

# 書き出し先ディレクトリ
OUTPUT_DIR = 'test_data3'

def get_output_dir(script_dir):
    """出力ディレクトリのパスを取得"""
    return os.path.join(script_dir, OUTPUT_DIR)

def generate_date_time_data(num_rows=100):
    """様々な日付・時刻型を含むデータを生成"""
    base_date = datetime(2020, 1, 1)
    
    data = {
        # ID
        'ID': [i for i in range(1, num_rows + 1)],
        
        # Date32型（日付のみ）
        '発売日_Date32': [
            (base_date + timedelta(days=random.randint(0, 1825))).date()
            for _ in range(num_rows)
        ],
        
        # Date64型（日付のみ、ミリ秒単位）
        '登録日_Date64': [
            (base_date + timedelta(days=random.randint(0, 1825))).date()
            for _ in range(num_rows)
        ],
        
        # Timestamp型（様々な単位）
        '作成日時_Timestamp秒': [
            base_date + timedelta(seconds=random.randint(0, 157680000))
            for _ in range(num_rows)
        ],
        '更新日時_Timestampミリ秒': [
            base_date + timedelta(milliseconds=random.randint(0, 157680000000))
            for _ in range(num_rows)
        ],
        '処理日時_Timestampマイクロ秒': [
            base_date + timedelta(microseconds=random.randint(0, 157680000000000))
            for _ in range(num_rows)
        ],
        'ログ日時_Timestampナノ秒': [
            base_date + timedelta(microseconds=random.randint(0, 157680000000000))
            for _ in range(num_rows)
        ],
        
        # Time32型（時刻のみ）
        '開始時刻_Time32秒': [
            time(
                hour=random.randint(0, 23),
                minute=random.randint(0, 59),
                second=random.randint(0, 59)
            )
            for _ in range(num_rows)
        ],
        '終了時刻_Time32ミリ秒': [
            time(
                hour=random.randint(0, 23),
                minute=random.randint(0, 59),
                second=random.randint(0, 59),
                microsecond=random.randint(0, 999999)
            )
            for _ in range(num_rows)
        ],
        
        # Time64型（時刻のみ）
        '計測時刻_Time64マイクロ秒': [
            time(
                hour=random.randint(0, 23),
                minute=random.randint(0, 59),
                second=random.randint(0, 59),
                microsecond=random.randint(0, 999999)
            )
            for _ in range(num_rows)
        ],
        '精密時刻_Time64ナノ秒': [
            time(
                hour=random.randint(0, 23),
                minute=random.randint(0, 59),
                second=random.randint(0, 59),
                microsecond=random.randint(0, 999999)
            )
            for _ in range(num_rows)
        ],
        
        # 説明
        '説明': [f'サンプルデータ{i}' for i in range(1, num_rows + 1)],
    }
    return data

def create_parquet_file(output_path, num_rows=100):
    """Parquetファイルを作成"""
    data = generate_date_time_data(num_rows)
    
    # スキーマを明示的に定義
    schema = pa.schema([
        ('ID', pa.int64()),
        ('発売日_Date32', pa.date32()),
        ('登録日_Date64', pa.date64()),
        ('作成日時_Timestamp秒', pa.timestamp('s')),
        ('更新日時_Timestampミリ秒', pa.timestamp('ms')),
        ('処理日時_Timestampマイクロ秒', pa.timestamp('us')),
        ('ログ日時_Timestampナノ秒', pa.timestamp('ns')),
        ('開始時刻_Time32秒', pa.time32('s')),
        ('終了時刻_Time32ミリ秒', pa.time32('ms')),
        ('計測時刻_Time64マイクロ秒', pa.time64('us')),
        ('精密時刻_Time64ナノ秒', pa.time64('ns')),
        ('説明', pa.string()),
    ])
    
    table = pa.table(data, schema=schema)
    pq.write_table(table, output_path)
    print(f"Created: {output_path} ({num_rows} rows)")

def generate_test_files(script_dir):
    """テストファイルを生成"""
    output_dir = get_output_dir(script_dir)
    
    # 出力ディレクトリを作成
    os.makedirs(output_dir, exist_ok=True)
    
    # 様々なサイズのテストファイルを生成
    test_files = [
        ('small3.parquet', 10),
        ('medium3.parquet', 100),
    ]
    
    for filename, num_rows in test_files:
        output_path = os.path.join(output_dir, filename)
        create_parquet_file(output_path, num_rows)
    
    print(f"\nAll test files created in: {output_dir}")

if __name__ == '__main__':
    import sys
    script_dir = os.path.dirname(os.path.abspath(__file__))
    generate_test_files(script_dir)

