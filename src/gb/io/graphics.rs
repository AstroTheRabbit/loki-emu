#![allow(non_snake_case)]

use crate::{
    byte_field,
    gb::{bus::Bus, emu::GameboyEmulator, utils::join_u16},
};

// ? Original color scheme
// ? 0b00: #0f380f
// ? 0b01: #306230
// ? 0b10: #8bac0f
// ? 0b11: #9bbc0f

byte_field! {
    #[derive(Debug)]
    pub VRAM;
    /// `0x8000..=0x97FF`
    pub tile_data: 6144,
    /// `0x9800..=0x9BFF`
    pub background_map_1: 1024,
    /// `0x9C00..=0x9FFF`
    pub background_map_2: 1024,
}

byte_field! {
    #[derive(Debug)]
    pub OAM;
    pub obj_0: 4,
    pub obj_1: 4,
    pub obj_2: 4,
    pub obj_3: 4,
    pub obj_4: 4,
    pub obj_5: 4,
    pub obj_6: 4,
    pub obj_7: 4,
    pub obj_8: 4,
    pub obj_9: 4,
    pub obj_10: 4,
    pub obj_11: 4,
    pub obj_12: 4,
    pub obj_13: 4,
    pub obj_14: 4,
    pub obj_15: 4,
    pub obj_16: 4,
    pub obj_17: 4,
    pub obj_18: 4,
    pub obj_19: 4,
    pub obj_20: 4,
    pub obj_21: 4,
    pub obj_22: 4,
    pub obj_23: 4,
    pub obj_24: 4,
    pub obj_25: 4,
    pub obj_26: 4,
    pub obj_27: 4,
    pub obj_28: 4,
    pub obj_29: 4,
    pub obj_30: 4,
    pub obj_31: 4,
    pub obj_32: 4,
    pub obj_33: 4,
    pub obj_34: 4,
    pub obj_35: 4,
    pub obj_36: 4,
    pub obj_37: 4,
    pub obj_38: 4,
    pub obj_39: 4,
}

#[derive(Debug)]
pub struct GraphicsRegisters {
    /// `0xFF40` - LCD control.
    pub LCDC: u8,
    /// `0xFF41` - LCD status.
    pub STAT: u8,
    /// `0xFF42` - Background vertical scroll.
    pub SCY: u8,
    /// `0xFF43` - Background horizontal scroll.
    pub SCX: u8,
    /// `0xFF44` - LCD Y coordinate.
    pub LY: u8,
    /// `0xFF45` - LCD Y compare.
    pub LYC: u8,
    /// `0xFF45` - OAM DMA source address.
    pub DMA: u8,
    /// `0xFF47` -  Background palette.
    pub BGP: u8,
    /// `0xFF48` -  Object palette 0.
    pub OBP0: u8,
    /// `0xFF49` -  Object palette 1.
    pub OBP1: u8,
    /// `0xFF4A` - Window Y coordinate.
    pub WY: u8,
    /// `0xFF4B` - Window X coordinate.
    pub WX: u8,

    /// `Some(index)` if transfer is in progress, `None` if not.
    pub DMA_transfer_progress: Option<u8>,
}

impl GraphicsRegisters {
    pub fn new() -> Self {
        Self {
            LCDC: 0x00,
            STAT: 0x00,
            SCY: 0x00,
            SCX: 0x00,
            LY: 0x00,
            LYC: 0x00,
            DMA: 0x00,
            BGP: 0x00,
            OBP0: 0x00,
            OBP1: 0x00,
            WY: 0x00,
            WX: 0x00,
            DMA_transfer_progress: None,
        }
    }

    pub fn update(emu: &mut GameboyEmulator) {
        if let Some(index) = emu.io_registers.graphics.DMA_transfer_progress {
            let value = Bus::read(emu, join_u16(index, emu.io_registers.graphics.DMA));
            Bus::write(emu, join_u16(index, 0xFE), value);
            emu.io_registers.graphics.DMA_transfer_progress = match index == 0x9F {
                true => None,
                false => Some(index + 1),
            };
        }
    }

    pub fn write_DMA(emu: &mut GameboyEmulator, value: u8) {
        emu.io_registers.graphics.DMA = value;
        emu.io_registers.graphics.DMA_transfer_progress = Some(0);
    }
}
