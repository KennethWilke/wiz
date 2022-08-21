use std::{fs, path::Path};

pub fn get_absolute_path(path: impl ToString) -> String {
    let path = path.to_string();
    let path = Path::new(path.as_str());
    fs::canonicalize(path)
        .expect("expected")
        .to_str()
        .expect("failed to convert PathBuf to &str")
        .to_string()
}
