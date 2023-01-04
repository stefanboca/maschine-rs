use super::{color::BlendMode, Canvas, Color, Error, UVec2};

pub struct MonochromeCanvas<const W: u32, const H: u32, const SIZE: usize, const NCHUNKS: usize> {
    chunk_dirty_flags: [bool; NCHUNKS],
    buffer: [u8; SIZE],
}

impl<const W: u32, const H: u32, const SIZE: usize, const NCHUNKS: usize>
    MonochromeCanvas<W, H, SIZE, NCHUNKS>
{
    pub fn new() -> Self {
        MonochromeCanvas {
            chunk_dirty_flags: [true; NCHUNKS],
            buffer: [0; SIZE],
        }
    }
}

impl<const W: u32, const H: u32, const SIZE: usize, const NCHUNKS: usize> Default
    for MonochromeCanvas<W, H, SIZE, NCHUNKS> {
    fn default() -> Self {
        Self::new()
    }

}

impl<const W: u32, const H: u32, const SIZE: usize, const NCHUNKS: usize> Canvas
    for MonochromeCanvas<W, H, SIZE, NCHUNKS>
{
    fn size(&self) -> UVec2 {
        UVec2::new(W, H)
    }

    fn number_of_chunks(&self) -> u32 {
        NCHUNKS as u32
    }

    fn buffer(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    fn set_dirty(&mut self) {
        for c in self.chunk_dirty_flags.iter_mut() {
            *c = true;
        }
    }

    fn is_dirty(&self) -> bool {
        self.chunk_dirty_flags.iter().any(|v| *v)
    }

    fn is_chunk_dirty(&self, chunk: u32) -> bool {
        *self
            .chunk_dirty_flags
            .get(chunk as usize)
            .to_owned()
            .unwrap_or(&false)
    }

    fn set_chunk_dirty(&mut self, y: u32) {
        let chunk_height = H / NCHUNKS as u32;
        if y < self.size().y && chunk_height != 0 && NCHUNKS != 0 {
            let chunk = u32::min(y / chunk_height, NCHUNKS as u32 - 1) as usize;
            if let Some(c) = self.chunk_dirty_flags.get_mut(chunk) {
                *c = true;
            }
        }
    }

    fn clear_dirty(&mut self) {
        self.chunk_dirty_flags.fill(false)
    }

    fn set_pixel(&mut self, p: UVec2, color: Color) -> Result<(), Error> {
        if p.oob(self.size()) {
            return Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            });
        } else if color.blend_mode == BlendMode::Transparent {
            return Ok(());
        }

        let old_color = self.get_pixel(p)?;
        let is_white = color.is_active();

        if old_color.is_active() != is_white {
            let idx = (SIZE as u32 / H * p.y + (p.x >> 3)) as usize;

            if is_white {
                self.buffer_mut()[idx] |= 0x80 >> (p.x & 7);
            } else {
                self.buffer_mut()[idx] &= (!0x80) >> (p.x & 7);
            }

            self.set_chunk_dirty(p.y);
        }
        Ok(())
    }

    fn get_pixel(&self, p: UVec2) -> Result<Color, Error> {
        if p.oob(self.size()) {
            Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            })
        } else if self.buffer()[(SIZE as u32 / H * p.y + (p.x >> 3)) as usize] & (0x80 >> (p.x & 7))
            == 0
        {
            Ok(Color::BLACK)
        } else {
            Ok(Color::WHITE)
        }
    }
}
