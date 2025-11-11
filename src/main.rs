mod app;
mod game;
mod renderer;

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Duration, Instant};

use softbuffer::Surface;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::game::Game;
use crate::renderer::{Renderer, SkiaRenderer};

const WIDTH: i32 = 32;
const HEIGHT: i32 = 32;
const TIKC_DT: f32 = 1.0 / 2.0;

use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowId};

struct Context {
    window: Rc<Window>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    renderer: SkiaRenderer,
    frame_buffer: Vec<u32>,
    game: Game,
    ticker: Instant,
    key_events: Vec<KeyCode>,
}

#[derive(Default)]
struct App {
    context: Option<Context>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.context.is_some() {
            return;
        }

        let mut game = Game::new(WIDTH, HEIGHT);

        game.init();

        let attr = Window::default_attributes()
            .with_title("Rust Snake")
            .with_resizable(true);

        let window = event_loop.create_window(attr).unwrap();

        let scale = window.scale_factor() * 16.0;

        let width = (WIDTH as f64 * scale) as u32;
        let height = (HEIGHT as f64 * scale) as u32;

        let inner_size = PhysicalSize::new(width, height);

        window.request_inner_size(inner_size);

        let window = Rc::new(window);

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        println!("width: {width}, height: {height}, scale: {scale}");

        let mut renderer = SkiaRenderer::new(width, height, scale as f32);

        let mut frame_buffer = vec![0; (width * height) as usize];

        let mut ticker = Instant::now();

        let mut key_events: Vec<KeyCode> = Vec::new();

        let context = Context {
            window,
            surface,
            renderer,
            frame_buffer,
            game,
            ticker,
            key_events,
        };

        self.context = Some(context);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let mut cx = self.context.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                let (width, height) = {
                    let size = cx.window.inner_size();
                    (size.width, size.height)
                };

                cx.renderer.clear();

                cx.game.render(&mut cx.renderer);

                cx.renderer.render(&mut cx.frame_buffer);

                cx.surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = cx.surface.buffer_mut().unwrap();

                for index in 0..(width * height) {
                    buffer[index as usize] = cx.frame_buffer[index as usize];
                }

                buffer.present().unwrap();

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                // cx.window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    match key {
                        KeyCode::KeyQ | KeyCode::Escape => event_loop.exit(),
                        _ => {
                            cx.key_events.push(key);
                        }
                    }
                }
            }

            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let mut cx = self.context.as_mut().unwrap();

        for key in cx.key_events.iter() {
            cx.game.on_key(*key);
        }

        cx.key_events.clear();

        let elapsed = cx.ticker.elapsed().as_secs_f32();

        if elapsed >= TIKC_DT {
            cx.game.tick();
            cx.ticker = Instant::now();
            cx.window.request_redraw();
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.context.take();
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}

// fn main() {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new().build(&event_loop).unwrap();

//     let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

//     println!("scale_factor==> {}", window.scale_factor());

//     let scale = window.scale_factor() * 10.0;

//     let width = (WIDTH as f64 * scale) as u32;
//     let height = (HEIGHT as f64 * scale) as u32;

//     let inner_size = PhysicalSize::new(width, height);

//     window.set_inner_size(inner_size);
//     window.set_resizable(false);

//     let mut game = Game::new(WIDTH, HEIGHT);

//     game.init();

//     let mut skia_renderer = SkiaRenderer::new(width, height, scale as f32);

//     let mut frame_buffer = vec![0; (width * height) as usize];

//     let mut ticker = Instant::now();
//     let mut key_events: Vec<VirtualKeyCode> = Vec::new();

//     const TIKC_DT: f32 = 1.0 / 10.0;

//     event_loop.run(move |event, _, control_flow| {
//         control_flow.set_poll();

//         match event {
//             Event::RedrawRequested(window_id) if window_id == window.id() => {
//                 let renderer = &mut skia_renderer;

//                 renderer.clear();

//                 game.render(renderer);

//                 renderer.render(&mut frame_buffer);

//                 graphics_context.set_buffer(&frame_buffer, width as u16, height as u16);
//             }

//             Event::MainEventsCleared => {
//                 for key in key_events.iter() {
//                     game.on_key(*key);
//                 }

//                 key_events.clear();

//                 let elapsed = ticker.elapsed().as_secs_f32();

//                 if elapsed >= TIKC_DT {
//                     game.tick();

//                     ticker = Instant::now();
//                     window.request_redraw();
//                 }
//             }

//             Event::WindowEvent { event, window_id } if window_id == window.id() => match &event {
//                 WindowEvent::CloseRequested => control_flow.set_exit(),
//                 WindowEvent::KeyboardInput { event, .. } => {
//                     if let PhysicalKey::Code(key) = event.physical_key {
//                         match key {
//                             KeyCode::KeyQ | KeyCode::Escape => control_flow.set_exit(),
//                             _ => {
//                                 key_events.push(key);
//                             }
//                         }
//                     }
//                 }
//                 _ => {}
//             },
//             Event::WindowEvent {
//                 event: WindowEvent::Resized(_),
//                 window_id,
//             } if window_id == window.id() => {
//                 window.request_redraw();
//             }
//             _ => {}
//         }
//     });
// }
