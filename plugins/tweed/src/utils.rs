use include_dir::{include_dir, Dir};
use std::{
    env::temp_dir,
    fs::{create_dir_all, File},
    io::Write,
};

static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/res");

pub fn ensure_models_folder() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut models = vec![];
    let mut tmp = temp_dir();
    tmp.push("tweed");
    create_dir_all(&tmp)?;

    for i in ASSETS_DIR.entries() {
        if let Some(i_file) = i.as_file() {
            let mut full_path = tmp.clone();
            full_path.push(i_file.path());
            let full_path_str = full_path.to_string_lossy().to_string();

            let mut file = File::create(&full_path_str)?;
            file.write_all(i_file.contents())?;
            models.push(full_path_str);
        }
    }
    models.sort();
    Ok(models)
}
