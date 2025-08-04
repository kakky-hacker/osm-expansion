use std::path::Path;

pub trait BaseLoader {
    fn load(&mut self, path: &Path);
}
