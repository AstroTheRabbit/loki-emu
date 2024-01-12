pub mod gb;
pub mod byte_field;

use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, rc::Rc};
use winit::{
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

// fn main() -> Result<(), EventLoopError> {
//     let event_loop = EventLoop::new().expect("Unable to create window!");

//     let window = Rc::new(
//         WindowBuilder::new()
//             .with_title("Hermes Console Emulator")
//             .build(&event_loop)
//             .expect("Unable to create window!"),
//     );

//     let context = Context::new(window.clone()).expect("Unable to create window!");

//     let mut surface = Surface::new(&context, window.clone()).expect("Unable to create window!");

//     let mut input = WinitInputHelper::new();

//     event_loop.run(|event, elwt| {
//         elwt.set_control_flow(ControlFlow::Poll);

//         if input.update(&event) {
//             if input.close_requested() {
//                 elwt.exit();
//                 return;
//             }

//             let (width, height) = {
//                 let size = window.inner_size();
//                 (size.width, size.height)
//             };

//             surface
//                 .resize(
//                     NonZeroU32::new(width).unwrap(),
//                     NonZeroU32::new(height).unwrap(),
//                 )
//                 .unwrap();

//             let mut buffer = surface.buffer_mut().unwrap();
//             for index in 0..(width * height) {
//                 let x = index % width;
//                 let y = index / width;
//                 let red = x % 255;
//                 let green = 0;
//                 let blue = y % 255;

//                 buffer[index as usize] = blue | (green << 8) | (red << 16);
//             }

//             buffer.present().unwrap();
//         }
//     })
// }

fn main() {
    byte_field! {
        #[derive(Debug, Clone, Copy)]
        pub TestStruct;
        pub part_1: 1,
        part_2: 2,
        pub part_3: 4,
        part_4: 8,
    }

    let mut test_struct = TestStruct {
        part_1: [0; 1],
        part_2: [0; 2],
        part_3: [0; 4],
        part_4: [0; 8],
    };

    test_struct[0] = 42;
    println!("test_struct[0]: {}", test_struct[0]); // prints "test_struct[0]: 42"

    test_struct[3] = 99;
    test_struct[10] = 0xFF;
    println!("test_struct[3]: {}", test_struct[3]); // prints "test_struct[3]: 99"

    dbg!(test_struct);
}
