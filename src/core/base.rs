use std::{
    fs::File,
    io::{Read, Result},
    time::Instant,
};

#[cfg(feature = "audio")]
use rodio::{
    MixerDeviceSink, Player,
    source::{SineWave, Source},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    ControlFlow(ControlFlow),
    Arithmetic(Arithmetic),
    DataTransfer(DataTransfer),
    Logical(Logical),
    Drawing(Drawing),
    Timer(Timer),
    Keyboard(Keyboard),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlFlow {
    CallSubroutine { address: u16 },
    ReturnFromSubroutine,
    SkipNextIfRegisterXEqualsImmediate { x_register: u8, value: u8 },
    SkipNextIfRegisterXNotEqualsImmediate { x_register: u8, value: u8 },
    SkipNextIfRegisterYNotEqualRegisterX { x_register: u8, y_register: u8 },
    SkipNextIfRegisterXEqualsRegisterY { x_register: u8, y_register: u8 },
    JumpToAddress { address: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arithmetic {
    AddImmediateToRegister { x_register: u8, value: u8 },
    AddRegisterXToIndex { x_register: u8 },
    AddRegisterYToRegisterX { x_register: u8, y_register: u8 },
    SubtractRegisterYFromRegisterX { x_register: u8, y_register: u8 },
    SubtractNRegisterXFromRegisterY { x_register: u8, y_register: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Logical {
    OrRegisterXWithRegisterY { x_register: u8, y_register: u8 },
    AndRegisterXWithRegisterY { x_register: u8, y_register: u8 },
    XorRegisterXWithRegisterY { x_register: u8, y_register: u8 },
    ShiftRegisterXRightWithRegisterY { x_register: u8, y_register: u8 },
    ShiftRegisterXLeftWithRegisterY { x_register: u8, y_register: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataTransfer {
    LoadImmediateToRegister { x_register: u8, value: u8 },
    LoadImmediateToIndexRegister { address: u16 },
    LoadRegisterYToRegisterX { x_register: u8, y_register: u8 },
    StoreBcdOfRegisterXAtIndex { x_register: u8 },
    SaveNumRegistersToImediate { n_registers: u8 },
    SaveImmediateToNumRegisters { n_registers: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Drawing {
    ClearScreen,
    DrawSpriteToScreen {
        x_register: u8,
        y_register: u8,
        n_rows: u8,
    },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Timer {
    LoadDelayTimerToRegisterX { x_register: u8 },
    LoadRegisterXToDelayTimer { x_register: u8 },
    LoadRegisterXToSoundTimer { x_register: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyboard {
    SkipIfKeyInRegisterXIsPressed { x_register: u8 },
    SkipIfKeyInRegisterXIsNotPressed { x_register: u8 },
    WaitUntilKeyIsPressedPressed { x_register: u8 },
}

pub struct Chip8 {
    pub current_instruction: Option<Instruction>,
    pub delay_register: u8,
    pub frame_buffer: [[u8; 64]; 32],
    pub general_registers: [u8; 16],
    pub index_register: u16,
    pub memory: [u8; 4096],
    pub program_counter: u16,
    pub stack_memory: [u16; 16],
    pub stack_pointer: u8,
    pub sound_timer: u8,

    pub delay_timer_reference: Option<Instant>,
    pub sound_timer_reference: Option<Instant>,

    // Stores whether 0-9 and A-F is pressed
    pub(super) key_states: [bool; 16],

    pub(super) pending_key_press_dest: Option<u8>,

    #[cfg(feature = "audio")]
    beep_player: Option<Player>,

    #[cfg(feature = "audio")]
    _handle: Option<MixerDeviceSink>,
}

const SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // '0'
    [0x20, 0x60, 0x20, 0x20, 0x70], // '1'
    [0xF0, 0x10, 0xf0, 0x80, 0xf0], // '2'
    [0xF0, 0x10, 0xf0, 0x10, 0xF0], // '3'
    [0x90, 0x90, 0xf0, 0x10, 0x10], // '4'
    [0xF0, 0x80, 0xf0, 0x10, 0xf0], // '5'
    [0xF0, 0x80, 0xf0, 0x90, 0xF0], // '6'
    [0x70, 0x10, 0x20, 0x40, 0x40], // '7'
    [0xF0, 0x90, 0xf0, 0x90, 0xF0], // '8'
    [0xF0, 0x90, 0xf0, 0x10, 0xF0], // '9'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'A'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'B'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'C'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'D'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'E'
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 'F'
];

pub const PROGRAM_START_OFFSET: usize = 0x200;

// Prevent checks in CI from failing, we don't need audio to play there
#[cfg(feature = "audio")]
fn initialize_audio() -> (Option<MixerDeviceSink>, Option<Player>) {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    handle.log_on_drop(false);
    let player = rodio::Player::connect_new(&handle.mixer());
    let source = SineWave::new(440.0).repeat_infinite().amplify(0.20);
    player.append(source);
    player.pause();

    (Some(handle), Some(player))
}

impl Chip8 {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        let (handle, player) = initialize_audio();

        let mut instance = Self {
            current_instruction: None,
            delay_register: 0,
            frame_buffer: [[0; 64]; 32],
            general_registers: [0; 16],
            index_register: 0,
            pending_key_press_dest: None,
            key_states: [false; 16],
            memory: [0; 4096],
            program_counter: PROGRAM_START_OFFSET as u16,
            stack_memory: [0; 16],
            stack_pointer: 0,
            sound_timer: 0,
            delay_timer_reference: None,
            sound_timer_reference: None,

            #[cfg(feature = "audio")]
            beep_player: player,

            #[cfg(feature = "audio")]
            _handle: handle,
        };

        SPRITES.iter().enumerate().for_each(|(index, item)| {
            let start = index * item.len();
            let end = start + item.len();
            instance.memory[start..end].copy_from_slice(item);
        });

        instance
    }

    pub fn new_from_program(insructions: &[u16]) -> Self {
        let mut instance = Self::new();

        for (index, &instr) in insructions.iter().enumerate() {
            let bytes = instr.to_be_bytes();
            let start = index * 2;

            let destinations = &mut instance.memory[PROGRAM_START_OFFSET as usize + start..];
            destinations[0..2].copy_from_slice(&bytes);
        }

        instance
    }

    pub fn new_from_program_file(program_path: &str) -> Result<Self> {
        let mut instance = Self::new();
        instance.load_program(program_path)?;
        Ok(instance)
    }

    pub fn load_program(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        file.read(&mut self.memory[(PROGRAM_START_OFFSET as usize)..])?;
        Ok(())
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        let start = self.program_counter as usize;
        let bytes = &self.memory[start..start + 2];

        let instruction = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        self.program_counter += 2;
        return instruction;
    }

    pub fn tick(&mut self) -> Result<()> {
        let opcode = self.fetch_instruction();
        let instruction = self.decode_instruction(opcode)?;
        self.current_instruction = Some(instruction);
        self.execute_instruction(instruction);
        self.handle_timers()?;
        Ok(())
    }

    fn handle_timers(&mut self) -> Result<()> {
        if self.delay_register == 0 && self.sound_timer == 0 {
            return Ok(());
        }

        self.handle_delay_timer();
        self.handle_sound_timer()?;
        Ok(())
    }

    fn handle_delay_timer(&mut self) {
        if self.delay_register == 0 {
            return;
        }
        if let Some(start) = self.delay_timer_reference {
            let elapsed = start.elapsed();
            let num_ticks_passed = (elapsed.as_secs_f32() * 60.0).round() as u8;
            self.delay_register = self
                .delay_register
                .checked_sub(num_ticks_passed)
                .unwrap_or(0);

            if self.delay_register == 0 {
                self.delay_timer_reference = None;
            }
        }
    }

    #[cfg(feature = "audio")]
    fn play_sound(&self) {
        if let Some(player) = &self.beep_player {
            if self.sound_timer > 0 {
                player.play();
            } else {
                player.pause();
            }
        }
    }

    #[cfg(not(feature = "audio"))]
    fn play_sound(&self) {}

    fn handle_sound_timer(&mut self) -> Result<()> {
        if self.sound_timer == 0 {
            return Ok(());
        }
        if let Some(start) = self.sound_timer_reference {
            let elapsed = start.elapsed();
            let num_ticks_passed = (elapsed.as_secs_f32() * 60.0).round() as u8;
            self.sound_timer = self.sound_timer.checked_sub(num_ticks_passed).unwrap_or(0);

            self.play_sound();
        }
        Ok(())
    }

    pub(super) fn set_flag_register(&mut self, is_active: bool) {
        self.general_registers[self.general_registers.len() - 1] = if is_active { 1 } else { 0 };
    }

    pub(super) fn get_register_value(&self, index: u8) -> u8 {
        self.general_registers[index as usize]
    }

    pub(super) fn set_register_value(&mut self, index: u8, value: u8) {
        self.general_registers[index as usize] = value;
    }

    pub fn set_key_state(&mut self, index: u8, is_pressed: bool) {
        self.key_states[index as usize] = is_pressed;
    }

    pub fn get_key_state(&mut self, index: u8) -> bool {
        self.key_states[index as usize]
    }

    pub fn is_waiting_for_key_press(&mut self) -> bool {
        self.handle_delay_timer();
        self.pending_key_press_dest.is_some()
    }

    pub fn respond_to_key_press(&mut self, key: u8) {
        let destination = self
            .pending_key_press_dest
            .expect("Expecting respond without waiting beforehand");

        self.general_registers[destination as usize] = key;
        self.pending_key_press_dest = None;
    }
}
