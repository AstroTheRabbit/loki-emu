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
// pub fn RLC_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = (v << 1) | new_carry as u8;

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate register `r8` right, setting the carry flag to the previous bit 0.
// pub fn RRC_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | ((new_carry as u8) << 7);

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Rotate the value at `r16` left, setting the carry flag to the previous bit 7.
// pub fn RLC_r16(&mut self, r16: RegisterPair) {
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
// pub fn RRC_r16(&mut self, r16: RegisterPair) {
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
// pub fn RL_r8(&mut self, r8: Register) {
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
// pub fn RR_r8(&mut self, r8: Register) {
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
// pub fn RL_r16(&mut self, r16: RegisterPair) {
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
// pub fn RR_r16(&mut self, r16: RegisterPair) {
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
// pub fn SLA_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b1000_0000);
//     let v = v << 1;

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift register `r8` right arithmetically.
// pub fn SRA_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = (v >> 1) | (v & 0b1000_0000);

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift register `r8` right logically.
// pub fn SRL_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let new_carry = get_bit(&v, 0b0000_0001);
//     let v = v >> 1;

//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::C, new_carry);
//     self.cpu.set_flag(Flag::N | Flag::H, false);
// }

// /// Shift the value at address `r16` left arithmetically.
// pub fn SLA_r16(&mut self, r16: RegisterPair) {
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
// pub fn SRA_r16(&mut self, r16: RegisterPair) {
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
// pub fn SRL_r16(&mut self, r16: RegisterPair) {
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
// pub fn SWAP_r8(&mut self, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     let v = (v << 4) | (v >> 4);
//     self.cpu.set_register(r8, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// /// Swap the upper and lower 4 bits of the value at address `r16`.
// pub fn SWAP_r16(&mut self, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     let v = (v << 4) | (v >> 4);
//     self.bus.write(address, v);
//     self.cpu.set_flag(Flag::Z, v == 0);
//     self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
// }

// /// Set the zero flag if bit `b` of register `r8` is not set.
// pub fn BIT_b_r8(&mut self, b: u8, r8: Register) {
//     let v = self.cpu.get_register(r8);
//     self.cpu.set_flag(Flag::Z, get_bit(&v, 1 << b));
//     self.cpu.set_flag(Flag::N, false);
//     self.cpu.set_flag(Flag::H, true);
// }

// /// Set the zero flag if bit `b` of the value at address `r16` is not set.
// pub fn BIT_b_r16(&mut self, b: u8, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let v = self.bus.read(address);
//     self.cpu.set_flag(Flag::Z, get_bit(&v, 1 << b));
//     self.cpu.set_flag(Flag::N, false);
//     self.cpu.set_flag(Flag::H, true);
// }

// // * RES & SET

// /// Set bit `b` of register `r8` to 0.
// pub fn RES_b_r8(&mut self, b: u8, r8: Register) {
//     let mut v = self.cpu.get_register(r8);
//     set_bit(&mut v, 1 << b, false);
//     self.cpu.set_register(r8, v);
// }

// /// Set bit `b` of the value at address `r16` to 0.
// pub fn RES_b_r16(&mut self, b: u8, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let mut v = self.bus.read(address);
//     set_bit(&mut v, 1 << b, false);
//     self.bus.write(address, v);
// }

// /// Set bit `b` of register `r8` to 1.
// pub fn SET_b_r8(&mut self, b: u8, r8: Register) {
//     let mut v = self.cpu.get_register(r8);
//     set_bit(&mut v, 1 << b, true);
//     self.cpu.set_register(r8, v);
// }

// /// Set bit `b` of the value at address `r16` to 1.
// pub fn SET_b_r16(&mut self, b: u8, r16: RegisterPair) {
//     let address = self.cpu.get_register_pair(r16);
//     let mut v = self.bus.read(address);
//     set_bit(&mut v, 1 << b, true);
//     self.bus.write(address, v);
// }