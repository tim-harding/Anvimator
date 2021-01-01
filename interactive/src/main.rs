use anyhow::Result;
use futures::executor::block_on;
use rendering::State;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// Todo: test rust-gpu
// https://embarkstudios.github.io/rust-gpu/book/writing-shader-crates.html

type WinitSize = winit::dpi::PhysicalSize<u32>;

fn main() -> Result<()> {
    prepare_logging()?;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let mut state = create_state(&window)?;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, .. } => {
            handle_window_event(event, control_flow, &mut state)
        }
        Event::RedrawRequested(_) => handle_redraw(control_flow, &mut state),
        Event::MainEventsCleared => {} // window.request_redraw(),
        _ => {}
    });
}

fn handle_redraw(control_flow: &mut ControlFlow, state: &mut State) {
    state.update();
    if let Err(e) = state.render() {
        log::error!("{}", e);
        *control_flow = ControlFlow::Exit;
    }
}

fn handle_window_event(event: &WindowEvent, control_flow: &mut ControlFlow, state: &mut State) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput { input, .. } => handle_keyboard(*input),
        WindowEvent::Resized(size) => handle_resize(state, *size),
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            handle_resize(state, **new_inner_size)
        }
        _ => {}
    }
}

fn handle_resize(state: &mut State, size: WinitSize) {
    let size = translate_size(size);
    state.resize(size);
}

fn handle_keyboard(input: KeyboardInput) {
    match input {
        KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode,
            ..
        } => {
            println!("{:?}", virtual_keycode);
        }
        _ => {}
    }
}

fn create_state(window: &Window) -> Result<State> {
    let size = translate_size(window.inner_size());
    block_on(State::new_from_window(size, window))
}

fn translate_size(size: WinitSize) -> (u32, u32) {
    (size.width.into(), size.height.into())
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
        .level(log::LevelFilter::Warn)
        .level_for("interactive", log::LevelFilter::Trace)
        .level_for("rendering", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
