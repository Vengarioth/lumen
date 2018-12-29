pub struct Buffer {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Buffer {
        Buffer {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        let i = ((((self.height - y - 1) * self.width) + x) * 4) as usize;
        (self.data[i], self.data[i + 1], self.data[i + 2], self.data[i + 3])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        let i = ((((self.height - y - 1) * self.width) + x) * 4) as usize;
        self.data[i] = r;
        self.data[i + 1] = g;
        self.data[i + 2] = b;
        self.data[i + 3] = a;
    }

    pub fn blit(&mut self, other: &Buffer, offset_x: u32, offset_y: u32) {
        for x in 0..other.width {
            for y in 0..other.height {
                let (r, g, b, a) = other.get_pixel(x, y);
                self.set_pixel(x + offset_x, y + offset_y, r, g, b, a);
            }
        }
    }

    pub fn swap(&mut self) -> Vec<u8> {
        use std::mem::swap;

        let size = (self.width * self.height * 4) as usize;
        let mut data = Vec::with_capacity(size);

        swap(&mut data, &mut self.data);

        data
    }

    pub fn save_to_file(&self, path: &str) {
        use std::path::Path;
        use std::fs::File;
        use std::io::BufWriter;
        use png::HasParameters;

        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.data).unwrap();
    }
}
