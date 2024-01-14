#![allow(non_snake_case)]

use super::{instructions::*, utils::*};

// * LD

/// Load register `r8_2` into register `r8_1`.
pub const fn LD_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("LD {:?}, {:?}", r8_1, r8_2).as_str(), |emu| {
        let v = emu.cpu.get_register(r8_2);
        emu.cpu.set_register(r8_1, v);
        return InstructionStep::Complete;
    })
}

/// Load immediate value `n8` into register `r8`.
pub const fn LD_r8_n8(r8: Register) -> Instruction {
    Instruction::new(format!("LD {:?}, n8", r8).as_str(), |emu| {
        let value = emu.read_pc();
        return InstructionStep::new(move |emu| {
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

/// Load the value at address `r16` into register `r8`.
pub const fn LD_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD {:?}, ({:?})", r8, r16).as_str(), |emu| {
        let value = emu.read_r16(r16);
        return InstructionStep::new(move |emu| {
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

/// Load register `r8` into the location of address `r16`.
pub const fn LD_r16_r8(r16: RegisterPair, r8: Register) -> Instruction {
    Instruction::new(format!("LD ({:?}), {:?}", r16, r8).as_str(), |emu| {
        let value = emu.cpu.get_register(r8);
        return InstructionStep::new(move |emu| {
            emu.write_r16(r16, value);
            return InstructionStep::Complete;
        });
    })
}

/// Load immediate value `n8` into the location of address `r16`.
pub const fn LD_r16_n8(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD ({:?}), n8", r16).as_str(), |emu| {
        // ? One bus read per m-cycle.
        return InstructionStep::new(move |emu| {
            let value = emu.read_pc();
            return InstructionStep::new(move |emu| {
                emu.write_r16(r16, value);
                return InstructionStep::Complete;
            });
        });
    })
}

/// Load immediate value `n16` into the register pair `r16`.
pub const fn LD_r16_n16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD ({:?}), n16", r16).as_str(), |emu| {
        // ? One bus read per m-cycle.
        return InstructionStep::new(move |emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(move |emu| {
                let msb = emu.read_pc();
                // The lsb and msb are technically listed as being written seperately, but eh.
                emu.cpu.set_register_pair(r16, join_u16(lsb, msb));
                return InstructionStep::Complete;
            });
        });
    })
}

/// Load the register pair `r16` into the location of immediate addresses `n16` and `n16 + 1`.
pub const fn LD_n16_r16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD (n16), {:?}", r16).as_str(), |emu| {
        // ? One bus read per m-cycle.
        return InstructionStep::new(move |emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(move |emu| {
                let msb = emu.read_pc();
                return InstructionStep::new(move |emu| {
                    let address = join_u16(lsb, msb);
                    let (lsb, msb) = split_u16(emu.cpu.get_register_pair(r16));
                    emu.bus.write(address, lsb);
                    return InstructionStep::new(move |emu| {
                        emu.bus.write(address + 1, msb);
                        return InstructionStep::Complete;
                    });
                });
            });
        });
    })
}

// * INC/DEC

/// Increment register pair `r16`.
#[macro_export]
macro_rules! INC_r16 {
    ($r16:expr) => {
        Instruction::new(
            format!("INC {:?}", $r16).as_str(),
            vec![
                &|_, emu| { /* Technically writes to the registers of the pair in seperate steps. */
                },
                &|_, emu| emu.cpu.inc_register_pair($r16),
            ],
        )
    };
}

/// Increment register `r8`.
#[macro_export]
macro_rules! INC_r8 {
    ($r8:expr) => {
        Instruction::new(
            format!("INC {:?}", $r8).as_str(),
            vec![&|_, emu| emu.cpu.inc_register($r8)],
        )
    };
}

/// Decrement register pair `r16`.
#[macro_export]
macro_rules! DEC_r16 {
    ($r16:expr) => {
        Instruction::new(
            format!("INC {:?}", $r16).as_str(),
            vec![
                &|_, emu| { /* Technically writes to the registers of the pair in seperate steps. */
                },
                &|_, emu| emu.cpu.dec_register_pair($r16),
            ],
        )
    };
}

/// Decrement register `r8`.
#[macro_export]
macro_rules! DEC_r8 {
    ($r8:expr) => {
        Instruction::new(
            format!("INC {:?}", $r8).as_str(),
            vec![&|_, emu| emu.cpu.dec_register($r8)],
        )
    };
}

// * ADD

/// Add registers `r8_1` and `r8_2`, storing the result in `r8_1`.
#[macro_export]
macro_rules! ADD_r8_r8 {
    ($r8_1:expr, $r8_2:expr) => {
        Instruction::new(
            format!("ADD {:?}, {:?}", $r8_1, $r8_2).as_str(),
            vec![&|_, _, emu| {
                let val = emu.cpu.get_register($r8_2);
                let v = self.cpu.add_register($r8_1, val);
                emu.cpu.set_register($r8_1, v);
            }],
        )
    };
}

/// Add register `r8` and immediate value `n8`, storing the result in `r8`.
#[macro_export]
macro_rules! ADD_r8_n8 {
    ($r8:expr) => {
        Instruction::new(
            format!("ADD {:?}, n8", $r8_).as_str(),
            vec![
                &|[t8, _, _, _], emu| *t8 = self.cpu.read(RegisterPair::PC),
                &|[t8, _, _, _], emu| {
                    let v = self.cpu.add_register($r8, *t8);
                    emu.cpu.set_register($r8, v);
                },
            ],
        )
    };
}

/// Add the value at address `r16` to register `r8`.
#[macro_export]
macro_rules! ADD_r8_r16 {
    ($r8:expr, $r16:expr) => {
        Instruction::new(
            format!("ADD {:?}, ({:?})", $r8, $r16).as_str(),
            vec![
                &|[t8, _, _, _], emu| *t8 = emu.read_pc(),
                &|[t8, _, _, _], emu| {
                    let v = self.cpu.add_register($r8, *t8);
                    emu.cpu.set_register($r8, v);
                },
            ],
        )
    };
}

/// Add register pairs `r16_1` and `r16_2`, storing the result in `r16_1`.
#[macro_export]
macro_rules! ADD_r16_r16 {
    ($r16_1:expr, $r16_2:expr) => {
        Instruction::new(
            format!("ADD {:?}, {:?}", $r16_1, $r16_2).as_str(),
            vec![
                &|[lsb, msb, _, _], emu| {
                    (*lsb, *msb) = split_u16(emu.cpu.get_register_pair($r16_2))
                },
                &|[lsb, msb, _, _], emu| {
                    let v = emu.cpu.add_register_pair($r16_1, join_u16(*lsb, *msb));
                    emu.cpu.set_register_pair($r16_1, v);
                },
            ],
        )
    };
}

// * JR & JP

/// Add the signed immediate value `e8` to the `PC` and jump to it.
#[macro_export]
macro_rules! JR_i8 {
    () => {
        Instruction::new(
            format!("ADD {:?}, {:?}", $r16_1, $r16_2).as_str(),
            vec![
                &|[lsb, msb, t8, _], emu| (*lsb, *msb) = split_u16(emu.cpu.get_register_pair(RegisterPair::PC)),
                &|[lsb, msb, t8, _], emu| *t8 = emu.read_pc(),
                &|[lsb, msb, t8, _], emu| *t8 = ,
            ],self.cpu
            //             .set_register_pair(RegisterPair::PC, r.wrapping_add_signed(v.into()));
        )
    };
}

// /// If flag `c` is set, add the signed immediate value `e8` to the `PC` and jump to it.
// pub const fn JR_c_e8(&mut self, c: Flag) {
//     let v = self.read_pc() as i8;
//     if self.cpu.get_flag(c) {
//         let r = self.cpu.get_register_pair(RegisterPair::PC);
//         self.cpu
//             .set_register_pair(RegisterPair::PC, r.wrapping_add_signed(v.into()));
//     }
// }

// /// If flag `c` is not set, add the signed immediate value `e8` to the `PC` and jump to it.
// pub const fn JR_nc_e8(&mut self, c: Flag) {
//     let v = self.read_pc() as i8;
//     if !self.cpu.get_flag(c) {
//         let r = self.cpu.get_register_pair(RegisterPair::PC);
//         self.cpu
//             .set_register_pair(RegisterPair::PC, r.wrapping_add_signed(v.into()));
//     }
// }

// /// Jump to the immediate address `a16`.
// pub const fn JP_a16(&mut self) {
//     let v = self.read_u16(RegisterPair::PC);
//     self.cpu.set_register_pair(RegisterPair::PC, v);
// }

// /// If flag `c` is set, jump to the immediate address `a16`.
// pub const fn JP_c_a16(&mut self, c: Flag) {
//     let v = self.read_u16(RegisterPair::PC);
//     if self.cpu.get_flag(c) {
//         self.cpu.set_register_pair(RegisterPair::PC, v);
//     }
// }

// /// If flag `c` is not set, jump to the immediate address `a16`.
// pub const fn JP_nc_a16(&mut self, c: Flag) {
//     let v = self.read_u16(RegisterPair::PC);
//     if !self.cpu.get_flag(c) {
//         self.cpu.set_register_pair(RegisterPair::PC, v);
//     }
// }

// // * ADC

// /// Add the carry flag, and registers `r8_1` and `r8_2`, storing the result in `r8_1`.
// pub const fn ADC_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
//     let val = self.cpu.get_register(r8_2) + self.cpu.get_flag(Flag::C) as u8;
//     let res = self.cpu.add_register(r8_1, val);
//     self.cpu.set_register(r8_1, res);
// }

// /// Add the carry flag, register `r8` and immediate value `n8`, storing the result in `r8`.
// pub const fn ADC_r8_n8(&mut self, r8: Register) {
//     let val = self.read_pc() + self.cpu.get_flag(Flag::C) as u8;
//     let v = self.cpu.add_register(r8, val);
//     self.cpu.set_register(r8, v);
// }

// /// Add the carry flag, register `r8` and the value at addresss `r16`, storing the result in `r8`.
// pub const fn ADC_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let val = self.bus.read(address) + self.cpu.get_flag(Flag::C) as u8;
//     let res = self.cpu.add_register(r8, val);
//     self.cpu.set_register(r8, res);
// }

// // * SUB

// /// Subtract register `r8_2` from register `r8_1`, storing the result in `r8_1`.
// pub const fn SUB_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
//     let val = self.cpu.get_register(r8_2);
//     let v = self.cpu.sub_register(r8_1, val);
//     self.cpu.set_register(r8_1, v);
// }

// /// Subtract immediate value `n8` from register `r8`, storing the result in `r8`.
// pub const fn SUB_r8_n8(&mut self, r8: Register) {
//     let val = self.read_pc();
//     let v = self.cpu.sub_register(r8, val);
//     self.cpu.set_register(r8, v);
// }

// /// Subtract the value at `r16` from register `r8`, storing the result in `r8`.
// pub const fn SUB_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let val = self.bus.read(address);
//     let res = self.cpu.sub_register(r8, val);
//     self.cpu.set_register(r8, res);
// }

// // * SBC

// /// Subtract the carry flag and register `r8_2` from register `r8_1`, storing the result in `r8_1`.
// pub const fn SBC_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
//     let val = self.cpu.get_register(r8_2) + self.cpu.get_flag(Flag::C) as u8;
//     let res = self.cpu.sub_register(r8_1, val);
//     self.cpu.set_register(r8_1, res);
// }

// /// Subtract the carry flag and the immediate value `n8` from register `r8`, storing the result in `r8`.
// pub const fn SBC_r8_n8(&mut self, r8: Register) {
//     let val = self.read_pc() + self.cpu.get_flag(Flag::C) as u8;
//     let res = self.cpu.sub_register(r8, val);
//     self.cpu.set_register(r8, res);
// }

// /// Subtract the carry flag and the value at `r16` from register `r8`, storing the result in `r8`.
// pub const fn SBC_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let val = self.bus.read(address) + self.cpu.get_flag(Flag::C) as u8;
//     let res = self.cpu.sub_register(r8, val);
//     self.cpu.set_register(r8, res);
// }

// // * AND

// /// Bitwise AND registers `r8_1` and `r8_2`, storing the result in `r8_1`.
// pub const fn AND_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
//     let val = self.cpu.get_register(r8_2);
//     let v = self.cpu.and_register(r8_1, val);
//     self.cpu.set_register(r8_1, v);

//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::H, true);
//     self.cpu.set_flag(Flag::N | Flag::C, false);
// }

// /// Bitwise AND register `r8` and the value at address`r16`, storing the result in `r8`.
// pub const fn AND_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let val = self.bus.read(address);
//     let v = self.cpu.and_register(r8, val);
//     self.cpu.set_register(r8, v);

//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::H, true);
//     self.cpu.set_flag(Flag::N | Flag::C, false);
// }

// // * XOR

// /// Bitwise XOR registers `r8_1` and `r8_2`, storing the result in `r8_1`.
// pub const fn XOR_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
//     let val = self.cpu.get_register(r8_2);
//     let v = self.cpu.xor_register(r8_1, val);
//     self.cpu.set_register(r8_1, v);

//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// /// Bitwise XOR register `r8` and the value at address`r16`, storing the result in `r8`.
// pub const fn XOR_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let val = self.bus.read(address);
//     let v = self.cpu.xor_register(r8, val);
//     self.cpu.set_register(r8, v);

//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// // * OR

// /// Bitwise OR registers `r8_1` and `r8_2`, storing the result in `r8_1`.
// pub const fn OR_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
//     let val = self.cpu.get_register(r8_2);
//     let v = self.cpu.or_register(r8_1, val);
//     self.cpu.set_register(r8_1, v);

//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// /// Bitwise OR register `r8` and the value at address`r16`, storing the result in `r8`.
// pub const fn OR_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let val = self.bus.read(address);
//     let v = self.cpu.or_register(r8, val);
//     self.cpu.set_register(r8, v);

//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// // * CP

// /// Subtract register `r8_2` from register `r8_1`, but do not store the result.
// pub const fn CP_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
//     let val = self.cpu.get_register(r8_2);
//     let _ = self.cpu.sub_register(r8_1, val);
// }

// /// Subtract the value at `r16` from register `r8`, but do not store the result.
// pub const fn CP_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let val = self.bus.read(address);
//     let _ = self.cpu.sub_register(r8, val);
// }

// // * RET, CALL & RST

// /// Return from subroutine.
// pub const fn RET(&mut self) {
//     let v = self.read_u16(RegisterPair::SP);
//     self.cpu.set_register_pair(RegisterPair::PC, v);
// }

// /// Call function at the immediate address `a16`.
// pub const fn CALL_a16(&mut self) {
//     let v = self.read_u16(RegisterPair::PC);
//     let prev_pc = self.cpu.get_register_pair(RegisterPair::PC);
//     self.write_stack(prev_pc);
//     self.cpu.set_register_pair(RegisterPair::PC, v);
// }

// /// If flag `c` is set, call function at the immediate address `a16`.
// pub const fn CALL_c_a16(&mut self, c: Flag) {
//     let v = self.read_u16(RegisterPair::PC);
//     if self.cpu.get_flag(c) {
//         let prev_pc = self.cpu.get_register_pair(RegisterPair::PC);
//         self.write_stack(prev_pc);
//         self.cpu.set_register_pair(RegisterPair::PC, v);
//     }
// }

// /// If flag `c` is not set, call function at the immediate address `a16`.
// pub const fn CALL_nc_a16(&mut self, c: Flag) {
//     let v = self.read_u16(RegisterPair::PC);
//     if !self.cpu.get_flag(c) {
//         let prev_pc = self.cpu.get_register_pair(RegisterPair::PC);
//         self.write_stack(prev_pc);
//         self.cpu.set_register_pair(RegisterPair::PC, v);
//     }
// }

// /// Call fixed address `a16`.
// pub const fn RST_a16(&mut self, a16: u16) {
//     let prev_pc = self.cpu.get_register_pair(RegisterPair::PC);
//     self.write_stack(prev_pc);
//     self.cpu.set_register_pair(RegisterPair::PC, a16);
// }

// // * PUSH & POP

// /// Push register pair `r16` into the stack.
// pub const fn PUSH_r16(&mut self, r16: RegisterPair) {
//     let val = self.cpu.get_register_pair(r16);
//     self.write_stack(val);
// }

// /// Pop from the stack to register pair `r16`.
// pub const fn POP_r16(&mut self, r16: RegisterPair) {
//     let v = self.read_u16(RegisterPair::SP);
//     self.cpu.set_register_pair(r16, v);
// }

// // * RLC & RRC

// // ? Opcode | Carry | Bit 0
// // ? RLC    | Bit 7 | Bit 7
// // ? RL     | Bit 7 | Carry
// // ? SL/SLA | Bit 7 | 0

// // ? Opcode | Carry | Bit 7
// // ? RRC    | Bit 0 | Bit 0
// // ? RR     | Bit 0 | Carry
// // ? SR/SRL | Bit 0 | 0
// // ? CB SRA | Bit 0 | Bit 7

// /// Rotate register `r8` left, setting the carry flag to the previous bit 7.
// pub const fn RLC_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = (v << 1) | new_carry as u8;

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate register `r8` right, setting the carry flag to the previous bit 0.
// pub const fn RRC_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | ((new_carry as u8) << 7);

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate the value at `r16` left, setting the carry flag to the previous bit 7.
// pub const fn RLC_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = (v << 1) | new_carry as u8;

//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate the value at `r16` right, setting the carry flag to the previous bit 0.
// pub const fn RRC_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | ((new_carry as u8) << 7);

//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
// }

// // * RL & RR

// /// Rotate register `r8` and the carry flag left.
// pub const fn RL_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let prev_carry = self.cpu.get_flag(Flag::C);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = (v << 1) | prev_carry as u8;

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate register `r8` and the carry flag right.
// pub const fn RR_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let prev_carry = self.cpu.get_flag(Flag::C);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | ((prev_carry as u8) << 7);

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate the value at address `r16` and the carry flag left.
// pub const fn RL_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let prev_carry = self.cpu.get_flag(Flag::C);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = (v << 1) | prev_carry as u8;

//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate the value at address `r16` and the carry flag right.
// pub const fn RR_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let prev_carry = self.cpu.get_flag(Flag::C);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | ((prev_carry as u8) << 7);

//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// // * SLA, SRA & SRL

// /// Shift register `r8` left arithmetically.
// pub const fn SLA_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = v << 1;

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift register `r8` right arithmetically.
// pub const fn SRA_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | (v & 0b1000_0000);

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift register `r8` right logically.
// pub const fn SRL_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = v >> 1;

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift the value at address `r16` left arithmetically.
// pub const fn SLA_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = v << 1;

//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift the value at address `r16` right arithmetically.
// pub const fn SRA_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | (v & 0b1000_0000);

//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift the value at address `r16` right logically.
// pub const fn SRL_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = v >> 1;

//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// // * SWAP & BIT

// /// Swap the upper and lower 4 bits of register `r8`.
// pub const fn SWAP_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let v = (v << 4) | (v >> 4);
//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// /// Swap the upper and lower 4 bits of the value at address `r16`.
// pub const fn SWAP_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let v = (v << 4) | (v >> 4);
//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// /// Set the zero flag if bit `b` of register `r8` is not set.
// pub const fn BIT_b_r8(&mut self, b: u8, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     self.cpu.set_flag(Flag::Z, get_bit(&v, 1 << b));
//     self.cpu.set_flag(Flag::N, false);
//     self.cpu.set_flag(Flag::H, true);
// }

// /// Set the zero flag if bit `b` of the value at address `r16` is not set.
// pub const fn BIT_b_r16(&mut self, b: u8, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     self.cpu.set_flag(Flag::Z, get_bit(&v, 1 << b));
//     self.cpu.set_flag(Flag::N, false);
//     self.cpu.set_flag(Flag::H, true);
// }

// // * RES & SET

// /// Set bit `b` of register `r8` to 0.
// pub const fn RES_b_r8(&mut self, b: u8, r8: Register) {
//     let mut v = self.cpu.get_register(r8);
//     set_bit(&mut v, 1 << b, false);
//     self.cpu.set_register(r8, v);
// }

// /// Set bit `b` of the value at address `r16` to 0.
// pub const fn RES_b_r16(&mut self, b: u8, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let mut v = self.bus.read(address);
//     set_bit(&mut v, 1 << b, false);
//     self.bus.write(address, v);
// }

// /// Set bit `b` of register `r8` to 1.
// pub const fn SET_b_r8(&mut self, b: u8, r8: Register) {
//     let mut v = self.cpu.get_register(r8);
//     set_bit(&mut v, 1 << b, true);
//     self.cpu.set_register(r8, v);
// }

// /// Set bit `b` of the value at address `r16` to 1.
// pub const fn SET_b_r16(&mut self, b: u8, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let mut v = self.bus.read(address);
//     set_bit(&mut v, 1 << b, true);
//     self.bus.write(address, v);
// }
