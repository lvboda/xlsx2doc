use std::fs;
use std::path::Path;
use super::data_type::Error;
use docx_rs::*;

pub fn read_docx<P: AsRef<Path>>(path: P) -> Result<Docx, Error> {
    match fs::read(path) {
        Ok(value) => match docx_rs::read_docx(&value) {
            Ok(docx) => Ok(docx),
            Err(e) => Err(Error::IoReadError(e.to_string())),
        },
        Err(e) => Err(Error::IoReadError(e.to_string())),
    }
}

pub fn write_docx<P: AsRef<Path>>(docx: Docx, target_path: P) -> Result<String, Error> {
    match fs::File::create(target_path) {
        Ok(file) => {
            let res = match docx.build().pack(file) {
                Ok(_) => {
                    Ok("Finish file write".to_string())
                },
                Err(e) => Err(Error::IoWriteError(e.to_string())),
            };
            res
        },
        Err(e) => Err(Error::IoWriteError(e.to_string())),
    }
}
