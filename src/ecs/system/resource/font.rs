use crate::{constants::FONTS, read_bytes_from_file};
use ab_glyph::FontVec;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn create_font_map() -> HashMap<String, FontVec> {
    let font_files = gather_font_files();
    let mut font_map = HashMap::with_capacity(font_files.len());

    for font_file in font_files {
        let Some(file_name) = font_file.file_stem().and_then(|name| name.to_str()) else {
            continue;
        };

        let Some(font) = read_font_file(&font_file) else {
            continue;
        };

        font_map.insert(file_name.to_string(), font);
    }

    font_map
}

fn read_font_file(file_path: &Path) -> Option<FontVec> {
    let path = file_path.to_str().unwrap();
    let font_data = read_bytes_from_file(path).expect(&("Failed to read: ".to_string() + path));
    match FontVec::try_from_vec(font_data) {
        Ok(font) => Some(font),
        Err(e) => {
            println!("Invalid font data from {file_path:?}: {e}");
            None
        }
    }
}

fn gather_font_files() -> Vec<PathBuf> {
    let mut font_files = Vec::with_capacity(2);

    for entry in read_dir(FONTS)
        .expect(&("Failed to read dir: ".to_string() + FONTS))
        .flatten()
    {
        let path = entry.path();
        if let Some(file_extension) = path.extension() {
            if file_extension == "ttf" {
                font_files.push(path);
            }
        }
    }

    font_files
}
