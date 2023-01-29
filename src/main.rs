mod game;
mod renderer;

use std::time::{Duration, Instant};

use softbuffer::GraphicsContext;
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::game::Game;
use crate::renderer::{Renderer, SkiaRenderer};

const WIDTH: i32 = 24;
const HEIGHT: i32 = 24;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    println!("scale_factor==> {}", window.scale_factor());

    let scale = window.scale_factor() * 10.0;

    let width = (WIDTH as f64 * scale) as u32;
    let height = (HEIGHT as f64 * scale) as u32;

    let inner_size = PhysicalSize::new(width, height);

    window.set_inner_size(inner_size);
    window.set_resizable(false);

    let mut game = Game::new(WIDTH, HEIGHT);

    game.init();

    let mut skia_renderer = SkiaRenderer::new(width, height, scale as f32);

    let mut frame_buffer = vec![0; (width * height) as usize];

    let mut last_frame = Instant::now();
    let mut ticker = Instant::now();
    let mut key_events: Vec<VirtualKeyCode> = Vec::new();

    const SIM_DT: f32 = 1.0 / 60.0;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let elapsed = last_frame.elapsed().as_secs_f32();

                if elapsed >= SIM_DT {
                    let renderer = &mut skia_renderer;

                    renderer.clear();

                    game.render(renderer);

                    renderer.render(&mut frame_buffer);

                    graphics_context.set_buffer(&frame_buffer, width as u16, height as u16);
                    last_frame = Instant::now();
                }
            }

            Event::MainEventsCleared => {
                if ticker.elapsed() > Duration::from_millis(100) {
                    game.tick();

                    ticker = Instant::now();
                }

                for key in key_events.iter() {
                    game.on_key(*key);
                }

                key_events.clear();

                window.request_redraw();
            }

            Event::WindowEvent { event, window_id } if window_id == window.id() => match &event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            VirtualKeyCode::Q | VirtualKeyCode::Escape => control_flow.set_exit(),
                            _ => {
                                key_events.push(key);
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                window_id,
            } if window_id == window.id() => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
