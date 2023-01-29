mod game;
mod renderer;

use std::time::{Duration, Instant};

use softbuffer::GraphicsContext;
use winit::dpi::PhysicalSize;
use winit::event::{Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ EventLoop};
use winit::window::WindowBuilder;

use crate::game::Game;
use crate::renderer::{Renderer, SkiaRenderer};

const WIDTH: i32 = 48;
const HEIGHT: i32 = 48;

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

    let mut buffer = vec![0; (width * height) as usize];

    let mut last_frame = Instant::now();
    let mut ticker = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                if last_frame.elapsed() > Duration::from_millis(30) {
                    let renderer = &mut skia_renderer;

                    renderer.clear();

                    game.render(renderer);

                    renderer.render(&mut buffer);

                    graphics_context.set_buffer(&buffer, width as u16, height as u16);

                    last_frame = Instant::now();
                }
            }

            Event::MainEventsCleared => {
                window.request_redraw();

                if ticker.elapsed() > Duration::from_millis(100) {
                    game.tick();

                    ticker = Instant::now();
                }
            }

            Event::WindowEvent {
                window_id,
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                ..
                            },
                        ..
                    },
            } => if window_id == window.id() {
                match virtual_code {
                    winit::event::VirtualKeyCode::Q | winit::event::VirtualKeyCode::Escape => {
                        control_flow.set_exit()
                    }
                    _ => {
                        game.on_key(virtual_code);
                    }
                }
                
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                control_flow.set_exit()
            }
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
