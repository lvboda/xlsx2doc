use std::{fs, io};

use super::{xlsx, docx};
use super::utils::{read_config, image_str_fill, get_files_path};
use super::data_type::DataType;

use serde_json;

use docx_rs::Document;

pub async fn bootstrap() {
    let (config_file_path, target_xlsx_path, template_docx_path, output_dir_path) = get_files_path();
    let mut write_count = 0;
    let mut write_error_count = 0;
    let mut fetch_count = 0;
    println!("Console info: 读取配置文件...");
    let config = read_config(config_file_path);
    println!("Console info: 读取xlsx文件...");
    let (xlsx_data, fetch_error_count) = xlsx::read_xlsx(target_xlsx_path, config).await;
    println!("Console info: 读取模版docx文件...");
    let docx = docx::read_docx(template_docx_path).expect("Console error: docx文件读取失败");
    println!("Console info: 序列化docx数据...");
    let template_docx = serde_json::to_string_pretty::<Document>(&docx.document).expect("Console error: docx数据序列化失败");
    if !output_dir_path.is_dir() {
        println!("Console info: 创建output文件夹...");
        fs::create_dir(output_dir_path.clone()).expect("Console error: output文件夹创建失败");
    }
    for xlsx_item in xlsx_data {
        fetch_count += 1;
        let mut docx = docx.clone();
        let mut docx_str = template_docx.clone();
        let mut id = "".to_string();
        let mut name = "".to_string();
        let mut workplace = "".to_string();
        let mut image_id = fetch_count;
        for item in xlsx_item {
            image_id += 1;
            if let Some(data) = item.data {
                match data {
                    DataType::Text(text) => {
                        let find_str = format!("${}", item.field);
                        docx_str = docx_str.replace(&find_str, &text.replace("\n", ""));
                        if item.field.contains("身份证号") {
                            id = text.clone();
                        }
                        if item.field.contains("姓名") {
                            name = text.clone();
                        }
                        if item.field.contains("所属单位") {
                            workplace = text.clone();
                        }
                    },
                    DataType::Image(image) => {
                        let find_str = format!("${}", item.field);
                        if let Some(find_index) = docx_str.find(&find_str) {
                            if let Some((width, height)) = item.size {
                                let docx_str_chunk = &docx_str[find_index .. docx_str.len()];
                                let mut ii = 0;
                                let mut index = 0;
                                while let Some(i) = docx_str_chunk.find("}") {
                                    ii += 1;
                                    if ii == 2 {
                                        index = find_index + i;
                                        break;
                                    } 
                                }
                                let str = image_str_fill(image_id, image, width, height);
                                docx_str = docx_str[0 .. index + 50].to_string() + &str + &docx_str[index .. docx_str.len()].to_string();
                                docx_str = docx_str.replace(&find_str, "");
                            }
                        }
                    }
                }
            }
        }
        let document = serde_json::from_str::<Document>(&docx_str).expect("Console error: Document反序列化失败");
        docx.document = document;
        let docx_name = format!("{}-{}.docx", workplace, name);
        println!("Console info: 准备写入{}", &docx_name);
        let mut docx_path = output_dir_path.join(&docx_name);
        if docx_path.is_file() {
            let docx_name = format!("{}-{}-{}.docx", workplace, name, id);
            docx_path = output_dir_path.join(&docx_name);
        }
        match docx::write_docx(docx, docx_path) {
            Ok(_) => {
                write_count += 1;
                println!("Console info: {}写入完成", &docx_name);
            },
            Err(e) => {
                write_error_count += 1;
                println!("Console error: {}写入错误, 错误信息:{:#?}", &docx_name, e);
            },
        }
    }

    println!("程序执行完成");
    println!("读取: {:?}条\n读取错误: {}条", fetch_count, fetch_error_count);
    println!("写入: {}条\n写入错误: {}条", write_count, write_error_count);
    println!("按任意键退出程序");
    io::stdin().read_line(&mut String::new()).unwrap();
}