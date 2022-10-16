use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use reqwest::{get, Error};
use serde_json::from_str;
use crate::data_type::Field;

pub async fn fetch_img(url: &str) -> Result<Vec<u8>, Error> {
    match get(url).await {
        Ok(res) => {
            match res.bytes().await {
                Ok(res) => Ok(res.to_vec()),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Vec<Field> {
    match fs::read_to_string(path) {
        Ok(config_str) => {
            match from_str(&config_str) {
                Ok(v) => v,
                Err(e) => panic!("Console error: config文件解析错误, 错误信息为: {}", e),
            }
        },
        Err(e) => panic!("Console error: config文件读取错误, 错误信息为: {}", e),
    }
}

pub fn get_files_path() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let mut current_dir_path = env::current_exe().expect("Console error: exe路径解析失败");
    current_dir_path.pop();
    let config_file_path = current_dir_path.join("config.json");
    let target_xlsx_path = current_dir_path.join("target.xlsx");
    let template_docx_path = current_dir_path.join("template.docx");
    let output_dir_path = current_dir_path.join("output");
    (config_file_path, target_xlsx_path, template_docx_path, output_dir_path)
}

pub fn image_str_fill(id: i32, image: Vec<u8>, width: i32, height: i32) -> String {
    let image_str = serde_json::to_string(&image).expect("图片反序列化错误");
    format!(r#"
      ,{{  
        "Drawing": {{
          "position_type": {{
            "Inline": {{
              "dist_t": 0,
              "dist_b": 0,
              "dist_l": 0,
              "dist_r": 0
            }}
          }},
          "position_h": {{ "Offset": 0 }},
          "position_v": {{ "Offset": 0 }},
          "data": {{
            "Pic": {{
              "id": {},
              "image": {},
              "size": [{}, {}],
              "positionType": {{
                "Inline": {{
                  "dist_t": 0,
                  "dist_b": 0,
                  "dist_l": 0,
                  "dist_r": 0
                }}
              }},
              "positionH": {{ "Offset": 0 }},
              "positionV": {{ "Offset": 0 }}
            }}
          }},
          "children": []
      "#, id, image_str, width, height)
}