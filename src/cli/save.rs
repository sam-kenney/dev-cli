use std::path::Path;


pub trait Save {
    fn mkdir(&self, dir: &String) -> &Self;
    fn save(&self, path: &Path) -> &Self;
}

impl Save for String {
    fn mkdir(&self, dir: &String) -> &Self {
        std::fs::create_dir_all(&dir).expect("Unable to create directory.");
        self
    }

    fn save(&self, path: &Path) -> &Self {
        std::fs::write(&path, self).expect("Unable to write file.");
        self
    }
}
