

use num_enum::{IntoPrimitive, FromPrimitive};

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum PrefixedInstruction {
    // * 0x0_
    /// Rotate register `B` left, setting the carry flag to the previous bit 7.
    RLC_B = 0x00,
    /// Rotate register `C` left, setting the carry flag to the previous bit 7.
    RLC_C,
    /// Rotate register `D` left, setting the carry flag to the previous bit 7.
    RLC_D,
    /// Rotate register `E` left, setting the carry flag to the previous bit 7.
    RLC_E,
    /// Rotate register `H` left, setting the carry flag to the previous bit 7.
    RLC_H,
    /// Rotate register `L` left, setting the carry flag to the previous bit 7.
    RLC_L,
    /// Rotate the value at address `HL` left, setting the carry flag to the previous bit 7.
    RLC_HL,
    /// Rotate register `A` left, setting the carry flag to the previous bit 7.
    RLC_A,
    /// Rotate register `B` right, setting the carry flag to the previous bit 0.
    RRC_B,
    /// Rotate register `C` right, setting the carry flag to the previous bit 0.
    RRC_C,
    /// Rotate register `D` right, setting the carry flag to the previous bit 0.
    RRC_D,
    /// Rotate register `E` right, setting the carry flag to the previous bit 0.
    RRC_E,
    /// Rotate register `H` right, setting the carry flag to the previous bit 0.
    RRC_H,
    /// Rotate register `L` right, setting the carry flag to the previous bit 0.
    RRC_L,
    /// Rotate the value at address `HL` right, setting the carry flag to the previous bit 0.
    RRC_HL,
    /// Rotate register `A` right, setting the carry flag to the previous bit 0.
    RRC_A,
    // * 0x1_
    /// Rotate register `B` and the carry flag left.
    RL_B,
    /// Rotate register `C` and the carry flag left.
    RL_C,
    /// Rotate register `D` and the carry flag left.
    RL_D,
    /// Rotate register `E` and the carry flag left.
    RL_E,
    /// Rotate register `H` and the carry flag left.
    RL_H,
    /// Rotate register `L` and the carry flag left.
    RL_L,
    /// Rotate the value at address `HL` and the carry flag left.
    RL_HL,
    /// Rotate register `A` and the carry flag left.
    RL_A,
    /// Rotate register `B` and the carry flag right.
    RR_B,
    /// Rotate register `C` and the carry flag right.
    RR_C,
    /// Rotate register `D` and the carry flag right.
    RR_D,
    /// Rotate register `E` and the carry flag right.
    RR_E,
    /// Rotate register `H` and the carry flag right.
    RR_H,
    /// Rotate register `L` and the carry flag right.
    RR_L,
    /// Rotate the value at address `HL` and the carry flag right.
    RR_HL,
    /// Rotate register `A` and the carry flag right.
    RR_A,
    // * 0x2_
    /// Shift register `B` left arithmetically.
    SLA_B,
    /// Shift register `C` left arithmetically.
    SLA_C,
    /// Shift register `D` left arithmetically.
    SLA_D,
    /// Shift register `E` left arithmetically.
    SLA_E,
    /// Shift register `H` left arithmetically.
    SLA_H,
    /// Shift register `L` left arithmetically.
    SLA_L,
    /// Shift the value at address `HL` left arithmetically.
    SLA_HL,
    /// Shift register `A` left arithmetically.
    SLA_A,
    /// Shift register `B` right arithmetically.
    SRA_B,
    /// Shift register `C` right arithmetically.
    SRA_C,
    /// Shift register `D` right arithmetically.
    SRA_D,
    /// Shift register `E` right arithmetically.
    SRA_E,
    /// Shift register `H` right arithmetically.
    SRA_H,
    /// Shift register `L` right arithmetically.
    SRA_L,
    /// Shift the value at address `HL` right arithmetically.
    SRA_HL,
    /// Shift register `A` right arithmetically.
    SRA_A,
    // * 0x3_
    /// Swap the upper and lower 4 bits of register `B`
    SWAP_B,
    /// Swap the upper and lower 4 bits of register `C`
    SWAP_C,
    /// Swap the upper and lower 4 bits of register `D`
    SWAP_D,
    /// Swap the upper and lower 4 bits of register `E`
    SWAP_E,
    /// Swap the upper and lower 4 bits of register `H`
    SWAP_H,
    /// Swap the upper and lower 4 bits of register `L`
    SWAP_L,
    /// Swap the upper and lower 4 bits of the value at address `HL`
    SWAP_HL,
    /// Swap the upper and lower 4 bits of register `A`
    SWAP_A,
    /// Shift register `B` right logically.
    SRL_B,
    /// Shift register `C` right logically.
    SRL_C,
    /// Shift register `D` right logically.
    SRL_D,
    /// Shift register `E` right logically.
    SRL_E,
    /// Shift register `H` right logically.
    SRL_H,
    /// Shift register `L` right logically.
    SRL_L,
    /// Shift the value at address `HL` right logically.
    SRL_HL,
    /// Shift register `A` right logically.
    SRL_A,
    // * 0x4_
    /// Set the zero flag if bit 0 of register `B` is not set.
    BIT_0_B,
    /// Set the zero flag if bit 0 of register `C` is not set.
    BIT_0_C,
    /// Set the zero flag if bit 0 of register `D` is not set.
    BIT_0_D,
    /// Set the zero flag if bit 0 of register `E` is not set.
    BIT_0_E,
    /// Set the zero flag if bit 0 of register `H` is not set.
    BIT_0_H,
    /// Set the zero flag if bit 0 of register `L` is not set.
    BIT_0_L,
    /// Set the zero flag if bit 0 of the value at address `HL` is not set.
    BIT_0_HL,
    /// Set the zero flag if bit 0 of register `A` is not set.
    BIT_0_A,
    /// Set the zero flag if bit 1 of register `B` is not set.
    BIT_1_B,
    /// Set the zero flag if bit 1 of register `C` is not set.
    BIT_1_C,
    /// Set the zero flag if bit 1 of register `D` is not set.
    BIT_1_D,
    /// Set the zero flag if bit 1 of register `E` is not set.
    BIT_1_E,
    /// Set the zero flag if bit 1 of register `H` is not set.
    BIT_1_H,
    /// Set the zero flag if bit 1 of register `L` is not set.
    BIT_1_L,
    /// Set the zero flag if bit 1 of the value at address `HL` is not set.
    BIT_1_HL,
    /// Set the zero flag if bit 1 of register `A` is not set.
    BIT_1_A,
    // * 0x5_
    /// Set the zero flag if bit 2 of register `B` is not set.
    BIT_2_B,
    /// Set the zero flag if bit 2 of register `C` is not set.
    BIT_2_C,
    /// Set the zero flag if bit 2 of register `D` is not set.
    BIT_2_D,
    /// Set the zero flag if bit 2 of register `E` is not set.
    BIT_2_E,
    /// Set the zero flag if bit 2 of register `H` is not set.
    BIT_2_H,
    /// Set the zero flag if bit 2 of register `L` is not set.
    BIT_2_L,
    /// Set the zero flag if bit 2 of the value at address `HL` is not set.
    BIT_2_HL,
    /// Set the zero flag if bit 2 of register `A` is not set.
    BIT_2_A,
    /// Set the zero flag if bit 3 of register `B` is not set.
    BIT_3_B,
    /// Set the zero flag if bit 3 of register `C` is not set.
    BIT_3_C,
    /// Set the zero flag if bit 3 of register `D` is not set.
    BIT_3_D,
    /// Set the zero flag if bit 3 of register `E` is not set.
    BIT_3_E,
    /// Set the zero flag if bit 3 of register `H` is not set.
    BIT_3_H,
    /// Set the zero flag if bit 3 of register `L` is not set.
    BIT_3_L,
    /// Set the zero flag if bit 3 of the value at address `HL` is not set.
    BIT_3_HL,
    /// Set the zero flag if bit 3 of register `A` is not set.
    BIT_3_A,
    // * 0x6_
    /// Set the zero flag if bit 4 of register `B` is not set.
    BIT_4_B,
    /// Set the zero flag if bit 4 of register `C` is not set.
    BIT_4_C,
    /// Set the zero flag if bit 4 of register `D` is not set.
    BIT_4_D,
    /// Set the zero flag if bit 4 of register `E` is not set.
    BIT_4_E,
    /// Set the zero flag if bit 4 of register `H` is not set.
    BIT_4_H,
    /// Set the zero flag if bit 4 of register `L` is not set.
    BIT_4_L,
    /// Set the zero flag if bit 4 of the value at address `HL` is not set.
    BIT_4_HL,
    /// Set the zero flag if bit 4 of register `A` is not set.
    BIT_4_A,
    /// Set the zero flag if bit 5 of register `B` is not set.
    BIT_5_B,
    /// Set the zero flag if bit 5 of register `C` is not set.
    BIT_5_C,
    /// Set the zero flag if bit 5 of register `D` is not set.
    BIT_5_D,
    /// Set the zero flag if bit 5 of register `E` is not set.
    BIT_5_E,
    /// Set the zero flag if bit 5 of register `H` is not set.
    BIT_5_H,
    /// Set the zero flag if bit 5 of register `L` is not set.
    BIT_5_L,
    /// Set the zero flag if bit 5 of the value at address `HL` is not set.
    BIT_5_HL,
    /// Set the zero flag if bit 5 of register `A` is not set.
    BIT_5_A,
    // * 0x7_
    /// Set the zero flag if bit 6 of register `B` is not set.
    BIT_6_B,
    /// Set the zero flag if bit 6 of register `C` is not set.
    BIT_6_C,
    /// Set the zero flag if bit 6 of register `D` is not set.
    BIT_6_D,
    /// Set the zero flag if bit 6 of register `E` is not set.
    BIT_6_E,
    /// Set the zero flag if bit 6 of register `H` is not set.
    BIT_6_H,
    /// Set the zero flag if bit 6 of register `L` is not set.
    BIT_6_L,
    /// Set the zero flag if bit 6 of the value at address `HL` is not set.
    BIT_6_HL,
    /// Set the zero flag if bit 6 of register `A` is not set.
    BIT_6_A,
    /// Set the zero flag if bit 7 of register `B` is not set.
    BIT_7_B,
    /// Set the zero flag if bit 7 of register `C` is not set.
    BIT_7_C,
    /// Set the zero flag if bit 7 of register `D` is not set.
    BIT_7_D,
    /// Set the zero flag if bit 7 of register `E` is not set.
    BIT_7_E,
    /// Set the zero flag if bit 7 of register `H` is not set.
    BIT_7_H,
    /// Set the zero flag if bit 7 of register `L` is not set.
    BIT_7_L,
    /// Set the zero flag if bit 7 of the value at address `HL` is not set.
    BIT_7_HL,
    /// Set the zero flag if bit 7 of register `A` is not set.
    BIT_7_A,
    // * 0x8_
    /// Set bit 0 of register `B` to 0.
    RES_0_B,
    /// Set bit 0 of register `C` to 0.
    RES_0_C,
    /// Set bit 0 of register `D` to 0.
    RES_0_D,
    /// Set bit 0 of register `E` to 0.
    RES_0_E,
    /// Set bit 0 of register `H` to 0.
    RES_0_H,
    /// Set bit 0 of register `L` to 0.
    RES_0_L,
    /// Set bit 0 of the value at address `HL` to 0.
    RES_0_HL,
    /// Set bit 0 of register `A` to 0.
    RES_0_A,
    /// Set bit 1 of register `B` to 0.
    RES_1_B,
    /// Set bit 1 of register `C` to 0.
    RES_1_C,
    /// Set bit 1 of register `D` to 0.
    RES_1_D,
    /// Set bit 1 of register `E` to 0.
    RES_1_E,
    /// Set bit 1 of register `H` to 0.
    RES_1_H,
    /// Set bit 1 of register `L` to 0.
    RES_1_L,
    /// Set bit 1 of the value at address `HL` to 0.
    RES_1_HL,
    /// Set bit 1 of register `A` to 0.
    RES_1_A,
    // * 0x9_
    /// Set bit 2 of register `B` to 0.
    RES_2_B,
    /// Set bit 2 of register `C` to 0.
    RES_2_C,
    /// Set bit 2 of register `D` to 0.
    RES_2_D,
    /// Set bit 2 of register `E` to 0.
    RES_2_E,
    /// Set bit 2 of register `H` to 0.
    RES_2_H,
    /// Set bit 2 of register `L` to 0.
    RES_2_L,
    /// Set bit 2 of the value at address `HL` to 0.
    RES_2_HL,
    /// Set bit 2 of register `A` to 0.
    RES_2_A,
    /// Set bit 3 of register `B` to 0.
    RES_3_B,
    /// Set bit 3 of register `C` to 0.
    RES_3_C,
    /// Set bit 3 of register `D` to 0.
    RES_3_D,
    /// Set bit 3 of register `E` to 0.
    RES_3_E,
    /// Set bit 3 of register `H` to 0.
    RES_3_H,
    /// Set bit 3 of register `L` to 0.
    RES_3_L,
    /// Set bit 3 of the value at address `HL` to 0.
    RES_3_HL,
    /// Set bit 3 of register `A` to 0.
    RES_3_A,
    // * 0xA_
    /// Set bit 4 of register `B` to 0.
    RES_4_B,
    /// Set bit 4 of register `C` to 0.
    RES_4_C,
    /// Set bit 4 of register `D` to 0.
    RES_4_D,
    /// Set bit 4 of register `E` to 0.
    RES_4_E,
    /// Set bit 4 of register `H` to 0.
    RES_4_H,
    /// Set bit 4 of register `L` to 0.
    RES_4_L,
    /// Set bit 4 of the value at address `HL` to 0.
    RES_4_HL,
    /// Set bit 4 of register `A` to 0.
    RES_4_A,
    /// Set bit 5 of register `B` to 0.
    RES_5_B,
    /// Set bit 5 of register `C` to 0.
    RES_5_C,
    /// Set bit 5 of register `D` to 0.
    RES_5_D,
    /// Set bit 5 of register `E` to 0.
    RES_5_E,
    /// Set bit 5 of register `H` to 0.
    RES_5_H,
    /// Set bit 5 of register `L` to 0.
    RES_5_L,
    /// Set bit 5 of the value at address `HL` to 0.
    RES_5_HL,
    /// Set bit 5 of register `A` to 0.
    RES_5_A,
    // * 0xB_
    /// Set bit 6 of register `B` to 0.
    RES_6_B,
    /// Set bit 6 of register `C` to 0.
    RES_6_C,
    /// Set bit 6 of register `D` to 0.
    RES_6_D,
    /// Set bit 6 of register `E` to 0.
    RES_6_E,
    /// Set bit 6 of register `H` to 0.
    RES_6_H,
    /// Set bit 6 of register `L` to 0.
    RES_6_L,
    /// Set bit 6 of the value at address `HL` to 0.
    RES_6_HL,
    /// Set bit 6 of register `A` to 0.
    RES_6_A,
    /// Set bit 7 of register `B` to 0.
    RES_7_B,
    /// Set bit 7 of register `C` to 0.
    RES_7_C,
    /// Set bit 7 of register `D` to 0.
    RES_7_D,
    /// Set bit 7 of register `E` to 0.
    RES_7_E,
    /// Set bit 7 of register `H` to 0.
    RES_7_H,
    /// Set bit 7 of register `L` to 0.
    RES_7_L,
    /// Set bit 7 of the value at address `HL` to 0.
    RES_7_HL,
    /// Set bit 7 of register `A` to 0.
    RES_7_A,
    // * 0xC_
    /// Set bit 0 of register `B` to 1.
    SET_0_B,
    /// Set bit 0 of register `C` to 1.
    SET_0_C,
    /// Set bit 0 of register `D` to 1.
    SET_0_D,
    /// Set bit 0 of register `E` to 1.
    SET_0_E,
    /// Set bit 0 of register `H` to 1.
    SET_0_H,
    /// Set bit 0 of register `L` to 1.
    SET_0_L,
    /// Set bit 0 of the value at address `HL` to 1.
    SET_0_HL,
    /// Set bit 0 of register `A` to 1.
    SET_0_A,
    /// Set bit 1 of register `B` to 1.
    SET_1_B,
    /// Set bit 1 of register `C` to 1.
    SET_1_C,
    /// Set bit 1 of register `D` to 1.
    SET_1_D,
    /// Set bit 1 of register `E` to 1.
    SET_1_E,
    /// Set bit 1 of register `H` to 1.
    SET_1_H,
    /// Set bit 1 of register `L` to 1.
    SET_1_L,
    /// Set bit 1 of the value at address `HL` to 1.
    SET_1_HL,
    /// Set bit 1 of register `A` to 1.
    SET_1_A,
    // * 0xD_
    /// Set bit 2 of register `B` to 1.
    SET_2_B,
    /// Set bit 2 of register `C` to 1.
    SET_2_C,
    /// Set bit 2 of register `D` to 1.
    SET_2_D,
    /// Set bit 2 of register `E` to 1.
    SET_2_E,
    /// Set bit 2 of register `H` to 1.
    SET_2_H,
    /// Set bit 2 of register `L` to 1.
    SET_2_L,
    /// Set bit 2 of the value at address `HL` to 1.
    SET_2_HL,
    /// Set bit 2 of register `A` to 1.
    SET_2_A,
    /// Set bit 3 of register `B` to 1.
    SET_3_B,
    /// Set bit 3 of register `C` to 1.
    SET_3_C,
    /// Set bit 3 of register `D` to 1.
    SET_3_D,
    /// Set bit 3 of register `E` to 1.
    SET_3_E,
    /// Set bit 3 of register `H` to 1.
    SET_3_H,
    /// Set bit 3 of register `L` to 1.
    SET_3_L,
    /// Set bit 3 of the value at address `HL` to 1.
    SET_3_HL,
    /// Set bit 3 of register `A` to 1.
    SET_3_A,
    // * 0xE_
    /// Set bit 4 of register `B` to 1.
    SET_4_B,
    /// Set bit 4 of register `C` to 1.
    SET_4_C,
    /// Set bit 4 of register `D` to 1.
    SET_4_D,
    /// Set bit 4 of register `E` to 1.
    SET_4_E,
    /// Set bit 4 of register `H` to 1.
    SET_4_H,
    /// Set bit 4 of register `L` to 1.
    SET_4_L,
    /// Set bit 4 of the value at address `HL` to 1.
    SET_4_HL,
    /// Set bit 4 of register `A` to 1.
    SET_4_A,
    /// Set bit 5 of register `B` to 1.
    SET_5_B,
    /// Set bit 5 of register `C` to 1.
    SET_5_C,
    /// Set bit 5 of register `D` to 1.
    SET_5_D,
    /// Set bit 5 of register `E` to 1.
    SET_5_E,
    /// Set bit 5 of register `H` to 1.
    SET_5_H,
    /// Set bit 5 of register `L` to 1.
    SET_5_L,
    /// Set bit 5 of the value at address `HL` to 1.
    SET_5_HL,
    /// Set bit 5 of register `A` to 1.
    SET_5_A,
    // * 0xF_
    /// Set bit 6 of register `B` to 1.
    SET_6_B,
    /// Set bit 6 of register `C` to 1.
    SET_6_C,
    /// Set bit 6 of register `D` to 1.
    SET_6_D,
    /// Set bit 6 of register `E` to 1.
    SET_6_E,
    /// Set bit 6 of register `H` to 1.
    SET_6_H,
    /// Set bit 6 of register `L` to 1.
    SET_6_L,
    /// Set bit 6 of the value at address `HL` to 1.
    SET_6_HL,
    /// Set bit 6 of register `A` to 1.
    SET_6_A,
    /// Set bit 7 of register `B` to 1.
    SET_7_B,
    /// Set bit 7 of register `C` to 1.
    SET_7_C,
    /// Set bit 7 of register `D` to 1.
    SET_7_D,
    /// Set bit 7 of register `E` to 1.
    SET_7_E,
    /// Set bit 7 of register `H` to 1.
    SET_7_H,
    /// Set bit 7 of register `L` to 1.
    SET_7_L,
    /// Set bit 7 of the value at address `HL` to 1.
    SET_7_HL,
    /// Set bit 7 of register `A` to 1.
    SET_7_A,
}