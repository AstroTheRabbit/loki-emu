use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

use crate::gb::utils::{get_bit, set_bit};

/// Info from the [Open Game Boy Documentation Project](https://mgba-emu.github.io/gbdoc/#mmio-p1).
#[derive(Debug)]
pub struct JoypadRegisters {
    /// A `u8` that determines whether each of the player's inputs are currently being pressed (set to 0) or not (set to 1).
    /// * bit 0: A
    /// * bit 1: B
    /// * bit 2: Select
    /// * bit 3: Start
    /// * bit 4: Right
    /// * bit 5: Left
    /// * bit 6: Up
    /// * bit 7: Down
    input_state: u8,
    /// Determines whether the directional inputs are read.
    bit_4_low: bool,
    /// Determines whether the non-directional inputs are read.
    bit_5_low: bool,
}

impl JoypadRegisters {
    pub fn new() -> Self {
        Self {
            input_state: 0x00,
            bit_4_low: true,
            bit_5_low: true,
        }
    }

    pub fn update(&mut self, input: &mut WinitInputHelper) {
        // TODO: Keybinds settings.
        set_bit(
            &mut self.input_state,
            0b0000_0001,
            !input.key_held(KeyCode::KeyQ),
        );
        set_bit(
            &mut self.input_state,
            0b0000_0010,
            !input.key_held(KeyCode::KeyE),
        );
        set_bit(
            &mut self.input_state,
            0b0000_0100,
            !input.key_held(KeyCode::KeyZ),
        );
        set_bit(
            &mut self.input_state,
            0b0000_1000,
            !input.key_held(KeyCode::KeyX),
        );
        set_bit(
            &mut self.input_state,
            0b0001_0000,
            !input.key_held(KeyCode::KeyD),
        );
        set_bit(
            &mut self.input_state,
            0b0010_0000,
            !input.key_held(KeyCode::KeyA),
        );
        set_bit(
            &mut self.input_state,
            0b0100_0000,
            !input.key_held(KeyCode::KeyW),
        );
        set_bit(
            &mut self.input_state,
            0b1000_0000,
            !input.key_held(KeyCode::KeyS),
        );
    }

    pub fn read(&self) -> u8 {
        match self.bit_4_low {
            true => match self.bit_5_low {
                true => 0b1100_0000 | (self.get_directional() & self.get_nondirectional()),
                false => 0b1101_0000 | self.get_directional(),
            },
            false => match self.bit_5_low {
                true => 0b1110_0000 | self.get_nondirectional(),
                false => 0b1111_1111,
            },
        }
    }

    pub fn write(&mut self, value: u8) {
        self.bit_4_low = get_bit(value, 0b0001_0000);
        self.bit_5_low = get_bit(value, 0b0010_0000);
    }

    #[inline]
    fn get_directional(&self) -> u8 {
        self.input_state >> 4
    }

    #[inline]
    fn get_nondirectional(&self) -> u8 {
        self.input_state & 0x0F
    }
}
