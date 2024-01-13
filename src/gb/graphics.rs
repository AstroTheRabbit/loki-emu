use crate::byte_field;

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