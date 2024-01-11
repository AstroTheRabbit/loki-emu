use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(non_camel_case_types)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Instruction {
    // * 0x0_
    /// No operation.
    NOP = 0x00,
    /// Load the immediate value `n16` into register pair `BC`.
    LD_BC_n16,
    /// Load the value of `A` into the location of address `BC`.
    LD_BC_A,
    /// Increment register pair `BC`.
    INC_BC,
    /// Increment register `B`.
    INC_B,
    /// Decrement register `B`.
    DEC_B,
    /// Load the immediate value `n8` into register `B`.
    LD_B_n8,
    /// Rotate register `A` left, setting the carry flag to the bit shifted out of the byte.
    RLCA,
    /// Load the value of register `SP` into the immediate address `a16`.
    LD_a16_SP,
    /// Add register pairs `HL` and `BC`, storing the result in `HL`.
    ADD_HL_BC,
    /// Load the value located at address `BC` into register `A`.
    LD_A_BC,
    /// Decrement register pair `BC`.
    DEC_BC,
    /// Increment register `C`.
    INC_C,
    /// Decrement register `C`.
    DEC_C,
    /// Load immediate value `n8` into register `C`.
    LD_C_n8,
    /// Rotate register `A` right, setting the carry flag to the bit shifted out of the byte.
    RRCA,

    // * 0x1_
    /// Stop the CPU & LCD display until a button is pressed (the byte after is also skipped).
    STOP,
    /// Load the immediate value `n16` into register pair `DE`.
    LD_DE_n16,
    /// Load the value of `A` into the location of address `DE`.
    LD_DE_A,
    /// Increment register pair `DE`.
    INC_DE,
    /// Increment register `D`.
    INC_D,
    /// Decrement register `D`.
    DEC_D,
    /// Load the immediate value `n8` into register `D`.
    LD_D_n8,
    /// Rotate register `A` left with the carry flag.
    RLA,
    /// Add the signed immediate value `e8` to the `PC` and jump to it.
    JR_e8,
    /// Add register pairs `HL` and `DE`, storing the result in `HL`.
    ADD_HL_DE,
    /// Load the value located at address `DE` into register `A`.
    LD_A_DE,
    /// Decrement register pair `DE`.
    DEC_DE,
    /// Increment register `D`.
    INC_E,
    /// Decrement register `D`.
    DEC_E,
    /// Load the immediate value `n8` into register `E`.
    LD_E_n8,
    /// Rotate register `A` right with the carry flag.
    RRA,

    // * 0x2_
    /// Add the signed immediate value `e8` to the `PC` and jump to it if the zero flag is not set.
    JR_NZ_e8,
    /// Load the immediate value `n16` into register pair `HL`.
    LD_HL_n16,
    /// Load the value of `A` into the location of address `HL`, then increment register pair `HL`.
    LDI_HL_A,
    /// Increment register pair `HL`.
    INC_HL,
    /// Increment register `H`.
    INC_H,
    /// Decrement register `H`.
    DEC_H,
    /// Load the immediate value `n8` into register `H`.
    LD_H_n8,
    /// Decimal Adjust Accumulator.
    DAA,
    /// Add the signed immediate value `e8` to the `PC` and jump to it if the zero flag is set.
    JR_Z_e8,
    /// Add register pair `HL` to itself, storing the result in `HL`.
    ADD_HL_HL,
    /// Load the value located at address `HL` into register `A`, then increment register pair `HL`.
    LDI_A_HL,
    /// Decrement register pair `HL`.
    DEC_HL,
    /// Increment register `L`.
    INC_L,
    /// Decrement register `L`.
    DEC_L,
    /// Load the immediate value `n8` into register `L`.
    LD_L_n8,
    /// Bitwise invert register `A`.
    CPL,

    // * 0x3_
    /// Add the signed immediate value `e8` to the `PC` and jump to it if the carry flag is not set.
    JR_NC_e8,
    /// Load the immdediate value `n16` into the `SP`.
    LD_SP_n16,
    /// Load the value of `A` into the location of address `HL`, then decrement register pair `HL`.
    LDD_HL_A,
    /// Increment the `SP`.
    INC_SP,
    /// Increment the value located at address `HL`.
    INCP_HL,
    /// Decrement the value located at address `HL`.
    DECP_HL,
    /// Load the immediate value `n8` into the value located at address `HL`.
    LD_HL_n8,
    /// Set the carry flag.
    SCF,
    /// Add the signed immediate value `e8` to the `PC` and jump to it if the carry flag is set.
    JR_C_e8,
    /// Add the `SP` to register pair `HL`.
    ADD_HL_SP,
    /// Load the value located at address `HL` into register `A`, then decrement register pair `HL`.
    LDD_A_HL,
    /// Decrement the `SP`.
    DEC_SP,
    /// Increment register `A`.
    INC_A,
    /// Decrement register `A`.
    DEC_A,
    /// Load the immediate value `n8` into register `A`.
    LD_A_n8,
    /// Invert the carry flag.
    CCF,

    // * 0x4_
    /// Load register `B` into itself.
    LD_B_B,
    /// Load register `C` into register `B`.
    LD_B_C,
    /// Load register `D` into register `B`.
    LD_B_D,
    /// Load register `E` into register `B`.
    LD_B_E,
    /// Load register `H` into register `B`.
    LD_B_H,
    /// Load register `L` into register `B`.
    LD_B_L,
    /// Load the value located at address `HL` into register `B`.
    LD_B_HL,
    /// Load register `A` into register `B`.
    LD_B_A,
    /// Load register `B` into register `C`.
    LD_C_B,
    /// Load register `C` into itself.
    LD_C_C,
    /// Load register `D` into register `C`.
    LD_C_D,
    /// Load register `E` into register `C`.
    LD_C_E,
    /// Load register `H` into register `C`.
    LD_C_H,
    /// Load register `L` into register `C`.
    LD_C_L,
    /// Load the value located at address `HL` into register `C`.
    LD_C_HL,
    /// Load register `A` into register `C`.
    LD_C_A,

    // * 0x5_
    /// Load register `B` into register `D`.
    LD_D_B,
    /// Load register `C` into register `D`.
    LD_D_C,
    /// Load register `D` into itself.
    LD_D_D,
    /// Load register `E` into register `D`.
    LD_D_E,
    /// Load register `H` into register `D`.
    LD_D_H,
    /// Load register `B` into register `D`.
    LD_D_L,
    /// Load the value located at address `HL` into register `D`.
    LD_D_HL,
    /// Load register `A` into register `D`.
    LD_D_A,
    /// Load register `B` into register `E`.
    LD_E_B,
    /// Load register `C` into register `E`.
    LD_E_C,
    /// Load register `D` into register `E`.
    LD_E_D,
    /// Load register `E` into itself.
    LD_E_E,
    /// Load register `H` into register `E`.
    LD_E_H,
    /// Load register `L` into register `E`.
    LD_E_L,
    /// Load the value located at address `HL` into register `E`.
    LD_E_HL,
    /// Load register `A` into register `E`.
    LD_E_A,

    // * 0x6_
    /// Load register `B` into register `H`.
    LD_H_B,
    /// Load register `C` into register `H`.
    LD_H_C,
    /// Load register `D` into register `H`.
    LD_H_D,
    /// Load register `E` into register `H`.
    LD_H_E,
    /// Load register `H` into itself.
    LD_H_H,
    /// Load register `L` into register `H`.
    LD_H_L,
    /// Load the value located at address `HL` into register `H`.
    LD_H_HL,
    /// Load register `A` into register `H`.
    LD_H_A,
    /// Load register `B` into register `L`.
    LD_L_B,
    /// Load register `C` into register `L`.
    LD_L_C,
    /// Load register `D` into register `L`.
    LD_L_D,
    /// Load register `E` into register `L`.
    LD_L_E,
    /// Load register `H` into register `L`.
    LD_L_H,
    /// Load register `L` into itself.
    LD_L_L,
    /// Load the value located at address `HL` into register `L`.
    LD_L_HL,
    /// Load register `A` into register `L`.
    LD_L_A,

    // * 0x7_
    /// Load register `B` into the location of address `HL`.
    LD_HL_B,
    /// Load register `C` into the location of address `HL`.
    LD_HL_C,
    /// Load register `D` into the location of address `HL`.
    LD_HL_D,
    /// Load register `E` into the location of address `HL`.
    LD_HL_E,
    /// Load register `H` into the location of address `HL`.
    LD_HL_H,
    /// Load register `L` into the location of address `HL`.
    LD_HL_L,
    /// Power down the CPU until an interrupt occurs.
    HALT,
    /// Load register `A` into the location of address `HL`.
    LD_HL_A,
    /// Load register `B` into register `A`.
    LD_A_B,
    /// Load register `C` into register `A`.
    LD_A_C,
    /// Load register `D` into register `A`.
    LD_A_D,
    /// Load register `E` into register `A`.
    LD_A_E,
    /// Load register `H` into register `A`.
    LD_A_H,
    /// Load register `L` into register `A`.
    LD_A_L,
    /// Load the value located at address `HL` into register `A`.
    LD_A_HL,
    /// Load register `A` into itself.
    LD_A_A,

    // * 0x8_
    /// Add register `B` to register `A`.
    ADD_A_B,
    /// Add register `C` to register `A`.
    ADD_A_C,
    /// Add register `D` to register `A`.
    ADD_A_D,
    /// Add register `E` to register `A`.
    ADD_A_E,
    /// Add register `H` to register `A`.
    ADD_A_H,
    /// Add register `L` to register `A`.
    ADD_A_L,
    /// Add the value located at address `HL` to register `A`.
    ADD_A_HL,
    /// Add register `A` to itself.
    ADD_A_A,
    /// Add register `B` and the carry flag to register `A`.
    ADC_A_B,
    /// Add register `B` and the carry flag to register `A`.
    ADC_A_C,
    /// Add register `D` and the carry flag to register `A`.
    ADC_A_D,
    /// Add register `E` and the carry flag to register `A`.
    ADC_A_E,
    /// Add register `H` and the carry flag to register `A`.
    ADC_A_H,
    /// Add register `L` and the carry flag to register `A`.
    ADC_A_L,
    /// Add the value located at address `HL` and the carry flag to register `A`.
    ADC_A_HL,
    /// Add register `A` and the carry flag to `A`.
    ADC_A_A,

    // * 0x9_
    /// Subtract register `B` from register `A`.
    SUB_A_B,
    /// Subtract register `C` from register `A`.
    SUB_A_C,
    /// Subtract register `D` from register `A`.
    SUB_A_D,
    /// Subtract register `E` from register `A`.
    SUB_A_E,
    /// Subtract register `H` from register `A`.
    SUB_A_H,
    /// Subtract register `L` from register `A`.
    SUB_A_L,
    /// Subtract the value located at address `HL` from register `A`.
    SUB_A_HL,
    /// Subtract register `A` from itself.
    SUB_A_A,
    /// Subtract register `B` and the carry flag from register `A`.
    SBC_A_B,
    /// Subtract register `B` and the carry flag from register `A`.
    SBC_A_C,
    /// Subtract register `D` and the carry flag from register `A`.
    SBC_A_D,
    /// Subtract register `E` and the carry flag from register `A`.
    SBC_A_E,
    /// Subtract register `H` and the carry flag from register `A`.
    SBC_A_H,
    /// Subtract register `L` and the carry flag from register `A`.
    SBC_A_L,
    /// Subtract the value located at address `HL` and the carry flag from register `A`.
    SBC_A_HL,
    /// Subtract register `A` and the carry flag from `A`.
    SBC_A_A,

    // * 0xA_
    /// Bitwise AND registers `A` and `B`, storing the result in `A`.
    AND_A_B,
    /// Bitwise AND registers `A` and `C`, storing the result in `A`.
    AND_A_C,
    /// Bitwise AND registers `A` and `D`, storing the result in `A`.
    AND_A_D,
    /// Bitwise AND registers `A` and `E`, storing the result in `A`.
    AND_A_E,
    /// Bitwise AND registers `A` and `H`, storing the result in `A`.
    AND_A_H,
    /// Bitwise AND registers `A` and `L`, storing the result in `A`.
    AND_A_L,
    /// Bitwise AND register `A` and the value located at address `HL` storing the result in `A`.
    AND_A_HL,
    /// Bitwise AND register `A` with itself, storing the result in `A`.
    AND_A_A,
    /// Bitwise XOR registers `A` and `B`, storing the result in `A`.
    XOR_A_B,
    /// Bitwise XOR registers `A` and `C`, storing the result in `A`.
    XOR_A_C,
    /// Bitwise XOR registers `A` and `D`, storing the result in `A`.
    XOR_A_D,
    /// Bitwise XOR registers `A` and `E`, storing the result in `A`.
    XOR_A_E,
    /// Bitwise XOR registers `A` and `H`, storing the result in `A`.
    XOR_A_H,
    /// Bitwise XOR registers `A` and `L`, storing the result in `A`.
    XOR_A_L,
    /// Bitwise XOR register `A` and the value located at address `HL` storing the result in `A`.
    XOR_A_HL,
    /// Bitwise XOR register `A` with itself, storing the result in `A`.
    XOR_A_A,

    // * 0xB_
    /// Bitwise OR registers `A` and `B`, storing the result in `A`.
    OR_A_B,
    /// Bitwise OR registers `A` and `C`, storing the result in `A`.
    OR_A_C,
    /// Bitwise OR registers `A` and `D`, storing the result in `A`.
    OR_A_D,
    /// Bitwise OR registers `A` and `E`, storing the result in `A`.
    OR_A_E,
    /// Bitwise OR registers `A` and `H`, storing the result in `A`.
    OR_A_H,
    /// Bitwise OR registers `A` and `L`, storing the result in `A`.
    OR_A_L,
    /// Bitwise OR register `A` and the value located at address `HL` storing the result in `A`.
    OR_A_HL,
    /// Bitwise OR register `A` with itself, storing the result in `A`.
    OR_A_A,
    /// Subtract register `B` from register `A`, but do not store the result.
    CP_A_B,
    /// Subtract register `C` from register `A`, but do not store the result.
    CP_A_C,
    /// Subtract register `D` from register `A`, but do not store the result.
    CP_A_D,
    /// Subtract register `E` from register `A`, but do not store the result.
    CP_A_E,
    /// Subtract register `H` from register `A`, but do not store the result.
    CP_A_H,
    /// Subtract register `L` from register `A`, but do not store the result.
    CP_A_L,
    /// Subtract the value located at address `HL` from register `A`, but do not store the result.
    CP_A_HL,
    /// Subtract register `A` from itself, but do not store the result.
    CP_A_A,

    // * 0xC_
    /// Return from subroutine if the zero flag is not set.
    RET_NZ,
    /// Pop register pair `BC` from the stack.
    POP_BC,
    /// Jump to the immediate address `a16` if the zero flag is not set.
    JP_NZ_a16,
    /// Jump to the immediate address `a16`.
    JP_a16,
    /// Call the immediate address `a16` if the zero flag is not set.
    CALL_NZ_a16,
    /// Push register pair `BC` into the stack.
    PUSH_BC,
    /// Add the immediate value `n8` to register `A`.
    ADD_A_n8,
    /// Call the address `0x00`.
    RST_0x00,
    /// Return from subroutine if the zero flag is set.
    RET_Z,
    /// Return from subroutine.
    RET,
    /// Jump to the immediate address `a16` if the zero flag is set.
    JP_Z_a16,
    /// Access to secondary prefixed instructions.
    PREFIX,
    /// Call the immediate address `a16` if the zero flag is set.
    CALL_Z_a16,
    /// Call the immediate address `a16`.
    CALL_a16,
    /// Add the immediate value `n8` and the carry flag to register `A`.
    ADC_A_n8,
    /// Call the address `0x08`.
    RST_0x08,

    // * 0xD_
    /// Return from subroutine if the carry flag is not set.
    RET_NC,
    /// Pop register pair `DE` from the stack.
    POP_DE,
    /// Jump to the immediate address `a16` if the carry flag is not set.
    JP_NC_a16,
    /// Call the immediate address `a16` if the carry flag is not set.
    CALL_NC_a16 = 0xD4,
    /// Push register pair `DE` into the stack.
    PUSH_DE,
    /// Subtract the immediate value `n8` from register `A`.
    SUB_A_n8,
    /// Call the address `0x10`.
    RST_0x10,
    /// Return from subroutine if the carry flag is set.
    RET_C,
    /// Return from subroutine and enable interupts.
    RETI,
    /// Jump to the immediate address `a16` if the carry flag is set.
    JP_C_a16,
    /// Call the immediate address `a16` if the carry flag is set.
    CALL_C_a16 = 0xDC,
    /// Subtract the immediate value `n8` and the carry flag from register `A`.
    SBC_A_n8 = 0xDE,
    /// Call the address `0x18`.
    RST_0x18,

    // * 0xE_
    /// Load register `A` into the location of immediate address `0xFF00 + a8`.
    LDH_a8_A,
    /// Pop register pair `HL` from the stack.
    POP_HL,
    /// Load register `A` into the location of address `0xFF00 + C`.
    LDH_C_A,
    /// Push register pair `HL` into the stack.
    PUSH_HL = 0xE5,
    /// Bitwise AND register `A` and the immediate value `n8`, storing the result in `A`.
    AND_A_n8,
    /// Call the address `0x20`.
    RST_0x20,
    /// Add the immediate signed value `e8` to the `SP`.
    ADD_SP_e8,
    /// Jump to the location of address `HL`.
    JP_HL,
    /// Load register `A` into the location of immediate address `a16`.
    LD_a16_A,
    /// Bitwise XOR register `A` and the immediate value `n8`, storing the result in `A`.
    XOR_A_n8 = 0xEE,
    /// Call the address `0x28`.
    RST_0x28,

    // * 0xF_
    /// Load register `A` into the location of immediate address `0xFF00 + a8`.
    LDH_A_a8,
    /// Pop register pair `AF` from the stack.
    POP_AF,
    /// Load register `A` into the location of address `0xFF00 + C`.
    LDH_A_C,
    /// Disable interrupts by clearing the IME flag.
    DI,
    /// Push register pair `AF` into the stack.
    PUSH_AF = 0xF5,
    /// Bitwise OR register `A` and the immediate value `n8`, storing the result in `A`.
    OR_A_n8,
    /// Call the address `0x30`.
    RST_0x30,
    /// Load the addition of the signed immediate value `e8` and the `SP` into register pair `HL`.
    LD_HL_SP_e8,
    /// Load register pair `HL` into the `SP`.
    LD_SP_HL,
    /// Load the value located at immediate address `a16` into register `A`.
    LD_A_a16,
    /// Enable interrupts by setting the IME flag after the next instruction.
    EI,
    /// Subtract the immediate value `n8` from register `A`, but do not store the result.
    CP_A_n8 = 0xFE,
    /// Call the address `0x38`.
    RST_0x38,
}