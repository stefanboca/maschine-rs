use super::{error::Error, vec::UVec2};

pub struct Font;

pub struct Character;

impl Font {
    pub fn char_spacing(&self) -> u32 {
        todo!()
    }

    pub fn get_char(&self, character: char) -> Result<&Character, Error> {
        todo!()
    }
}

impl Character {
    pub fn size(&self) -> UVec2 {
        todo!()
    }

    pub fn get_pixel(&self, p: UVec2) -> bool {
        todo!()
    }
}
