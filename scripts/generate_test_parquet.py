#!/usr/bin/env python3
"""
テスト用のParquetファイルを生成するスクリプト
コマンドライン引数でテストパターンを指定可能
"""

import sys
import argparse
import os

def main():
    parser = argparse.ArgumentParser(
        description='Generate test Parquet files',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='''
Examples:
  python generate_test_parquet.py 1    # Generate pattern 1 (user data)
  python generate_test_parquet.py 2    # Generate pattern 2 (product inventory data)
  python generate_test_parquet.py 3    # Generate pattern 3 (date/time types)
        '''
    )
    parser.add_argument(
        'pattern',
        type=int,
        choices=[1, 2, 3],
        help='Test data pattern ID (1: user data, 2: product inventory data, 3: date/time types)'
    )
    
    args = parser.parse_args()
    
    # スクリプトのディレクトリを取得
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    # パターンに応じてモジュールをインポートして実行
    if args.pattern == 1:
        import parquet_test_data1
        parquet_test_data1.generate_test_files(script_dir)
    elif args.pattern == 2:
        import parquet_test_data2
        parquet_test_data2.generate_test_files(script_dir)
    elif args.pattern == 3:
        import parquet_test_data3
        parquet_test_data3.generate_test_files(script_dir)
    else:
        print(f"Error: Unknown pattern {args.pattern}")
        sys.exit(1)

if __name__ == '__main__':
    main()
