use super::*;

use std::io::Result;
use std::io::prelude::*;
use std::fs::File;

pub struct HtmlCanvasBackend<'a> {
    filename: &'a str,
    size: Option<(u32, u32)>,
    file: Option<File>,
}

impl<'a> HtmlCanvasBackend<'a> {
    pub fn new(filename: &'a str) -> HtmlCanvasBackend {
        HtmlCanvasBackend {
            filename,
            size: None,
            file: None,
        }
    }
}

impl<'a> RenderBackend for HtmlCanvasBackend<'a> {
    fn render_size(&mut self, width: u32, height: u32) {
        self.size = Some((width, height));
    }

    fn start_render(&mut self) -> Result<()> {
        let mut file = File::create(self.filename)?;

        let html_header = include_str!("html.header.txt");

        write!(file, "{}", html_header)?;

        self.file = Some(file);

        Ok(())
    }

    fn finish_render(&mut self) -> Result<()> {
        let html_footer = include_str!("html.footer.txt");

        if let Some(ref mut file) = self.file {
            write!(file, "{}", html_footer)?;
            file.flush()?;
        }
        Ok(())
    }

    fn put_pixel(&mut self, x: u32, y: u32, color: &Color) -> Result<()> {
        if let Some(ref mut file) = self.file {
            write!(
                file,
                "pixel({}, {}, {}, {}, {});\n",
                x,
                y,
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8
            )?;
        }
        Ok(())
    }
}
