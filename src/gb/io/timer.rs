#![allow(non_snake_case)]

use crate::gb::{emu::GameboyEmulator, utils::*, bus::Bus};

#[derive(Debug, PartialEq)]
pub enum TIMAOverflowState {
    /// `TIMA` is incrementing as usual.
    NotOverflowed,
    /// `TIMA` has overflowed.
    Overflowed {
        /// The number of t-cycles that have passed since `TIMA` overflowed.
        cycles: u8,
    },
    /// `TIMA` is being set to `TMA` this t-cycle, and so it cannot be written to.
    SettingToTMA,
}

#[derive(Debug)]
pub struct TimerRegisters {
    /// `0xFF04` - Clock divider.
    pub DIV: u16,
    /// `0xFF05` - Timer value.
    pub TIMA: u8,
    /// `0xFF06` - Timer reload.
    pub TMA: u8,
    /// `0xFF07` - Timer control.
    pub TAC: u8,
    /// The previous AND result, used to determine when `TIMA` increments.
    pub prev_and_result: bool,
    pub TIMA_overflow_state: TIMAOverflowState,
}

impl TimerRegisters {
    pub fn new() -> Self {
        Self {
            DIV: 0,
            TIMA: 0,
            TMA: 0,
            TAC: 0,
            prev_and_result: false,
            TIMA_overflow_state: TIMAOverflowState::NotOverflowed,
        }
    }

    /// Update the timer as if 4 t-cycles have passed.
    pub fn update(emu: &mut GameboyEmulator) {
        // ? https://hacktix.github.io/GBEDG/timers/#timer-operation
        for _ in 0..4 {
            emu.io_registers.timer.DIV += 1;

            match &mut emu.io_registers.timer.TIMA_overflow_state {
                TIMAOverflowState::NotOverflowed => {
                    let bitmask = match emu.io_registers.timer.TAC & 0b0011 {
                        0b00 => 0b0010_0000_0000,
                        0b01 => 0b0000_0000_1000,
                        0b10 => 0b0000_0010_0000,
                        0b11 => 0b0000_1000_0000,
                        _ => panic!("GB - Timer: Invalid result?!"),
                    };
                    let div_bit = emu.io_registers.timer.DIV & bitmask != 0;
                    let timer_enable = get_bit(emu.io_registers.timer.TAC, 0b0100);
                    let and_result = div_bit & timer_enable;

                    if emu.io_registers.timer.prev_and_result == true && and_result == false {
                        let (tima, tima_overflow) = emu.io_registers.timer.TIMA.overflowing_add(1);
                        emu.io_registers.timer.TIMA = tima;
                        if tima_overflow {
                            emu.io_registers.timer.TIMA_overflow_state =
                                TIMAOverflowState::Overflowed { cycles: 0 };
                        }
                    }
                }
                TIMAOverflowState::Overflowed { cycles } => {
                    *cycles += 1;
                    if *cycles == 3 {
                        emu.io_registers.timer.TIMA_overflow_state =
                            TIMAOverflowState::SettingToTMA;
                    }
                }
                TIMAOverflowState::SettingToTMA => {
                    emu.io_registers.timer.TIMA = emu.io_registers.timer.TMA;
                    emu.io_registers.timer.TIMA_overflow_state = TIMAOverflowState::NotOverflowed;
                    let mut value = Bus::read(emu, 0xFF0F);
                    set_bit(&mut value, InterruptMask::Timer, true);
                    Bus::write(emu, 0xFF0F, value);
                }
            }
        }
    }

    #[inline]
    pub fn read_DIV(&self) -> u8 {
        return split_u16(self.DIV).1;
    }

    #[inline]
    pub fn write_DIV(&mut self) {
        self.DIV = 0;
    }

    #[inline]
    pub fn write_TIMA(&mut self, value: u8) {
        // ? Writes to TIMA ignored on the same t-cycle that TIMA is set to TMA after overflow.
        if self.TIMA_overflow_state != TIMAOverflowState::SettingToTMA {
            self.TIMA = value;
            self.TIMA_overflow_state = TIMAOverflowState::NotOverflowed;
        }
    }
}
