use log::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
  dpi::LogicalSize,
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use super::{clock::Clock, config::Config};

pub struct Engine {
  pub window: Window,
  pub fb: Pixels,
  pub eye: (i32, i32),
  pub input: WinitInputHelper,
  pub config: Config,
  pub clock: Clock,
}

impl Engine {
  pub const HEIGHT: u32 = 256;
  pub const WIDTH: u32 = 256;

  pub fn new() -> (Self, EventLoop<()>) {
    flexi_logger::Logger::try_with_str("game")
      .expect("Failed to initialize logger")
      .start()
      .expect("Failed to start logger");

    let config = Config::load().unwrap_or_else(|e| {
      error!("Error loading config file: {}", e);
      error!("Using default settings");
      Config::default()
    });

    let event_loop = EventLoop::new();

    let size = LogicalSize::new(Self::WIDTH as f64, Self::HEIGHT as f64);

    let window = WindowBuilder::new()
      .with_title("Hello Pixels")
      .with_inner_size(size)
      .with_min_inner_size(size)
      .build(&event_loop)
      .unwrap();

    let inner_size = window.inner_size();

    let surface_texture =
      SurfaceTexture::new(inner_size.width, inner_size.height, &window);

    let fb = Pixels::new(Self::WIDTH, Self::HEIGHT, surface_texture).unwrap();
    (
      Self {
        window,
        fb,
        eye: (0, 0),
        input: WinitInputHelper::new(),
        config,
        clock: Clock::new(),
      },
      event_loop,
    )
  }
}

pub type StartHook<St> = fn(engine: &mut Engine) -> St;
pub type InputHook<St> = fn(state: &mut St, engine: &Engine);
pub type DrawHook<St> = fn(state: &St, engine: &mut Engine);
pub type EndHook<St> = fn(state: &mut St, engine: &mut Engine);

pub trait State: 'static {
  fn new() -> Self;
}

pub fn launch<St: State>(
  start_hook: StartHook<St>,
  input_hook: InputHook<St>,
  draw_hook: DrawHook<St>,
  end_hook: EndHook<St>,
) {
  let (mut engine, event_loop) = Engine::new();
  let mut state = (start_hook)(&mut engine);
  event_loop.run(move |event, _, control| {
    if engine.input.update(&event) {
      if let Some(size) = engine.input.window_resized() {
        engine.fb.resize_surface(size.width, size.height).unwrap();
      }
      engine.clock.tick();
      (input_hook)(&mut state, &engine);
    }

    if let winit::event::Event::RedrawRequested(_) = event {
      (draw_hook)(&state, &mut engine);
      engine.fb.render().unwrap();
    }

    if let winit::event::Event::WindowEvent {
      event: winit::event::WindowEvent::CloseRequested,
      ..
    } = event
    {
      (end_hook)(&mut state, &mut engine);
      control.set_exit();
    }

    engine.window.request_redraw();
  });
}
