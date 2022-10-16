use std::path::Path;
use calamine::{Reader, open_workbook_auto};

use super::utils::fetch_img;
use super::data_type::{Field, DataType};

pub async fn read_xlsx<P: AsRef<Path>>(path: P, config: Vec<Field>) -> (Vec<Vec<Field>>, i32)  {
    let mut workbook = open_workbook_auto(path).expect("Console error: xlsx文件路径解析错误");
    let sheet_name = format!("{}", workbook.sheet_names()[0]);
    if let Some(res) = workbook.worksheet_range(&sheet_name) {
        match res {
            Ok(range) => {
                let (rows, columns) = range.get_size();
                let mut res_data = Vec::new();
                let mut fetch_error_count = 0;
                for row in 1 .. rows {
                    let mut config_copy = config.clone();
                    for column in 0 .. columns {
                        let field = range.get_value((0, column.try_into().unwrap()));
                        let value = range.get_value((row.try_into().unwrap(), column.try_into().unwrap()));
                        if let Some(key) = field {
                            if let Some(value) = value {
                                let mut error_flag = false;
                                for item in &mut config_copy {
                                    let key_str = key.to_string();
                                    let value_str = value.to_string();
                                    if key_str.contains(&item.field) {
                                        match item.field_type.as_str() {
                                            "text" => {
                                                item.data = Some(DataType::Text(value_str.trim().to_string()));    
                                            },
                                            "date" => {
                                                if let Some(date) = value.as_date() {
                                                    item.data = Some(DataType::Text(date.to_string().trim().to_string()));
                                                }
                                            },
                                            "image" => {
                                                println!("Console info: 请求图片资源...");
                                                // let (tx, mut rx) = tokio::sync::mpsc::channel(32);
                                                // tokio::spawn(async move {
                                                //     match fetch_img(&value_str).await {
                                                //         Ok(img_bytes) => {
                                                //             tx.send(Some(DataType::Image(img_bytes))).await;
                                                //         },
                                                //         Err(e) => {
                                                //             // error_flag = true;
                                                //             println!("Console error: 资源请求失败, 错误行: {}, 错误信息: {:?}", row, e);
                                                //         },
                                                //     }
                                                // });                     
                                                // while let Some(image) = rx.recv().await {
                                                //     item.data = image;
                                                // }
                                                match fetch_img(&value_str).await {
                                                    Ok(img_bytes) => {
                                                        // item.data = Some(DataType::Image(vec![]));
                                                        item.data = Some(DataType::Image(img_bytes));
                                                    },
                                                    Err(e) => {
                                                        error_flag = true;
                                                        println!("Console error: 资源请求失败, 错误行: {}, 错误信息: {:?}", row, e);
                                                    },
                                                }
                                            },
                                            _ => {
                                                println!("Console error: config文件格式错误");
                                            },
                                        }
                                    }
                                }
                                if error_flag {
                                    fetch_error_count += 1;
                                }
                            }
                        }
                    }
                    res_data.push(config_copy);
                }
                (res_data, fetch_error_count)
            },
            Err(_) => panic!("Console error: 未找到xlsx表"),
        }
    } else {
        panic!("Console error: 未找到xlsx表");
    }
}