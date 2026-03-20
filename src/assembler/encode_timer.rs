use crate::Timer;

pub fn encode_timer(instruction: &Timer) -> u16 {
    match *instruction {
        Timer::LoadDelayTimerToRegisterX { x_register } => 0xF007 | ((x_register as u16) << 8),
        Timer::LoadRegisterXToSoundTimer { x_register } => 0xF015 | ((x_register as u16) << 8),
        Timer::LoadRegisterXToDelayTimer { x_register } => 0xF018 | ((x_register as u16) << 8),
    }
}
