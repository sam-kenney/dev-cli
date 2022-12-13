use std::path::Path;
use crate::cli::{save::Save, utils};

pub struct File {
    pub name: String,
    pub dir: String,
    pub content: String,
}

impl File{
    pub fn new(name: String, dir: String, content: String) -> Self {
        let dir_path: String = format!("{}/{}/", utils::current_dir(), dir);
        Self {
            name: name,
            dir: dir_path,
            content: content,
        }
    }

    pub fn write(&self) -> &Self {
        let file_path: String = format!("{}/{}", self.dir, self.name);
        self.content.to_string().mkdir(&self.dir).save(Path::new(&file_path));
        self
    }
}
