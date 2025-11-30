use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use arrow::array::*;
use arrow::datatypes::*;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Serialize, Deserialize)]
pub struct ParquetData {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn read_parquet(data: &[u8]) -> Result<JsValue, JsValue> {
    let bytes = Bytes::from(data.to_vec());

    let builder = ParquetRecordBatchReaderBuilder::try_new(bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to create reader: {}", e)))?;

    let schema = builder.schema().clone();
    let mut reader = builder.build()
        .map_err(|e| JsValue::from_str(&format!("Failed to build reader: {}", e)))?;

    let mut all_rows: Vec<Vec<String>> = Vec::new();
    let columns: Vec<String> = schema
        .fields()
        .iter()
        .map(|f| f.name().clone())
        .collect();

    while let Some(batch) = reader.next() {
        let batch = batch
            .map_err(|e| JsValue::from_str(&format!("Failed to read batch: {}", e)))?;

        for row_idx in 0..batch.num_rows() {
            let mut row: Vec<String> = Vec::new();

            for col_idx in 0..batch.num_columns() {
                let column = batch.column(col_idx);
                let value = array_value_to_string(column, row_idx);
                row.push(value);
            }

            all_rows.push(row);
        }
    }

    let result = ParquetData {
        columns,
        total_rows: all_rows.len(),
        rows: all_rows,
    };

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize: {}", e)))
}

fn array_value_to_string(array: &dyn Array, index: usize) -> String {
    if array.is_null(index) {
        return "NULL".to_string();
    }

    match array.data_type() {
        DataType::Boolean => {
            let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Int8 => {
            let arr = array.as_any().downcast_ref::<Int8Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Int16 => {
            let arr = array.as_any().downcast_ref::<Int16Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Int32 => {
            let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Int64 => {
            let arr = array.as_any().downcast_ref::<Int64Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::UInt8 => {
            let arr = array.as_any().downcast_ref::<UInt8Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::UInt16 => {
            let arr = array.as_any().downcast_ref::<UInt16Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::UInt32 => {
            let arr = array.as_any().downcast_ref::<UInt32Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::UInt64 => {
            let arr = array.as_any().downcast_ref::<UInt64Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Float32 => {
            let arr = array.as_any().downcast_ref::<Float32Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Float64 => {
            let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Utf8 => {
            let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
            arr.value(index).to_string()
        }
        DataType::LargeUtf8 => {
            let arr = array.as_any().downcast_ref::<LargeStringArray>().unwrap();
            arr.value(index).to_string()
        }
        DataType::Binary => {
            let arr = array.as_any().downcast_ref::<BinaryArray>().unwrap();
            format!("{:?}", arr.value(index))
        }
        DataType::LargeBinary => {
            let arr = array.as_any().downcast_ref::<LargeBinaryArray>().unwrap();
            format!("{:?}", arr.value(index))
        }
        DataType::Date32 => {
            let arr = array.as_any().downcast_ref::<Date32Array>().unwrap();
            format_date32(arr.value(index))
        }
        DataType::Date64 => {
            let arr = array.as_any().downcast_ref::<Date64Array>().unwrap();
            format_date64(arr.value(index))
        }
        DataType::Time32(unit) => {
            format_time32(array, index, *unit)
        }
        DataType::Time64(unit) => {
            format_time64(array, index, *unit)
        }
        DataType::Timestamp(unit, _tz) => {
            format_timestamp(array, index, *unit)
        }
        DataType::Duration(unit) => {
            format_duration(array, index, *unit)
        }
        DataType::Interval(unit) => {
            format_interval(array, index, *unit)
        }
        _ => format!("Unsupported type: {:?}", array.data_type()),
    }
}

/// Date32型（1970-01-01からの日数）を日付文字列に変換
fn format_date32(days: i32) -> String {
    // Date32は1970-01-01からの日数
    // 1970-01-01は紀元からの日数で719163日目
    const UNIX_EPOCH_DAYS: i32 = 719163;
    let date = NaiveDate::from_num_days_from_ce_opt(UNIX_EPOCH_DAYS + days)
        .unwrap_or_else(|| NaiveDate::from_num_days_from_ce_opt(UNIX_EPOCH_DAYS).unwrap());
    date.format("%Y-%m-%d").to_string()
}

/// Date64型（1970-01-01からのミリ秒数）を日付文字列に変換
fn format_date64(millis: i64) -> String {
    // Date64は1970-01-01 00:00:00 UTCからのミリ秒数
    let datetime = DateTime::<Utc>::from_timestamp_millis(millis)
        .unwrap_or_else(|| Utc::now());
    datetime.format("%Y-%m-%d").to_string()
}

/// Time32型を時刻文字列に変換
fn format_time32(array: &dyn Array, index: usize, unit: TimeUnit) -> String {
    match unit {
        TimeUnit::Second => {
            let arr = array.as_any().downcast_ref::<Time32SecondArray>().unwrap();
            let seconds = arr.value(index);
            format_time_from_seconds(seconds as i64, 0)
        }
        TimeUnit::Millisecond => {
            let arr = array.as_any().downcast_ref::<Time32MillisecondArray>().unwrap();
            let millis = arr.value(index);
            let seconds = (millis / 1000) as i64;
            let nanos = ((millis % 1000) * 1_000_000) as i64;
            format_time_from_seconds(seconds, nanos)
        }
        _ => format!("Unsupported Time32 unit: {:?}", unit),
    }
}

/// Time64型を時刻文字列に変換
fn format_time64(array: &dyn Array, index: usize, unit: TimeUnit) -> String {
    match unit {
        TimeUnit::Microsecond => {
            let arr = array.as_any().downcast_ref::<Time64MicrosecondArray>().unwrap();
            let micros = arr.value(index);
            let seconds = micros / 1_000_000;
            let nanos = (micros % 1_000_000) * 1000;
            format_time_from_seconds(seconds, nanos)
        }
        TimeUnit::Nanosecond => {
            let arr = array.as_any().downcast_ref::<Time64NanosecondArray>().unwrap();
            let nanos = arr.value(index);
            let seconds = nanos / 1_000_000_000;
            let remaining_nanos = nanos % 1_000_000_000;
            format_time_from_seconds(seconds, remaining_nanos)
        }
        _ => format!("Unsupported Time64 unit: {:?}", unit),
    }
}

/// 秒数とナノ秒から時刻文字列を生成
fn format_time_from_seconds(seconds: i64, nanos: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if nanos > 0 {
        let micros = nanos / 1000;
        if micros > 0 {
            format!("{:02}:{:02}:{:02}.{:06}", hours, minutes, secs, micros)
        } else {
            format!("{:02}:{:02}:{:02}", hours, minutes, secs)
        }
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }
}

/// Timestamp型を日時文字列に変換
fn format_timestamp(array: &dyn Array, index: usize, unit: TimeUnit) -> String {
    let timestamp = match unit {
        TimeUnit::Second => {
            let arr = array.as_any().downcast_ref::<TimestampSecondArray>().unwrap();
            arr.value(index) as i64
        }
        TimeUnit::Millisecond => {
            let arr = array.as_any().downcast_ref::<TimestampMillisecondArray>().unwrap();
            arr.value(index)
        }
        TimeUnit::Microsecond => {
            let arr = array.as_any().downcast_ref::<TimestampMicrosecondArray>().unwrap();
            arr.value(index)
        }
        TimeUnit::Nanosecond => {
            let arr = array.as_any().downcast_ref::<TimestampNanosecondArray>().unwrap();
            arr.value(index)
        }
    };

    // タイムスタンプを秒単位に変換
    let (seconds, nanos) = match unit {
        TimeUnit::Second => (timestamp, 0),
        TimeUnit::Millisecond => (timestamp / 1000, (timestamp % 1000) * 1_000_000),
        TimeUnit::Microsecond => (timestamp / 1_000_000, (timestamp % 1_000_000) * 1000),
        TimeUnit::Nanosecond => (timestamp / 1_000_000_000, timestamp % 1_000_000_000),
    };

    // UTCとして扱う（タイムゾーン処理は将来の改善で対応）
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos as u32)
        .unwrap_or_else(|| Utc::now());

    // マイクロ秒精度まで表示
    if nanos > 0 {
        let micros = nanos / 1000;
        datetime.format("%Y-%m-%d %H:%M:%S").to_string() + &format!(".{:06}", micros)
    } else {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

/// Duration型を期間文字列に変換
fn format_duration(array: &dyn Array, index: usize, unit: TimeUnit) -> String {
    let value = match unit {
        TimeUnit::Second => {
            let arr = array.as_any().downcast_ref::<DurationSecondArray>().unwrap();
            arr.value(index) as i64
        }
        TimeUnit::Millisecond => {
            let arr = array.as_any().downcast_ref::<DurationMillisecondArray>().unwrap();
            arr.value(index)
        }
        TimeUnit::Microsecond => {
            let arr = array.as_any().downcast_ref::<DurationMicrosecondArray>().unwrap();
            arr.value(index)
        }
        TimeUnit::Nanosecond => {
            let arr = array.as_any().downcast_ref::<DurationNanosecondArray>().unwrap();
            arr.value(index)
        }
    };

    // 期間を適切な単位で表示
    match unit {
        TimeUnit::Second => format!("{}s", value),
        TimeUnit::Millisecond => format!("{}ms", value),
        TimeUnit::Microsecond => format!("{}μs", value),
        TimeUnit::Nanosecond => format!("{}ns", value),
    }
}

/// Interval型を間隔文字列に変換
fn format_interval(array: &dyn Array, index: usize, unit: IntervalUnit) -> String {
    match unit {
        IntervalUnit::YearMonth => {
            let arr = array.as_any().downcast_ref::<IntervalYearMonthArray>().unwrap();
            let value = arr.value(index);
            let years = value / 12;
            let months = value % 12;
            if years > 0 {
                format!("{}年{}ヶ月", years, months)
            } else {
                format!("{}ヶ月", months)
            }
        }
        IntervalUnit::DayTime => {
            let arr = array.as_any().downcast_ref::<IntervalDayTimeArray>().unwrap();
            let value = arr.value(index);
            // IntervalDayTimeはdaysとmillisecondsのフィールドを持つ構造体
            let days = value.days;
            let millis = value.milliseconds;
            let hours = millis / 3_600_000;
            let minutes = (millis % 3_600_000) / 60_000;
            let seconds = (millis % 60_000) / 1000;
            if days > 0 {
                format!("{}日 {:02}:{:02}:{:02}", days, hours, minutes, seconds)
            } else {
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
            }
        }
        IntervalUnit::MonthDayNano => {
            let arr = array.as_any().downcast_ref::<IntervalMonthDayNanoArray>().unwrap();
            let value = arr.value(index);
            // IntervalMonthDayNanoはmonths, days, nanosecondsのフィールドを持つ構造体
            let months = value.months;
            let days = value.days;
            let nanos = value.nanoseconds;
            let years = months / 12;
            let remaining_months = months % 12;
            let seconds = nanos / 1_000_000_000;
            let remaining_nanos = nanos % 1_000_000_000;
            
            let mut parts = Vec::new();
            if years > 0 {
                parts.push(format!("{}年", years));
            }
            if remaining_months > 0 {
                parts.push(format!("{}ヶ月", remaining_months));
            }
            if days > 0 {
                parts.push(format!("{}日", days));
            }
            if seconds > 0 || remaining_nanos > 0 {
                let hours = seconds / 3600;
                let mins = (seconds % 3600) / 60;
                let secs = seconds % 60;
                if remaining_nanos > 0 {
                    let micros = remaining_nanos / 1000;
                    parts.push(format!("{:02}:{:02}:{:02}.{:06}", hours, mins, secs, micros));
                } else {
                    parts.push(format!("{:02}:{:02}:{:02}", hours, mins, secs));
                }
            }
            
            if parts.is_empty() {
                "0".to_string()
            } else {
                parts.join(" ")
            }
        }
    }
}
