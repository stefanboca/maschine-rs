use super::{
    color::{BlendMode, Color},
    error::Error,
    font::Font,
    vec::UVec2,
};

pub enum CircleType {
    Full,
    SemiLeft,
    SemiTop,
    SemiRight,
    SemiBottom,
    QuarterTopLeft,
    QuarterTopRight,
    QuarterBottomRight,
    QuarterBottomLeft,
}

pub trait Canvas {
    fn size(&self) -> UVec2;

    fn number_of_chunks(&self) -> u32;

    fn buffer(&self) -> &[u8];
    fn buffer_mut(&mut self) -> &mut [u8];

    fn set_dirty(&mut self);
    fn is_dirty(&self) -> bool;

    fn is_chunk_dirty(&self, chunk: u32) -> bool;
    fn set_chunk_dirty(&mut self, y: u32);
    fn clear_dirty(&mut self);

    fn white(&mut self) {
        self.fill(0xFF);
        self.set_dirty();
    }
    fn black(&mut self) {
        self.fill(0x00);
        self.set_dirty();
    }
    fn invert(&mut self) {
        for p in self.buffer_mut() {
            *p = !*p;
        }
    }
    fn fill(&mut self, value: u8) {
        self.buffer_mut().fill(value);
    }

    fn get_pixel(&self, p: UVec2) -> Result<Color, Error> {
        if p.oob(self.size()) {
            Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            })
        } else {
            let idx = (self.size().x * p.x + p.y) as usize;
            Ok(self.buffer().as_chunks::<3>().0[idx].into())
        }
    }
    fn set_pixel(&mut self, p: UVec2, color: Color) -> Result<(), Error> {
        if p.oob(self.size()) {
            Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            })
        } else if color.blend_mode == BlendMode::Transparent {
            Ok(())
        } else {
            let old_color = self.get_pixel(p)?;

            if old_color != color {
                let idx = (self.size().x * p.x + p.y) as usize;
                self.buffer_mut().as_chunks_mut().0[idx] = color.as_array_rgb();

                self.set_chunk_dirty(p.y);
            }

            Ok(())
        }
    }

    fn line(&mut self, p0: UVec2, p1: UVec2, color: Color) -> Result<(), Error> {
        let dx = p1.x as i32 - p0.x as i32;
        let dy = p1.y as i32 - p0.y as i32;
        let s1 = dx.signum();
        let s2 = dy.signum();
        let mut dx = dx.abs();
        let mut dy = dy.abs();

        let swapped = dy > dx;
        if swapped {
            (dx, dy) = (dy, dx);
        }

        let mut p = p0;
        let mut e = (dy << 1) - dx;
        for _ in 0..dx {
            self.set_pixel(p, color)?;

            if e >= 0 {
                if swapped {
                    p.x = (p.x as i32 + s1) as u32;
                } else {
                    p.y = (p.y as i32 + s2) as u32;
                }
            }
            if swapped {
                p.y = (p.y as i32 + s2) as u32;
            } else {
                p.x = (p.x as i32 + s1) as u32;
            }
            e += dy << 1
        }

        Ok(())
    }
    fn line_vertical(&mut self, p: UVec2, length: u32, color: Color) -> Result<(), Error> {
        for y in p.y..p.y + length {
            self.set_pixel(UVec2::new(p.x, y), color)?;
        }
        Ok(())
    }
    fn line_horizontal(&mut self, p: UVec2, length: u32, color: Color) -> Result<(), Error> {
        for x in p.x..p.x + length {
            self.set_pixel(UVec2::new(x, p.y), color)?;
        }
        Ok(())
    }

    fn triangle(&mut self, p0: UVec2, p1: UVec2, p2: UVec2, color: Color) -> Result<(), Error> {
        self.line(p0, p1, color)?;
        self.line(p1, p2, color)?;
        self.line(p2, p0, color)
    }
    fn triangle_filled(
        &mut self,
        p0: UVec2,
        p1: UVec2,
        p2: UVec2,
        color: Color,
        fill_color: Color,
    ) -> Result<(), Error> {
        let (mut p0, mut p1, mut p2) = (p0, p1, p2);
        // Sort coordinates by y order (p2.y >= p1.y >= p0.y)
        if p0.y > p1.y {
            (p0, p1) = (p1, p0);
        }
        if p1.y > p2.y {
            (p1, p2) = (p2, p1);
        }
        if p0.y > p1.y {
            (p0, p1) = (p1, p0);
        }

        if p0.y == p2.y {
            let a = u32::min(u32::min(p0.x, p1.x), p2.x);
            let b = u32::max(u32::max(p0.x, p1.x), p2.x);

            return self.line_horizontal(UVec2::new(a, p0.y), b - a + 1, fill_color);
        }

        let dx01 = p1.x as i32 - p0.x as i32;
        let dy01 = p1.y as i32 - p0.y as i32;
        let dx02 = p2.x as i32 - p0.x as i32;
        let dy02 = p2.y as i32 - p0.y as i32;
        let dx12 = p2.x as i32 - p1.x as i32;
        let dy12 = p2.y as i32 - p1.y as i32;

        let mut sa = 0;
        let mut sb = 0;

        // For upper part of triangle, find scanline crossings for segments
        // 0-1 and 0-2.  If y1=y2 (flat-bottomed triangle), the scanline y1
        // is included here (and second loop will be skipped, avoiding a /0
        // error there), otherwise scanline y1 is skipped here and handled
        // in the second loop...which also avoids a /0 error here if y0=y1
        // (flat-topped triangle).
        let last = if p1.y == p2.y { p1.y } else { p1.y - 1 };
        for y in p0.y..=last {
            let a = (p0.x as i32 + sa / dy01) as u32;
            let b = (p0.x as i32 + sb / dy01) as u32;
            sa += dx01;
            sb += dx02;
            // longhand:
            // a = x0 + (x1 - x0) * (y - y0) / (y1 - y0);
            // b = x0 + (x2 - x0) * (y - y0) / (y2 - y0);
            let (a, b) = (u32::min(a, b), u32::max(a, b));
            self.line_horizontal(UVec2::new(a, y), b - a + 1, fill_color)?;
        }

        // For lower part of triangle, find scanline crossings for segments
        // 0-2 and 1-2.  This loop is skipped if y1=y2.
        sa = dx12 * (last as i32 - p1.y as i32);
        sb = dx02 * (last as i32 - p0.y as i32);
        for y in last..=p2.y {
            let a = (p1.x as i32 + sa / dy12) as u32;
            let b = (p0.x as i32 + sb / dy02) as u32;
            sa += dx12;
            sb += dx02;
            // longhand:
            //  a = x1 + (x2 - x1) * (y - y1) / (y2 - y1);
            //  b = x0 + (x2 - x0) * (y - y0) / (y2 - y0);
            let (a, b) = (u32::min(a, b), u32::max(a, b));
            self.line_horizontal(UVec2::new(a, y), b - a + 1, fill_color)?;
        }

        self.line(p0, p1, color)?;
        self.line(p1, p2, color)?;
        self.line(p2, p0, color)
    }

    fn rectangle(&mut self, p: UVec2, size: UVec2, color: Color) -> Result<(), Error> {
        self.rectangle_filled(
            p,
            size,
            color,
            Color::from_blend_mode(BlendMode::Transparent),
        )
    }
    fn rectangle_filled(
        &mut self,
        p: UVec2,
        size: UVec2,
        color: Color,
        fill_color: Color,
    ) -> Result<(), Error> {
        if p.oob(self.size()) {
            return Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            });
        }
        if size.x == 0 || size.y == 0 {
            return Ok(());
        }

        self.line_horizontal(p, size.x, color)?;
        self.line_horizontal(UVec2::new(p.x, p.y + size.y - 1), size.x, color)?;

        if fill_color.blend_mode == BlendMode::Transparent {
            return Ok(());
        }

        if size.x > size.y {
            let line_width = size.x - 2;
            for y in (p.y + 1)..(p.y + size.y - 1) {
                self.line_horizontal(UVec2::new(p.x + 1, y), line_width, fill_color)?;
            }
        } else {
            let line_height = size.y - 2;
            for x in (p.x + 1)..(p.x + size.x - 1) {
                self.line_horizontal(UVec2::new(x, p.y + 1), line_height, fill_color)?;
            }
        }

        Ok(())
    }

    fn rectangle_rounded(
        &mut self,
        p: UVec2,
        size: UVec2,
        r: u32,
        color: Color,
    ) -> Result<(), Error> {
        self.rectangle_rounded_filled(
            p,
            size,
            r,
            color,
            Color::from_blend_mode(BlendMode::Transparent),
        )
    }
    fn rectangle_rounded_filled(
        &mut self,
        p: UVec2,
        size: UVec2,
        r: u32,
        color: Color,
        fill_color: Color,
    ) -> Result<(), Error> {
        if p.oob(self.size()) {
            return Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            });
        }
        if size.x == 0 || size.y == 0 {
            return Ok(());
        }

        let smallest_side = u32::min(size.x, size.y);
        let (r, r_offset) = if r * 2 > smallest_side {
            (smallest_side >> 1, smallest_side)
        } else {
            (r, 2 * r)
        };

        let p1 = UVec2::new(p.x + r, p.y + r);
        let p2 = UVec2::new(p.x + size.x - r - 1, p.y + size.y - r - 1);
        self.line_horizontal(UVec2::new(p1.x, p.y), size.x - r, color)?;
        self.line_horizontal(UVec2::new(p1.x, p.y + size.y - 1), size.x - r, color)?;
        self.line_horizontal(UVec2::new(p.x, p1.y), size.y - r, color)?;
        self.line_horizontal(UVec2::new(p.x + size.x - 1, p1.y), size.y - r, color)?;

        self.circle_filled(p1, r, CircleType::QuarterTopLeft, color, fill_color)?;
        self.circle_filled(
            UVec2::new(p2.x, p1.y),
            r,
            CircleType::QuarterTopRight,
            color,
            fill_color,
        )?;
        self.circle_filled(p2, r, CircleType::QuarterBottomRight, color, fill_color)?;
        self.circle_filled(
            UVec2::new(p1.x, p2.y),
            r,
            CircleType::QuarterBottomRight,
            color,
            fill_color,
        )?;

        if fill_color.blend_mode == BlendMode::Transparent || size.x <= 2 || size.y <= 2 {
            return Ok(());
        }

        self.rectangle_filled(
            UVec2::new(p1.x, p.y + 1),
            UVec2::new(size.x - r_offset, r),
            fill_color,
            fill_color,
        )?;
        self.rectangle_filled(
            UVec2::new(p.x + 1, p.y + r),
            UVec2::new(size.x - 2, size.y - r_offset),
            fill_color,
            fill_color,
        )?;
        self.rectangle_filled(
            UVec2::new(p1.x, p2.y),
            UVec2::new(size.x - r_offset, r),
            fill_color,
            fill_color,
        )
    }

    fn circle(
        &mut self,
        p: UVec2,
        r: u32,
        circle_type: CircleType,
        color: Color,
    ) -> Result<(), Error> {
        self.circle_filled(
            p,
            r,
            circle_type,
            color,
            Color::from_blend_mode(BlendMode::Transparent),
        )
    }
    fn circle_filled(
        &mut self,
        p: UVec2,
        r: u32,
        circle_type: CircleType,
        color: Color,
        fill_color: Color,
    ) -> Result<(), Error> {
        if r == 0 {
            return Ok(());
        }

        let mut rx0 = -(r as i32);
        let mut ry0 = rx0;
        let mut rx1 = 1;
        let mut ry1 = 1;

        match circle_type {
            CircleType::SemiLeft => {
                rx1 = 0;
            }
            CircleType::SemiRight => {
                rx0 = 0;
            }
            CircleType::SemiTop => {
                ry1 = 0;
            }
            CircleType::SemiBottom => {
                ry0 = 0;
            }

            CircleType::QuarterTopLeft => {
                rx1 = 0;
                ry1 = 0;
            }
            CircleType::QuarterTopRight => {
                rx0 = 0;
                ry1 = 0;
            }
            CircleType::QuarterBottomLeft => {
                rx1 = 0;
                ry0 = 0;
            }
            CircleType::QuarterBottomRight => {
                rx0 = 0;
                ry0 = 0;
            }
            _ => {}
        }

        let rsq = r * r;
        for x in rx0..=rx1 {
            for y in ry0..=ry1 {
                let xysq = (x * x + y * y) as u32;
                let p = UVec2::new((p.x as i32 + x) as u32, (p.y as i32 + y) as u32);
                if u32::abs_diff(rsq, xysq) < r {
                    self.set_pixel(p, color)?;
                } else if fill_color.blend_mode != BlendMode::Transparent && xysq < rsq {
                    self.set_pixel(p, fill_color)?;
                }
            }
        }

        Ok(())
    }

    fn put_bitmap(
        &mut self,
        p: UVec2,
        size: UVec2,
        bitmap: &[u8],
        color: Color,
    ) -> Result<(), Error> {
        if p.oob(self.size()) {
            return Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            });
        }

        let drawable_height = if p.y + size.y > self.size().y {
            self.size().y - p.y
        } else {
            size.y
        };
        let drawable_width = if p.x + size.x > self.size().x {
            self.size().x - p.x
        } else {
            size.x
        };

        for y in 0..drawable_height {
            for x in 0..drawable_width {
                if (bitmap[((x >> 3) + y * (size.x >> 3)) as usize] & (0x01 << (7 - (x & 7)))) != 0
                {
                    self.set_pixel(p + UVec2::new(x, y), color)?;
                }
            }
        }

        Ok(())
    }

    fn put_canvas(
        &mut self,
        other: &dyn Canvas,
        dest: UVec2,
        src: UVec2,
        size: UVec2,
    ) -> Result<(), Error> {
        if dest.oob(self.size()) {
            return Err(Error::OutOfBounds {
                p: dest,
                bounds: self.size(),
            });
        }
        if src.oob(other.size()) {
            return Err(Error::OutOfBounds {
                p: src,
                bounds: other.size(),
            });
        }

        let w = if size.x <= other.size().x && size.x > 0 {
            size.x
        } else {
            other.size().x
        };
        let h = if size.y <= other.size().y && size.y > 0 {
            size.y
        } else {
            other.size().y
        };

        let drawable_height = if dest.y + h > self.size().y {
            self.size().y - dest.y
        } else {
            h
        };
        let drawable_width = if dest.x + w > self.size().x {
            self.size().x - dest.x
        } else {
            w
        };

        for y in 0..drawable_height {
            for x in 0..drawable_width {
                self.set_pixel(
                    dest + UVec2::new(x, y),
                    other.get_pixel(src + UVec2::new(x, y))?,
                )?;
            }
        }

        Ok(())
    }

    fn put_character(
        &mut self,
        p: UVec2,
        character: char,
        color: Color,
        font: &Font,
    ) -> Result<(), Error> {
        if p.oob(self.size()) {
            return Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            });
        }

        let character = font.get_char(character)?;

        for y in 0..character.size().y {
            for x in 0..character.size().x {
                let offset = UVec2::new(x, y);
                if character.get_pixel(offset) {
                    self.set_pixel(p + offset, color)?;
                }
            }
        }

        Ok(())
    }
    fn put_text(
        &mut self,
        p: UVec2,
        text: String,
        color: Color,
        font: &Font,
        spacing: u32,
    ) -> Result<(), Error> {
        let char_width = font.char_spacing() + spacing;

        if p.oob(self.size()) {
            return Err(Error::OutOfBounds {
                p,
                bounds: self.size(),
            });
        }

        let mut p = p;
        for c in text.chars() {
            if p.x > self.size().x + char_width {
                return Err(Error::OutOfBounds {
                    p,
                    bounds: self.size(),
                });
            }
            self.put_character(p, c, color, font)?;
            p.x += char_width;
        }

        Ok(())
    }
}
