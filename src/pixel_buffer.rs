#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub struct PixelBuffer {
    width: u32,
    height: u32,
    buf: Vec<Pixel>,
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buf: vec![Pixel::default(); (width * height) as usize],
        }
    }

    pub fn clear(&mut self, color: Pixel) {
        self.buf.fill(color);
    }

    pub fn pitch(&self) -> u32 {
        self.width * std::mem::size_of::<Pixel>() as u32
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl AsRef<[u8]> for PixelBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.buf.as_ptr() as *const u8,
                (self.pitch() * self.height) as usize,
            )
        }
    }
}
