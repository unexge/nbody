use rand::prelude::*;
use rgx::core::*;
use rgx::kit::shape2d::{Batch, Fill, Shape, Stroke};
use rgx::kit::{self, ZDepth};
use rgx::math::*;
use std::f64::consts::PI;
use winit::{
    event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use nbody::{
    body::{Body, G},
    simulation::{brute_force::BruteForce, Simulation},
    vec2::Vec2,
};

const SOLAR_MASS: f64 = 1.98892e30;

struct ColorfulBody(Body, Rgba);

fn main() -> Result<(), std::io::Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    let mut r = Renderer::new(&window)?;
    let mut win = window.inner_size().to_physical(window.hidpi_factor());

    let pip: kit::shape2d::Pipeline = r.pipeline(Blending::default());

    let mut textures = r.swap_chain(win.width as u32, win.height as u32, PresentMode::default());

    let mut sim = BruteForce::new();
    let mut bodies = create_bodies(500);

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(StartCause::Init) => {
            window.request_redraw();
            *control_flow = ControlFlow::Wait;
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
                VirtualKeyCode::Escape => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                win = size.to_physical(window.hidpi_factor());

                let (w, h) = (win.width as u32, win.height as u32);
                textures = r.swap_chain(w, h, PresentMode::default());
            }
            WindowEvent::RedrawRequested => {
                let (w, h) = (win.width, win.height);
                let batch = create_batch(&bodies, w, h);

                let buffer = batch.finish(&r);

                let mut frame = r.frame();

                let out = textures.next();

                r.update_pipeline(&pip, kit::ortho(out.width, out.height), &mut frame);

                {
                    let pass = &mut frame.pass(PassOp::Clear(Rgba::TRANSPARENT), &out);

                    pass.set_pipeline(&pip);
                    pass.draw_buffer(&buffer);
                }

                r.present(frame);

                sim.step(&mut bodies.iter_mut().map(|b| &mut b.0).collect(), 1e11);
                window.request_redraw();
            }
            _ => {}
        },
        _ => *control_flow = ControlFlow::Poll,
    });
}

fn create_batch(bodies: &Vec<ColorfulBody>, w: f64, h: f64) -> Batch {
    let mut batch = Batch::new();

    for body in bodies {
        batch.add(Shape::Circle(
            Point2::new(
                ((body.0.pos().x() * w / 1e18) + w / 2.0) as f32,
                ((body.0.pos().y() * h / 1e18) + h / 2.0) as f32,
            ),
            ZDepth::ZERO,
            2.0,
            32,
            Stroke::NONE,
            Fill::Solid(body.1),
        ));
    }

    batch
}

fn create_bodies(size: usize) -> Vec<ColorfulBody> {
    assert!(size > 0);

    let mut bodies = Vec::with_capacity(size);

    bodies.push(ColorfulBody(
        Body::new(Vec2::zero(), Vec2::zero(), 1e6 * SOLAR_MASS),
        Rgba::new(255.0, 0.0, 0.0, 1.0),
    ));

    for _ in 1..size {
        let pos = Vec2::new(
            1e18 * -((1.0 - rand_0_1()).ln()) / -1.8 * (0.5 - rand_0_1()),
            1e18 * -((1.0 - rand_0_1()).ln()) / -1.8 * (0.5 - rand_0_1()),
        );
        let magv = circlev(&pos);
        let abs_angle = (pos.x() / pos.y()).abs().atan();
        let thetav = PI / 2.0 - abs_angle;
        let velocity = {
            let velocity = Vec2::new(
                -1.0 * pos.y().signum() * thetav.cos() * magv,
                pos.x().signum() * thetav.sin() * magv,
            );

            if random() {
                velocity * -1.0
            } else {
                velocity
            }
        };

        let mass = rand_0_1() * SOLAR_MASS * 10.0 + 1e20;

        bodies.push(ColorfulBody(
            Body::new(pos, velocity, mass),
            Rgba::new(
                (mass * 254.0 / (SOLAR_MASS * 10.0 + 1e20)) as f32,
                (mass * 254.0 / (SOLAR_MASS * 10.0 + 1e20)) as f32,
                255.0,
                1.0,
            ),
        ));
    }

    bodies
}

fn circlev(elem: &Vec2) -> f64 {
    ((G * 1e6 * SOLAR_MASS) / (elem.x().powi(2) + elem.y().powi(2)).sqrt()).sqrt()
}

fn rand_0_1() -> f64 {
    thread_rng().gen_range(0.0, 1.0)
}
