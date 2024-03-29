// #![cfg(not(test))]
pub mod byte_field;
pub mod gb;

use std::{num::NonZeroU32, rc::Rc, time::Instant};

use gb::io::graphics::PPU;
use softbuffer::{Buffer, Context, Surface};
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::gb::{
    bus::{Bus, HRAM, WRAM},
    cartridge::Cartridge,
    cpu::CPU,
    emu::GameboyEmulator,
    instructions::instructions::Instruction,
    io::{
        graphics::{OAM, VRAM},
        io_registers::IORegisters,
    },
    utils::IME,
};

pub type RenderBuffer<'a> = Buffer<'a, Rc<Window>, Rc<Window>>;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().expect("Unable to create window!");
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Loki Emulator")
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(160, 144))
            .build(&event_loop)
            .expect("Unable to create window!"),
    );

    let context = Context::new(window.clone()).expect("Unable to create window!");

    let mut surface = Surface::new(&context, window.clone()).expect("Unable to create window!");

    let mut input = WinitInputHelper::new();

    let mut emu = GameboyEmulator {
        prev_update: Instant::now(),
        cpu: CPU::new_init(),
        ppu: PPU::new_init(),
        ime: IME::Disabled,
        is_halted: false,
        bus: Bus {
            cartridge: Cartridge::load_from_file("./roms/gb/tests/blargg/01-special.gb").unwrap(),
            vram: VRAM::new_empty(),
            wram: WRAM::new_empty(),
            oam: OAM::new_empty(),
            hram: HRAM::new_empty(),
        },
        io_registers: IORegisters::new(),
        current_instruction: Instruction::default(),
    };

    Bus::write(&mut emu, 0xFF44, 0x90);

    if let Ok(title) = emu.bus.cartridge.get_title() {
        window.set_title(format!("Loki Emulator - {title}").as_str());
    }

    event_loop.run(|event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        if input.update(&event) {
            if input.close_requested() {
                elwt.exit();
                return;
            }

            let (width, height) = {
                let size = window.inner_size();
                (size.width, size.height)
            };

            surface
                .resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                )
                .unwrap();

            let mut buffer = surface.buffer_mut().unwrap();
            emu.update(&mut input, window.clone(), &mut buffer);
            buffer.present().unwrap();
        }
    })
}
