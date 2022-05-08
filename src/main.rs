use cgmath::Vector3;
use mini_raytracer::components::{Camera, Light, Rgba, Sphere};
use mini_raytracer::World;
use pixels::{Error, Pixels, SurfaceTexture};
use std::io::{self, Write};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const REFLECTION_PASSES: u32 = 3;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("mini-raytracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let spheres = vec![
        Sphere::new(
            Vector3::new(0.0, -1.0, 4.0),
            Rgba::new(255.0, 0.0, 0.0, 255.0),
            1.0,
            // shiny
            500.0,
            // 20% reflective
            0.2,
        ),
        Sphere::new(
            Vector3::new(2.0, 0.0, 5.0),
            Rgba::new(0.0, 0.0, 255.0, 255.0),
            1.0,
            // shiny
            500.0,
            // 30% reflective
            0.3,
        ),
        Sphere::new(
            Vector3::new(-2.0, 0.0, 5.0),
            Rgba::new(0.0, 255.0, 0.0, 255.0),
            1.0,
            // somewhat shiny
            10.0,
            //40% reflective
            0.4,
        ),
        Sphere::new(
            Vector3::new(0.0, -5001.2, 0.0),
            Rgba::new(100.0, 100.0, 100.0, 255.0),
            5000.0,
            // very shiny
            1000.0,
            // 50% reflective
            0.5,
        ),
    ];

    let lights = vec![
        Light::Ambient(0.2),
        Light::Point(0.6, Vector3::new(2.0, 1.0, 0.0)),
        Light::Directional(0.2, Vector3::new(1.0, 4.0, 4.0)),
    ];

    let background_color = Rgba::new(0.0, 0.0, 0.0, 255.0);

    let camera = Camera::new(Vector3::new(0.0, 0.0, 0.0));

    let world = World::new(
        WIDTH.into(),
        HEIGHT.into(),
        REFLECTION_PASSES,
        camera,
        spheres,
        lights,
        background_color,
    );

    let mut frames = 0;
    let mut last_frame_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            frames += 1;
            let elapsed = last_frame_update.elapsed().as_millis();
            if elapsed >= 1000 {
                last_frame_update = Instant::now();
                print!("\rFps: {}", frames);
                io::stdout().flush().unwrap();
                frames = 0
            }
            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // 3 the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}
