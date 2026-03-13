use crate::{Chip8, Drawing, Drawing::*};

impl Chip8 {
    pub(super) fn execute_drawing_instruction(&mut self, instruction: Drawing) {
        match instruction {
            ClearScreen => {
                for row in self.frame_buffer.iter_mut() {
                    for pixel in row {
                        *pixel = 0x00;
                    }
                }
            }
            DrawSpriteToScreen {
                x_register,
                y_register,
                n_rows,
            } => {
                let x_start = self.general_registers[x_register as usize];
                let y_start = self.general_registers[y_register as usize];
                let height = self.frame_buffer.len();
                let width = self.frame_buffer[0].len();
                for row in 0..n_rows {
                    let sprite_row = self.memory[(self.index_register as usize) + (row as usize)];
                    for bit_index in 0..8 {
                        let bit = (sprite_row & (1 << (7 - bit_index))) >> (7 - bit_index);

                        let curr_pixel = self.frame_buffer
                            [(y_start as usize + row as usize) % height]
                            [(x_start as usize + bit_index) % width];

                        if curr_pixel ^ bit == 0 && curr_pixel != 0 {
                            self.general_registers[self.general_registers.len() - 1] = 1;
                        }

                        self.frame_buffer[(y_start as usize + row as usize) % height]
                            [(x_start as usize + bit_index) % width] ^= bit;
                    }
                }
            }
        }
    }
}
