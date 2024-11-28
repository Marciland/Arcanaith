use super::ResourceSystem;
use ab_glyph::FontVec;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

fn read_font_file(file_path: &Path) -> Option<FontVec> {
    let path = file_path.to_str()?;
    let font_data = read_bytes_from_file(path).expect(&("Failed to read: ".to_string() + path));
    match FontVec::try_from_vec(font_data) {
        Ok(font) => Some(font),
        Err(e) => {
            println!("Invalid font data from {file_path:?}: {e}");
            None
        }
    }
}

impl ResourceSystem {
    fn create_font_map(&self) -> HashMap<String, FontVec> {
        let font_files = self.gather_font_files();
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

    fn gather_font_files(&self) -> Vec<PathBuf> {
        let mut font_files = Vec::with_capacity(2);

        for entry in read_dir(&self.font_base_path)
            .expect("Failed to read font base path!")
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
}
