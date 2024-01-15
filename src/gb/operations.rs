#![allow(non_snake_case)]

use super::{instructions::*, utils::*};

// * LD

/// Load register `r8_2` into register `r8_1`.
pub fn LD_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("LD {:?}, {:?}", r8_1, r8_2), move |emu| {
        let v = emu.cpu.get_register(r8_2);
        emu.cpu.set_register(r8_1, v);
        return InstructionStep::Complete;
    })
}

/// Load immediate value `n8` into register `r8`.
pub fn LD_r8_n8(r8: Register) -> Instruction {
    Instruction::new(format!("LD {:?}, n8", r8), move |emu| {
        let value = emu.read_pc();
        return InstructionStep::new(move |emu| {
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

/// Load the value at address `r16` into register `r8`.
pub fn LD_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD {:?}, ({:?})", r8, r16), move |emu| {
        let value = emu.read_r16(r16);
        return InstructionStep::new(move |emu| {
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

/// Load register `r8` into the location of address `r16`.
pub fn LD_r16_r8(r16: RegisterPair, r8: Register) -> Instruction {
    Instruction::new(format!("LD ({:?}), {:?}", r16, r8), move |emu| {
        let value = emu.cpu.get_register(r8);
        return InstructionStep::new(move |emu| {
            emu.write_r16(r16, value);
            return InstructionStep::Complete;
        });
    })
}

/// Load immediate value `n8` into the location of address `r16`.
pub fn LD_r16_n8(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD ({:?}), n8", r16), move |_emu| {
        // ? One bus read or write per m-cycle.
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
pub fn LD_r16_n16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD ({:?}), n16", r16), move |_emu| {
        // ? One bus read or write per m-cycle.
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
pub fn LD_n16_r16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("LD (n16), {:?}", r16), move |_emu| {
        // ? One bus read or write per m-cycle.
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

// * INC & DEC

/// Increment register `r8`.
pub fn INC_r8(r8: Register) -> Instruction {
    Instruction::new(format!("INC {:?}", r8), move |emu| {
        emu.cpu.inc_register(r8);
        return InstructionStep::Complete;
    })
}

/// Increment register pair `r16`.
pub fn INC_r16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("INC {:?}", r16), move |_emu| {
        // ? Technically writes to each register seperately.
        return InstructionStep::new(move |emu| {
            emu.cpu.inc_register_pair(r16);
            return InstructionStep::Complete;
        });
    })
}

/// Decrement register `r8`.
pub fn DEC_r8(r8: Register) -> Instruction {
    Instruction::new(format!("DEC {:?}", r8), move |emu| {
        emu.cpu.dec_register(r8);
        return InstructionStep::Complete;
    })
}

/// Decrement register pair `r16`.
pub fn DEC_r16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("DEC {:?}", r16), move |_emu| {
        // ? Technically writes to each register seperately.
        return InstructionStep::new(move |emu| {
            emu.cpu.dec_register_pair(r16);
            return InstructionStep::Complete;
        });
    })
}

// * JR & JP

/// Add the signed immediate value `e8` to the `PC` and jump to it.
pub fn JR_n8() -> Instruction {
    Instruction::new(format!("JR n8"), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let value = emu.read_pc() as i16;
            return InstructionStep::new(move |emu| {
                let pc = emu.cpu.get_register_pair(RegisterPair::PC);
                emu.cpu.set_register_pair(RegisterPair::PC, pc.wrapping_add_signed(value));
                return InstructionStep::Complete;
            });
        });
    })
}

/// If flag `c` is set, add the signed immediate value `n8` to the `PC` and jump to it.
pub fn JR_c_n8(c: Flag) -> Instruction {
    Instruction::new(format!("JR {:?}, n8", c), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let value = emu.read_pc() as i16;
            if emu.cpu.get_flag(c) {
                return InstructionStep::new(move |emu| {
                    let pc = emu.cpu.get_register_pair(RegisterPair::PC);
                    emu.cpu.set_register_pair(RegisterPair::PC, pc.wrapping_add_signed(value));
                    return InstructionStep::Complete;
                });
            } else {
                return InstructionStep::Complete;
            }
        });
    })
}

/// If flag `c` is not set, add the signed immediate value `n8` to the `PC` and jump to it.
pub fn JR_nc_n8(c: Flag) -> Instruction {
    Instruction::new(format!("JR N{:?}, n8", c), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let value = emu.read_pc() as i16;
            if !emu.cpu.get_flag(c) {
                return InstructionStep::new(move |emu| {
                    let pc = emu.cpu.get_register_pair(RegisterPair::PC);
                    emu.cpu.set_register_pair(RegisterPair::PC, pc.wrapping_add_signed(value));
                    return InstructionStep::Complete;
                });
            } else {
                return InstructionStep::Complete;
            }
        });
    })
}

/// Jump to the immediate address `n16`.
pub fn JP_n16() -> Instruction {
    Instruction::new(format!("JP n16"), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(move |emu| {
                let msb = emu.read_pc();
                let value = join_u16(lsb, msb);
                return InstructionStep::new(move |emu| {
                    emu.cpu.set_register_pair(RegisterPair::PC, value);
                    return InstructionStep::Complete;
                });
            });
        });
    })
}

/// If flag `c` is set, jump to the immediate address `n16`.
pub fn JP_c_n16(c: Flag) -> Instruction {
    Instruction::new(format!("JP {:?}, n16", c), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(move |emu| {
                let msb = emu.read_pc();
                let value = join_u16(lsb, msb);
                if emu.cpu.get_flag(c) {
                    return InstructionStep::new(move |emu| {
                        emu.cpu.set_register_pair(RegisterPair::PC, value);
                        return InstructionStep::Complete;
                    });
                } else {
                    return InstructionStep::Complete;
                }
            });
        });
    })
}

/// If flag `c` is not set, jump to the immediate address `n16`.
pub fn JP_nc_n16(c: Flag) -> Instruction {
    Instruction::new(format!("JP N{:?}, n16", c), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(move |emu| {
                let msb = emu.read_pc();
                let value = join_u16(lsb, msb);
                if !emu.cpu.get_flag(c) {
                    return InstructionStep::new(move |emu| {
                        emu.cpu.set_register_pair(RegisterPair::PC, value);
                        return InstructionStep::Complete;
                    });
                } else {
                    return InstructionStep::Complete;
                }
            });
        });
    })
}

// * ADD

/// Add registers `r8_1` and `r8_2`, storing the result in `r8_1`.
pub fn ADD_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("ADD {:?}, {:?}", r8_1, r8_2), move |emu| {
        let mut value = emu.cpu.get_register(r8_2);
        value = emu.cpu.add_register(r8_1, value);
        emu.cpu.set_register(r8_1, value);
        return InstructionStep::Complete;
    })
}

/// Add the value at address `r16` to register `r8`.
pub fn ADD_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("ADD {:?}, ({:?})", r8, r16), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let mut value = emu.read_r16(r16);
            value = emu.cpu.add_register(r8, value);
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

/// Add register pairs `r16_1` and `r16_2`, storing the result in `r16_1`.
pub fn ADD_r16_r16(r16_1: RegisterPair, r16_2: RegisterPair) -> Instruction {
    Instruction::new(format!("ADD {:?}, ({:?})", r16_1, r16_2), move |_emu| {
        // ? Technically writes to each register seperately.
        return InstructionStep::new(move |emu| {
            let mut value = emu.cpu.get_register_pair(r16_2);
            value = emu.cpu.add_register_pair(r16_1, value);
            emu.cpu.set_register_pair(r16_1, value);
            return InstructionStep::Complete;
        });
    })
}

// * ADC

/// Add the carry flag, and registers `r8_1` and `r8_2`, storing the result in `r8_1`.
pub fn ADC_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("ADC {:?}, {:?}", r8_1, r8_2), move |emu| {
        let mut value = emu.cpu.get_register(r8_2) + emu.cpu.get_flag(Flag::C) as u8;
        value = emu.cpu.add_register(r8_1, value);
        emu.cpu.set_register(r8_1, value);
        return InstructionStep::Complete;
    })
}

/// Add the carry flag, register `r8` and the value at addresss `r16`, storing the result in `r8`.
pub fn ADC_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("ADC {:?}, ({:?})", r8, r16), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let mut value = emu.read_r16(r16) + emu.cpu.get_flag(Flag::C) as u8;
            value = emu.cpu.add_register(r8, value);
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

// * SUB

/// Subtract register `r8_2` from register `r8_1`, storing the result in `r8_1`.
pub fn SUB_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("SUB {:?}, {:?}", r8_1, r8_2), move |emu| {
        let mut value = emu.cpu.get_register(r8_2);
        value = emu.cpu.sub_register(r8_1, value);
        emu.cpu.set_register(r8_1, value);
        return InstructionStep::Complete;
    })
}



/// Subtract the value at `r16` from register `r8`, storing the result in `r8`.
pub fn SUB_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("SUB {:?}, ({:?})", r8, r16), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let mut value = emu.read_r16(r16);
            value = emu.cpu.sub_register(r8, value);
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

// * SBC

/// Subtract the carry flag and register `r8_2` from register `r8_1`, storing the result in `r8_1`.
pub fn SBC_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("SBC {:?}, {:?}", r8_1, r8_2), move |emu| {
        let mut value = emu.cpu.get_register(r8_2) + emu.cpu.get_flag(Flag::C) as u8;
        value = emu.cpu.sub_register(r8_1, value);
        emu.cpu.set_register(r8_1, value);
        return InstructionStep::Complete;
    })
}

/// Subtract the carry flag and the value at `r16` from register `r8`, storing the result in `r8`.
pub fn SBC_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("SBC {:?}, ({:?})", r8, r16), move |_emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let mut value = emu.read_r16(r16) + emu.cpu.get_flag(Flag::C) as u8;
            value = emu.cpu.sub_register(r8, value);
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

// * AND

/// Bitwise AND registers `r8_1` and `r8_2`, storing the result in `r8_1`.
pub fn AND_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("AND {:?}, {:?}", r8_1, r8_2), |emu| {
        let mut value = emu.cpu.get_register(r8_2);
        let value = emu.cpu.and_register(r8_1, value);
        emu.cpu.set_register(r8_1, value);
        return InstructionStep::Complete;
    })
}


/// Bitwise AND register `r8` and the value at address`r16`, storing the result in `r8`.
pub fn AND_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("AND {:?}, ({:?})", r8, r16), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let mut value = emu.read_r16(r16);
            let value = emu.cpu.and_register(r8, value);
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

// * XOR

/// Bitwise XOR registers `r8_1` and `r8_2`, storing the result in `r8_1`.
pub fn XOR_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("XOR {:?}, {:?}", r8_1, r8_2), |emu| {
        let mut value = emu.cpu.get_register(r8_2);
        let value = emu.cpu.xor_register(r8_1, value);
        emu.cpu.set_register(r8_1, value);
        return InstructionStep::Complete;
    })
}


/// Bitwise XOR register `r8` and the value at address`r16`, storing the result in `r8`.
pub fn XOR_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("XOR {:?}, ({:?})", r8, r16), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let mut value = emu.read_r16(r16);
            let value = emu.cpu.xor_register(r8, value);
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

// * OR

/// Bitwise OR registers `r8_1` and `r8_2`, storing the result in `r8_1`.
pub fn OR_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("OR {:?}, {:?}", r8_1, r8_2), |emu| {
        let mut value = emu.cpu.get_register(r8_2);
        let value = emu.cpu.or_register(r8_1, value);
        emu.cpu.set_register(r8_1, value);
        return InstructionStep::Complete;
    })
}


/// Bitwise OR register `r8` and the value at address`r16`, storing the result in `r8`.
pub fn OR_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("OR {:?}, ({:?})", r8, r16), move |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(move |emu| {
            let mut value = emu.read_r16(r16);
            value = emu.cpu.or_register(r8, value);
            emu.cpu.set_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

// * CP

/// Subtract register `r8_2` from register `r8_1`, but do not store the result.
pub fn CP_r8_r8(r8_1: Register, r8_2: Register) -> Instruction {
    Instruction::new(format!("CP {:?}, {:?}", r8_1, r8_2), |emu| {
        let value = emu.cpu.get_register(r8_2);
        emu.cpu.sub_register(r8_1, value);
        return InstructionStep::Complete;
    })
}


/// Subtract the value at `r16` from register `r8`, but do not store the result.
pub fn CP_r8_r16(r8: Register, r16: RegisterPair) -> Instruction {
    Instruction::new(format!("CP {:?}, ({:?})", r8, r16), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let value = emu.read_r16(r16);
            emu.cpu.sub_register(r8, value);
            return InstructionStep::Complete;
        });
    })
}

// * RET, CALL & RST

/// Return from subroutine.
pub fn RET() -> Instruction {
    Instruction::new("RET".to_string(), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let lsb = emu.read_sp();
            return InstructionStep::new(|emu| {
                let msb = emu.read_sp();
                return InstructionStep::new(|emu| {
                    emu.cpu.set_register_pair(RegisterPair::PC, join_u16(lsb, msb));
                    return InstructionStep::Complete;
                });
            });
        });
    })
}

/// Return from subroutine if flag `c` is set.
pub fn RET_c(c: Flag) -> Instruction {
    Instruction::new(format!("RET {:?}", c), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            if emu.cpu.get_flag(c) {
                return InstructionStep::new(|emu| {
                    let lsb = emu.read_sp();
                    return InstructionStep::new(|emu| {
                        let msb = emu.read_sp();
                        return InstructionStep::new(|emu| {
                            emu.cpu.set_register_pair(RegisterPair::PC, join_u16(lsb, msb));
                            return InstructionStep::Complete;
                        });
                    });
                });
            } else {
                return InstructionStep::Complete;
            }
        });
    })
}

/// Return from subroutine if flag `c` is not set.
pub fn RET_nc(c: Flag) -> Instruction {
    Instruction::new(format!("RET N{:?}", c), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            if !emu.cpu.get_flag(c) {
                return InstructionStep::new(|emu| {
                    let lsb = emu.read_sp();
                    return InstructionStep::new(|emu| {
                        let msb = emu.read_sp();
                        return InstructionStep::new(|emu| {
                            emu.cpu.set_register_pair(RegisterPair::PC, join_u16(lsb, msb));
                            return InstructionStep::Complete;
                        });
                    });
                });
            } else {
                return InstructionStep::Complete;
            }
        });
    })
}

/// Call the immediate address `n16`.
pub fn CALL_n16() -> Instruction {
    Instruction::new("CALL n16".to_string(), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(|emu| {
                let msb = emu.read_pc();
                return InstructionStep::new(|emu| {
                    // ? "Branch decision?"
                    return InstructionStep::new(|emu| {
                        emu.write_sp(msb);
                        return InstructionStep::new(|emu| {
                            emu.write_sp(lsb);
                            return InstructionStep::Complete;
                        });
                    });
                });
            });
        });
    })
}

/// Call the immediate address `n16` if flag `c` is set.
pub fn CALL_c_n16(c: Flag) -> Instruction {
    Instruction::new(format!("CALL {:?} n16", c), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(|emu| {
                let msb = emu.read_pc();
                if emu.cpu.get_flag(c) {
                    return InstructionStep::new(|emu| {
                        // ? "Branch decision?"
                        return InstructionStep::new(|emu| {
                            emu.write_sp(msb);
                            return InstructionStep::new(|emu| {
                                emu.write_sp(lsb);
                                return InstructionStep::Complete;
                            });
                        });
                    });
                } else {
                    return InstructionStep::Complete;
                }
            });
        });
    })
}

/// Call the immediate address `n16` if flag `c` is not set.
pub fn CALL_nc_n16(c: Flag) -> Instruction {
    Instruction::new(format!("CALL N{:?} n16", c), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let lsb = emu.read_pc();
            return InstructionStep::new(|emu| {
                let msb = emu.read_pc();
                if !emu.cpu.get_flag(c) {
                    return InstructionStep::new(|emu| {
                        // ? "Branch decision?"
                        return InstructionStep::new(|emu| {
                            emu.write_sp(msb);
                            return InstructionStep::new(|emu| {
                                emu.write_sp(lsb);
                                return InstructionStep::Complete;
                            });
                        });
                    });
                } else {
                    return InstructionStep::Complete;
                }
            });
        });
    })
}

/// Call the fixed address `n16`.
pub fn RST_n16(n16: u16) -> Instruction {
    Instruction::new(format!("RST {:#X}", n16), |emu| {
        // ? Listed as 4 m-cycles long.
        let (lsb, msb) = split_u16(emu.cpu.get_register_pair(RegisterPair::PC));
        return InstructionStep::new(|emu| {
            emu.write_sp(msb);
            return InstructionStep::new(|emu| {
                emu.write_sp(lsb);
                return InstructionStep::new(|emu| {
                    emu.cpu.set_register_pair(RegisterPair::PC, n16);
                    return InstructionStep::Complete;
                });
            });
        });
    })
}

// * PUSH & POP

/// Push register pair `r16` into the stack.
pub fn PUSH_r16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("PUSH {:?}", r16), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let (lsb, msb) = split_u16(emu.cpu.get_register_pair(r16));
            return InstructionStep::new(|emu| {
                emu.write_sp(msb);
                return InstructionStep::new(|emu| {
                    emu.write_sp(lsb);
                    return InstructionStep::Complete;
                });
            });
        });
    })
}

/// Pop from the stack to register pair `r16`.
pub fn POP_r16(r16: RegisterPair) -> Instruction {
    Instruction::new(format!("POP {:?}", r16), |emu| {
        // ? One bus read or write per m-cycle.
        return InstructionStep::new(|emu| {
            let lsb = emu.read_sp();
            return InstructionStep::new(|emu| {
                let msb = emu.read_sp();
                emu.cpu.set_register_pair(r16, join_u16(lsb, msb));
                return InstructionStep::Complete;
            });
        });
    })
}
