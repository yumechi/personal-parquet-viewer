use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use arrow::array::*;
use arrow::datatypes::*;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use bytes::Bytes;

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
        _ => format!("Unsupported type: {:?}", array.data_type()),
    }
}
