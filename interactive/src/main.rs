use anyhow::Result;
use futures::executor::block_on;
use rendering::State;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    prepare_logging()?;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let size = window.inner_size();
    let size: (u32, u32) = (size.width.into(), size.height.into());
    let mut state = block_on(State::new_from_window(size, &window))?;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode,
                    ..
                } => {
                    println!("{:?}", virtual_keycode);
                }
                _ => {}
            },
            WindowEvent::Resized(size) => {
                state.resize((size.width.into(), size.height.into()));
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize((new_inner_size.width.into(), new_inner_size.height.into()))
            }
            _ => {}
        },
        _ => {}
    });
}

fn prepare_logging() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("wgpu", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
