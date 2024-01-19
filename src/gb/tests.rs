#[test]
#[cfg(test)]
fn jsmoo_instruction_tests() -> Result<(), Box<dyn std::error::Error>> {
    use crate::gb::{
        bus::{Bus, HRAM, WRAM},
        cartridge::Cartridge,
        cpu::CPU,
        emu::GameboyEmulator,
        instructions::instructions::Instruction,
        io::{
            graphics::{OAM, PPU, VRAM},
            io_registers::IORegisters,
        },
        utils::*,
    };
    use serde::Deserialize;
    use std::{fs, time::Instant};

    #[derive(Debug, Deserialize)]
    struct JsmooTest {
        pub name: String,
        pub initial: JsmooTestState,
        pub r#final: JsmooTestState,
        pub cycles: Vec<(u16, Option<u8>, String)>,
    }

    #[derive(Debug, Deserialize)]
    struct JsmooTestState {
        pub pc: u16,
        pub sp: u16,
        pub a: u8,
        pub b: u8,
        pub c: u8,
        pub d: u8,
        pub e: u8,
        pub f: u8,
        pub h: u8,
        pub l: u8,
        pub ram: Vec<(u16, u8)>,
    }

    impl From<&JsmooTestState> for GameboyEmulator {
        fn from(value: &JsmooTestState) -> Self {
            let mut emu = Self {
                prev_update: Instant::now(),
                cpu: CPU::from(value),
                ppu: PPU::new_init(),
                ime: IME::Disabled,
                bus: Bus {
                    cartridge: Cartridge::new_empty(),
                    vram: VRAM::new_empty(),
                    wram: WRAM::new_empty(),
                    oam: OAM::new_empty(),
                    hram: HRAM::new_empty(),
                },
                is_halted: false,
                io_registers: IORegisters::new(),
                current_instruction: Instruction::default(),
            };
            Bus::reset_test_ram();
            for (address, value) in &value.ram {
                Bus::write(&mut emu, *address, *value);
            }
            return emu;
        }
    }

    impl From<&JsmooTestState> for CPU {
        fn from(value: &JsmooTestState) -> Self {
            Self::new(
                value.a, value.f, value.b, value.c, value.d, value.e, value.h, value.l, value.pc,
                value.sp,
            )
        }
    }

    #[inline]
    fn test_error(expected: &JsmooTest, result: &mut GameboyEmulator) {
        let expected_cpu = CPU::from(&expected.r#final);

        let mut ram = Vec::with_capacity(expected.r#final.ram.len());
        for (address, expected) in &expected.r#final.ram {
            let result = Bus::read(result, *address);
            ram.push((*address, *expected, result));
        }
        panic!(
            "Jsmoo test {} failed!\n\nExpected CPU: {:#X?}\nResult CPU: {:#X?}\nRAM: {:#X?}",
            expected.name, expected_cpu, result.cpu, ram,
        );
    }

    let mut tests_dir = fs::read_dir("./roms/gb/tests/jsmoo/tests")?
        .filter_map(|p| p.ok())
        .collect::<Vec<_>>();
    tests_dir.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for file_path in tests_dir {
        let file = fs::OpenOptions::new().read(true).open(file_path.path())?;
        let file_buf = std::io::BufReader::new(file);
        let tests: Vec<JsmooTest> = serde_json::from_reader(file_buf)?;

        for test in tests {
            let mut emu = GameboyEmulator::from(&test.initial);
            for _ in 0..test.cycles.len() {
                if emu.is_halted {
                    continue;
                }

                // ? Get the next instruction if the previous instruction has completed.
                if emu.current_instruction.has_completed() {
                    emu.current_instruction = emu.read_pc().into();
                }

                // ? Run the current instruction.
                let mut instruction = std::mem::take(&mut emu.current_instruction);
                instruction.step(&mut emu);
                emu.current_instruction = instruction;
            }

            if test.r#final.pc != emu.cpu.get_register_pair(RegisterPair::PC)
                || test.r#final.sp != emu.cpu.get_register_pair(RegisterPair::SP)
                || test.r#final.a != emu.cpu.get_register(Register::A)
                || test.r#final.b != emu.cpu.get_register(Register::B)
                || test.r#final.c != emu.cpu.get_register(Register::C)
                || test.r#final.d != emu.cpu.get_register(Register::D)
                || test.r#final.e != emu.cpu.get_register(Register::E)
                || test.r#final.f != emu.cpu.get_register(Register::F)
                || test.r#final.h != emu.cpu.get_register(Register::H)
                || test.r#final.l != emu.cpu.get_register(Register::L)
            {
                test_error(&test, &mut emu);
            }

            for (address, value) in &test.r#final.ram {
                if Bus::read(&mut emu, *address) != *value {
                    test_error(&test, &mut emu);
                }
            }
        }
        println!("Jsmoo test file {:#?} passed...", file_path.file_name());
    }

    Ok(())
}
