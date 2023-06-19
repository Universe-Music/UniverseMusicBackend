use std::fs;
use std::fs::{File, ReadDir};
use std::io::ErrorKind;

#[derive(Clone, Debug)]
pub struct ErrorDef {
    path: String,
    error: ErrorKind,
}
pub struct Files {
    instance: Vec<ReadDir>,
    errors: Vec<ErrorDef>,
    last: ReadDir,
}

impl Files {
    pub fn new(path: &str) -> Result<Files, std::io::Error> {
        let f = match fs::read_dir(path) {
            Err(e) => return Err(e),
            Ok(f) => f,
        };
        Ok(Files {
            instance: Vec::new(),
            errors: Vec::new(),
            last: f,
        })
    }
    pub fn get_errors(&self) -> Vec<ErrorDef> {
        return self.errors.clone();
    }
    pub fn next(&mut self) -> Option<String> {
        let file_obj = loop {
            let available_file = loop {
                let f = match self.last.next() {
                    None => {
                        if self.instance.len() == 0 {
                            return None;
                        } else {
                            self.last = self.instance.pop().unwrap();
                        }
                        continue;
                    }
                    Some(t) => t,
                };
                match f {
                    Ok(t) => break t,
                    Err(e) => {
                        self.errors.push(ErrorDef {
                            path: String::from(""),
                            error: e.kind(),
                        });
                    }
                };
            };
            match available_file.metadata() {
                Ok(meta) => {
                    if meta.is_dir() {
                        self.add_dir(available_file.path().to_str().unwrap_or(""));
                        println!("dir: {:?}", available_file.path());
                        continue;
                    } else {
                        println!("file: {:?}", available_file.path());
                        break available_file.path().to_str().unwrap_or("").to_string();
                    }
                }
                Err(e) => {
                    self.errors.push(ErrorDef {
                        path: available_file.path().to_str().unwrap_or("").to_string(),
                        error: e.kind(),
                    });
                    continue;
                }
            }
        };
        Some(file_obj)
    }
    fn add_dir(&mut self, path: &str) {
        let dir = fs::read_dir(path);
        match dir {
            Err(e) => self.errors.push(ErrorDef {
                path: String::from(path),
                error: e.kind(),
            }),
            Ok(t) => self.instance.push(t),
        }
    }
}
