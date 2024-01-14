pub mod byte_field;
pub mod gb;

use gb::{
    bus::{Bus, IORegisters, HRAM, WRAM},
    cartridge::CartridgeHeader,
    cpu::CPU,
    emu::GameBoyEmulator,
    graphics::{OAM, VRAM},
    utils::IME,
};
use softbuffer::{Buffer, Context, Surface};
use std::{num::NonZeroU32, rc::Rc, time::Instant};
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

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

    let mut emu = GameBoyEmulator {
        cpu: CPU::new_init(),
        ime: IME::Disabled,
        is_halted: false,
        current_cycles: 0,
        bus: Bus {
            cartridge_header: CartridgeHeader::load_from_file("./roms/Tetris.gb").unwrap(),
            vram: VRAM::new_empty(),
            wram: WRAM::new_empty(),
            oam: OAM::new_empty(),
            io_registers: IORegisters::new_empty(),
            hram: HRAM::new_empty(),
            ie_register: 0,
        },
    };

    window.set_title(
        format!(
            "Loki Emulator - {}",
            emu.bus.cartridge_header.get_title().unwrap()
        )
        .as_str(),
    );

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
            emu.update(window.clone(), &mut input, &mut buffer);
            buffer.present().unwrap();
        }
    })
}
