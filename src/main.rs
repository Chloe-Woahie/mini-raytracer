use cgmath::{Vector2, Vector3};
use mini_raytracer::components::{Canvas, FrameCoords, Rgba, Sphere, Viewport};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

struct World {
    canvas: Canvas,
    viewport: Viewport,
    spheres: Vec<Sphere>,
    background_color: [u8; 4],
}

impl World {
    pub fn new(width: f64, height: f64, spheres: Vec<Sphere>, background_color: [u8; 4]) -> Self {
        Self {
            canvas: Canvas::new(width, height),
            viewport: Viewport::new(1.0, 1.0, 1.0),
            spheres,
            background_color,
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        // iterates through each pixel, we will need to do raytracing to find the color of each pixel
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;

            let frame_coords = FrameCoords::new(Vector2::new(x, y));

            // convert the canvas coords to the viewport coords
            let direction = frame_coords.to_viewport_coords(&self.canvas, &self.viewport);

            let rgba_opt = mini_raytracer::trace_ray(
                Vector3::new(0.0, 0.0, 0.0),
                direction,
                &self.spheres,
                1.0,
                f64::MAX,
            );
            let rgba = match rgba_opt {
                Some(_rgba) => _rgba.to_u8_array(),
                None => self.background_color,
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Basic Ray Collisions")
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
            Vector3::new(0.0, -1.0, 3.0),
            Rgba::new(255.0, 0.0, 0.0, 255.0),
            1.0,
        ),
        Sphere::new(
            Vector3::new(2.0, 0.0, 4.0),
            Rgba::new(0.0, 0.0, 255.0, 255.0),
            1.0,
        ),
        Sphere::new(
            Vector3::new(-2.0, 0.0, 4.0),
            Rgba::new(0.0, 255.0, 0.0, 255.0),
            1.0,
        ),
    ];

    let background_color = [0, 0, 0, 255];

    let mut world = World::new(WIDTH.into(), HEIGHT.into(), spheres, background_color);
    world.draw(pixels.get_frame());

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
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

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}
