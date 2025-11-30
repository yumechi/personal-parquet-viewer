use parquet_viewer_wasm::read_parquet_internal;
use std::fs;

fn read_test_file(filename: &str) -> Vec<u8> {
    // テストファイルのパスを構築
    // プロジェクトルートからの相対パス
    let test_file_path = format!("../scripts/test_data/{}", filename);
    fs::read(&test_file_path)
        .unwrap_or_else(|_| {
            // test_data2も試す
            let test_file_path2 = format!("../scripts/test_data2/{}", filename);
            fs::read(&test_file_path2).unwrap_or_else(|_| {
                // test_data3も試す
                let test_file_path3 = format!("../scripts/test_data3/{}", filename);
                fs::read(&test_file_path3)
                    .unwrap_or_else(|_| {
                        panic!("Failed to read test file: {}. Tried: {}, {}, {}", 
                               filename, test_file_path, test_file_path2, test_file_path3)
                    })
            })
        })
}

#[test]
fn test_read_small_parquet() {
    let data = read_test_file("small.parquet");
    let result = read_parquet_internal(&data);
    assert!(result.is_ok(), "Failed to read small.parquet: {:?}", result.err());
    
    let parquet_data = result.unwrap();
    assert!(parquet_data.total_rows > 0, "Expected rows but got 0");
    assert!(!parquet_data.columns.is_empty(), "Expected columns but got none");
    assert_eq!(parquet_data.rows.len(), parquet_data.total_rows);
}

#[test]
fn test_read_small2_parquet() {
    let data = read_test_file("small2.parquet");
    let result = read_parquet_internal(&data);
    assert!(result.is_ok(), "Failed to read small2.parquet: {:?}", result.err());
    
    let parquet_data = result.unwrap();
    assert!(parquet_data.total_rows > 0);
    // 発売日と最終更新日時が含まれていることを確認
    assert!(parquet_data.columns.iter().any(|c| c == "発売日"), "Expected '発売日' column");
    assert!(parquet_data.columns.iter().any(|c| c == "最終更新日時"), "Expected '最終更新日時' column");
}

#[test]
fn test_read_small3_parquet() {
    let data = read_test_file("small3.parquet");
    let result = read_parquet_internal(&data);
    assert!(result.is_ok(), "Failed to read small3.parquet: {:?}", result.err());
    
    let parquet_data = result.unwrap();
    assert!(parquet_data.total_rows > 0);
    // 様々な日付・時刻型のカラムが含まれていることを確認
    assert!(parquet_data.columns.iter().any(|c| c.contains("Date32")), "Expected Date32 column");
    assert!(parquet_data.columns.iter().any(|c| c.contains("Date64")), "Expected Date64 column");
    assert!(parquet_data.columns.iter().any(|c| c.contains("Timestamp")), "Expected Timestamp column");
}

#[test]
fn test_read_parquet_with_date_types() {
    // small2.parquetにはDate32とTimestampが含まれている
    let data = read_test_file("small2.parquet");
    let result = read_parquet_internal(&data);
    assert!(result.is_ok());
    
    let parquet_data = result.unwrap();
    // 発売日（Date32）が正しくフォーマットされていることを確認
    let release_date_idx = parquet_data.columns.iter()
        .position(|c| c == "発売日")
        .expect("発売日 column not found");
    
    // 最初の行の日付がYYYY-MM-DD形式であることを確認
    if let Some(first_row) = parquet_data.rows.first() {
        let date_str = &first_row[release_date_idx];
        // YYYY-MM-DD形式であることを確認（NULLでない場合）
        if date_str != "NULL" {
            assert!(date_str.matches('-').count() == 2, 
                   "Expected YYYY-MM-DD format, got: {}", date_str);
            assert_eq!(date_str.len(), 10, "Expected YYYY-MM-DD format (10 chars), got: {} ({} chars)", 
                      date_str, date_str.len());
        }
    }
}

#[test]
fn test_read_parquet_with_all_date_time_types() {
    // small3.parquetには様々な日付・時刻型が含まれている
    let data = read_test_file("small3.parquet");
    let result = read_parquet_internal(&data);
    assert!(result.is_ok(), "Failed to read small3.parquet: {:?}", result.err());
    
    let parquet_data = result.unwrap();
    assert!(parquet_data.total_rows > 0);
    
    // 日付型が正しくフォーマットされていることを確認
    if let Some(first_row) = parquet_data.rows.first() {
        for (col_idx, col_name) in parquet_data.columns.iter().enumerate() {
            let value = &first_row[col_idx];
            if value != "NULL" {
                if col_name.contains("Date32") || col_name.contains("Date64") {
                    // 日付はYYYY-MM-DD形式
                    assert!(value.matches('-').count() == 2, 
                           "Expected YYYY-MM-DD format for {}, got: {}", col_name, value);
                } else if col_name.contains("Timestamp") {
                    // タイムスタンプはYYYY-MM-DD HH:MM:SS形式
                    assert!(value.contains(' '), 
                           "Expected YYYY-MM-DD HH:MM:SS format for {}, got: {}", col_name, value);
                } else if col_name.contains("Time32") || col_name.contains("Time64") {
                    // 時刻はHH:MM:SS形式
                    assert!(value.matches(':').count() >= 2, 
                           "Expected HH:MM:SS format for {}, got: {}", col_name, value);
                }
            }
        }
    }
}

