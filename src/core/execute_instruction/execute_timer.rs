use std::time::Instant;

use crate::{Chip8, Timer, Timer::*};

impl Chip8 {
    pub(super) fn execute_timer_instruction(&mut self, instruction: Timer) {
        match instruction {
            LoadDelayTimerToRegisterX { x_register } => {
                self.set_register_value(x_register, self.delay_register);
            }
            LoadRegisterXToDelayTimer { x_register } => {
                self.delay_register = self.get_register_value(x_register);
                self.delay_timer_reference = Some(Instant::now());
            }
            LoadRegisterXToSoundTimer { x_register } => {
                self.sound_timer = self.get_register_value(x_register);
                self.sound_timer_reference = Some(Instant::now());
            }
        }
    }
}
