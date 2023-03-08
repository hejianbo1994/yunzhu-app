// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image;
use image::io::Reader as ImageReader;
use std::fmt;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use text_to_png::{TextPng, TextRenderer, TextToPngError};
use thiserror::Error;

// impl fmt::Display for TextPng {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "1111", self.data)
//     }
// }

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("")]
    ImageError(#[from] image::error::ImageError),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn handle_text_to_png(html: String) -> Result<(), Error> {
    let renderer = TextRenderer::default();
    let res = renderer.render_text_to_png_data(html, 64, "Dark Turquoise");
    match res {
        Ok(v) => {
            println!("sr value: {:?}", v);
            println!("sr value: {:?}", (&v.data).len());
            let mut bytes: Vec<u8> = Vec::new();
            let img = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()?
                .decode()?;
            bytes = v.data;
            img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
            img.save("empty.png")?;

            // let buffer: &[u8] = &v.data;
            // image::save_buffer(
            //     "image.png",
            //     buffer,
            //     v.size.width,
            //     v.size.height,
            //     image::ColorType::Rgb8,
            // )
            // .unwrap();
            Ok(())
        }
        Err(e) => {
            println!("sr value: {:?}", e);
            // Err(Error(""))
            panic!("called `Option::unwrap()` on a `None` value")
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_text_to_png])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
