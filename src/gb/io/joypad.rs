use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

use crate::gb::{utils::{get_bit, set_bit, InterruptMask}, emu::GameboyEmulator};

/// Info from the [Open Game Boy Documentation Project](https://mgba-emu.github.io/gbdoc/#mmio-p1).
#[derive(Debug)]
pub struct JoypadRegisters {
    /// A `u8` that determines whether each of the player's inputs are currently being pressed (set to 0) or not (set to 1).
    /// * bit 0: A / Right
    /// * bit 1: B / Left
    /// * bit 2: Select / Up
    /// * bit 3: Start / Down
    /// * bit 4: Directional inputs are read.
    /// * bit 5: Nondirectional inputs are read.
    pub input_state: u8,
    pub key_binds: KeyBinds,
}

impl JoypadRegisters {
    pub fn new() -> Self {
        Self {
            input_state: 0b0000_1111,
            key_binds: KeyBinds::default(),
        }
    }

    pub fn update(emu: &mut GameboyEmulator, input: &mut WinitInputHelper) {
        let mut new_state = emu.io_registers.joypad.input_state | 0xF;

        if !get_bit(new_state, 0b0001_0000) {
            new_state &= emu.io_registers.joypad.key_binds.get_directional(input);
        }
        if !get_bit(new_state, 0b0010_0000) {
            new_state &= emu.io_registers.joypad.key_binds.get_nondirectional(input);
        }

        // ? Joypad interrupt if any bits 0 to 3 goes from 1 to 0 (gets activated).
        if emu.io_registers.joypad.input_state & !new_state != 0 {
            emu.set_interrupt_flag(InterruptMask::Joypad, true);
        }

        emu.io_registers.joypad.input_state = new_state;
    }

    pub fn write(&mut self, value: u8) {
        set_bit(&mut self.input_state, 0b0001_0000, get_bit(value, 0b0001_0000));
        set_bit(&mut self.input_state, 0b0010_0000, get_bit(value, 0b0010_0000));
    }
}

#[derive(Debug)]
pub struct KeyBinds {
    pub button_a: KeyCode,
    pub button_b: KeyCode,
    pub button_select: KeyCode,
    pub button_start: KeyCode,

    pub button_right: KeyCode,
    pub button_left: KeyCode,
    pub button_up: KeyCode,
    pub button_down: KeyCode,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            button_a: KeyCode::KeyQ,
            button_b: KeyCode::KeyE,
            button_select: KeyCode::KeyZ,
            button_start: KeyCode::KeyX,

            button_right: KeyCode::KeyD,
            button_left: KeyCode::KeyA,
            button_up: KeyCode::KeyW,
            button_down: KeyCode::KeyS,
        }
    }
}

impl KeyBinds {
    #[inline]
    pub fn get_directional(&self, input: &mut WinitInputHelper) -> u8 {
        (input.key_held(self.button_right) as u8) |
        (input.key_held(self.button_left) as u8) << 1 |
        (input.key_held(self.button_up) as u8) << 2 |
        (input.key_held(self.button_down) as u8) << 3
    }

    #[inline]
    pub fn get_nondirectional(&self, input: &mut WinitInputHelper) -> u8 {
        (input.key_held(self.button_a) as u8) |
        (input.key_held(self.button_b) as u8) << 1 |
        (input.key_held(self.button_select) as u8) << 2 |
        (input.key_held(self.button_start) as u8) << 3
    }
}