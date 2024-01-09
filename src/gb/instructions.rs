#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u8)]
pub enum Instruction {
    // * 0x0_
    /// No operation.
    NOP = 0x00,
    /// Load the immediate value `n16` into register pair `BC`.
    LD_BC_n16,
    /// Load the value of `A` into the address `BC`.
    LD_BC_A,
    /// Increment register pair `BC`.
    INC_BC,
    /// Increment register `B`.
    INC_B,
    /// Decrement register `B`.
    DEC_B,
    /// Load the immediate value `n8` into register `B`.
    LD_B_n8,
    /// Rotate register `A` left, setting the carry flag to the new MSB.
    RLCA,
    /// Load the value of register `SP` into immediate address `a16`.
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
    /// Rotate register `A` right, setting the carry flag to the new LSB.
    RRCA,

    // * 0x1_
    /// Put the CPU into a low power state until an interrupt occurs (`n8` is any byte, usually `0x00`).
    STOP_n8,
    /// Load the immediate value `n16` into register pair `DE`.
    LD_DE_n16,
    /// Load the value of `A` into location of address `DE`.
    LD_DE_A,
    /// Increment register pair `DE`.
    INC_DE,
    /// Increment register `D`.
    INC_D,
    /// Decrement register `D`.
    DEC_D,
    /// Load the immediate value `n8` into register `D`.
    LD_D_n8,
    /// Rotate register `A` left through the carry flag.
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
    /// Rotate register `A` right through the carry flag.
    RRA,

    // * 0x2_
    /// If the zero flag is not set, add the signed immediate value `e8` to the `PC` and jump to it.
    JR_NZ_e8,
    /// Load the immediate value `n16` into register pair `HL`.
    LD_HL_n16,
    /// Load the value of `A` into location of address `HL`, then increment register pair `HL`.
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
    /// If the zero flag is set, add the signed immediate value `e8` to the `PC` and jump to it.
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
    /// If the carry flag is not set, add the signed immediate value `e8` to the `PC` and jump to it.
    JR_NC_e8,
    /// Load the immdediate value `n16` into the `SP`.
    LD_SP_n16,
    /// Load the value of `A` into location of address `HL`, then decrement register pair `HL`.
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
    /// If the carry flag is set, add the signed immediate value `e8` to the `PC` and jump to it.
    JR_C_e8,
    /// Add register pair `HL` and the `SP`, storing the result in `HL`.
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
    /// Complement (invert) the carry flag.
    CCF,

    // * 0x4_
    LD_B_B,
    LD_B_C,
    LD_B_D,
    LD_B_E,
    LD_B_H,
    LD_B_L,
    LD_B_HL,
    LD_B_A,
    LD_C_B,
    LD_C_C,
    LD_C_D,
    LD_C_E,
    LD_C_H,
    LD_C_L,
    LD_C_HL,
    LD_C_A,

    // * 0x5_
    LD_D_B,
    LD_D_C,
    LD_D_D,
    LD_D_E,
    LD_D_H,
    LD_D_L,
    LD_D_HL,
    LD_D_A,
    LD_E_B,
    LD_E_C,
    LD_E_D,
    LD_E_E,
    LD_E_H,
    LD_E_L,
    LD_E_HL,
    LD_E_A,

    // * 0x6_
    LD_H_B,
    LD_H_C,
    LD_H_D,
    LD_H_E,
    LD_H_H,
    LD_H_L,
    LD_H_HL,
    LD_H_A,
    LD_L_B,
    LD_L_C,
    LD_L_D,
    LD_L_E,
    LD_L_H,
    LD_L_L,
    LD_L_HL,
    LD_L_A,

    // * 0x7_
    LD_HL_B,
    LD_HL_C,
    LD_HL_D,
    LD_HL_E,
    LD_HL_H,
    LD_HL_L,
    HALT,
    LD_HL_A,
    LD_A_B,
    LD_A_C,
    LD_A_D,
    LD_A_E,
    LD_A_H,
    LD_A_L,
    LD_A_HL,
    LD_A_A,

    // * 0x8_
    ADD_A_B,
    ADD_A_C,
    ADD_A_D,
    ADD_A_E,
    ADD_A_H,
    ADD_A_L,
    ADD_A_HL,
    ADD_A_A,
    ADC_A_B,
    ADC_A_C,
    ADC_A_D,
    ADC_A_E,
    ADC_A_H,
    ADC_A_L,
    ADC_A_HL,
    ADC_A_A,

    // * 0x9_
    SUB_A_B,
    SUB_A_C,
    SUB_A_D,
    SUB_A_E,
    SUB_A_H,
    SUB_A_L,
    SUB_A_HL,
    SUB_A_A,
    SBC_A_B,
    SBC_A_C,
    SBC_A_D,
    SBC_A_E,
    SBC_A_H,
    SBC_A_L,
    SBC_A_HL,
    SBC_A_A,

    // * 0xA_
    AND_A_B,
    AND_A_C,
    AND_A_D,
    AND_A_E,
    AND_A_H,
    AND_A_L,
    AND_A_HL,
    AND_A_A,
    XOR_A_B,
    XOR_A_C,
    XOR_A_D,
    XOR_A_E,
    XOR_A_H,
    XOR_A_L,
    XOR_A_HL,
    XOR_A_A,

    // * 0xB_
    OR_A_B,
    OR_A_C,
    OR_A_D,
    OR_A_E,
    OR_A_H,
    OR_A_L,
    OR_A_HL,
    OR_A_A,
    CP_A_B,
    CP_A_C,
    CP_A_D,
    CP_A_E,
    CP_A_H,
    CP_A_L,
    CP_A_HL,
    CP_A_A,

    // * 0xC_
    RET_NZ,
    POP_BC,
    JP_NZ_a16,
    JP_a16,
    CALL_NZ_a16,
    PUSH_BC,
    ADD_A_n8,
    RST_x00,
    RET_Z,
    RET,
    JP_Z_a16,
    PREFIX,
    CALL_Z_a16,
    CALL_a16,
    ADC_A_n8,
    RST_x08,

    // * 0xD_
    RET_NC,
    POP_DE,
    JP_NC_a16,
    CALL_NC_a16 = 0xD4,
    PUSH_DE,
    SUB_A_n8,
    RST_x10,
    RET_C,
    RETI,
    JP_C_a16,
    CALL_C_a16 = 0xDC,
    SBC_A_n8 = 0xDE,
    RST_x18,

    // * 0xE_
    LDH_a8_A,
    POP_HL,
    LDIO_C_A,
    PUSH_HL = 0xE5,
    AND_A_n8,
    RST_0x20,
    ADD_SP_e8,
    JP_HL,
    LD_a16_A,
    XOR_A_n8 = 0xEE,
    RST_0x28,

    // * 0xF_
    LDH_A_a8,
    POP_AF,
    LDIO_A_C,
    DI,
    PUSH_AF = 0xF5,
    OR_A_n8,
    RST_0x30,
    LD_HL_SP_e8,
    LD_SP_HL,
    LDA_a16,
    EI,
    CP_A_n8 = 0xFE,
    RST_0x38,
}