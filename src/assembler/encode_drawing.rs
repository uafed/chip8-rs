use crate::Drawing;

pub fn encode_drawing(instruction: &Drawing) -> u16 {
    match *instruction {
        Drawing::ClearScreen => 0x00E0,
        Drawing::DrawSpriteToScreen {
            x_register,
            y_register,
            n_rows,
        } => 0xD000 | ((x_register as u16) << 8) | ((y_register as u16) << 4) | (n_rows as u16),
    }
}
