extern crate glium;
//extern crate glutin;
use std::time::{Duration, SystemTime};
use glium::implement_vertex;
use glium::uniforms::SamplerWrapFunction;
use glium::*;
use glium::texture::RawImage2d;
//use glutin::{ContextBuilder};

#[derive(Copy, Clone)]
struct GridVertex {
    position: [f32; 3],
    texcoords: [f32; 2],
}
implement_vertex!(GridVertex, position, texcoords);

#[derive(Copy, Clone)]
struct BackgroundVertex {
    position: [f32; 3],
    color: [f32; 3],
}
implement_vertex!(BackgroundVertex, position, color);


fn main() {
    let program_start_time = SystemTime::now();
    let mut dim = glutin::dpi::LogicalSize::new(1024.0, 768.0);
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(dim)
        .with_title("OUTRUN");
    let context = glutin::ContextBuilder::new().with_vsync(false);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let grid_vertices = &[
        GridVertex {
            position: [-1.0, 0.0, -1.0],
            texcoords: [0.0,1.0],
        },
        GridVertex {
            position: [1.0, 0.0, -1.0],
            texcoords: [1.0,1.0],
        },
        GridVertex {
            position: [-1.0, 0.0, 0.0],
            texcoords: [0.0,0.0],
        },
        GridVertex {
            position: [1.0, 0.0, 0.0],
            texcoords: [1.0,0.0],
        },
    ];
    let background_vertices = &[
        BackgroundVertex {
            position: [-1.0, 1.0, 0.0],
            color: [0.0, 0.0, 0.0],
        },
        BackgroundVertex {
            position: [1.0, 1.0, 0.0],
            color: [0.0, 0.0, 0.0],
        },
        BackgroundVertex {
            position: [-1.0, -1.0, 0.0],
            color: [0.1, 0.0, 0.1],
        },
        BackgroundVertex {
            position: [1.0, -1.0, 0.0],
            color: [0.1, 0.0, 0.1],
        },
    ];
    
    let grid_vbo = vertex::VertexBuffer::new(&display, grid_vertices).unwrap();
    let background_vbo = vertex::VertexBuffer::new(&display, background_vertices).unwrap();
    let mut frame = display.draw();
    let indices = [0 as u32, 1, 2, 3];
    let ebo = index::IndexBuffer::new(&display, index::PrimitiveType::TriangleStrip, &indices).unwrap();
    let img = image::load_from_memory(include_bytes!("Untitled.png")).unwrap().to_rgba();
    let tex_dim: (u32, u32) = (img.width(), img.height());
    let tex = Texture2d::new(&display, RawImage2d::from_raw_rgba(img.into_vec(), tex_dim)).unwrap();
    let tex_data = glium::uniforms::Sampler::new(&tex)
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        .wrap_function(SamplerWrapFunction::Repeat)
        .anisotropy(16);
    let grid_transform:[[f32; 4]; 4] = cgmath::Matrix4::from_translation([0.0, -0.008, 0.0].into()).into();
    let perspective:[[f32; 4]; 4] = cgmath::perspective(cgmath::Deg(75.0), 1024.0 / 768.0, 0.01, 100.0).into();
    let back_colour:[f32; 3] = [0.1, 0.0, 0.1];
    let grid_uniforms = uniform! {
        texData: tex_data,
        perspective: perspective,
        model: grid_transform,
        backColour: back_colour,
        time: SystemTime::now().duration_since(program_start_time).unwrap().as_micros() as f32 / 1000000.0 % 1.0,
    };
    let background_transform:[[f32; 4]; 4] = cgmath::Matrix4::from_translation([0.0, 0.0, -3.0].into()).into();
    let background_uniforms = uniform! {
        model: background_transform,
        perspective: perspective,
    };

    let grid_shader = Program::from_source(&display, include_str!("grid_shaders/vert.vert"), include_str!("grid_shaders/frag.frag"), None).unwrap();
    let background_shader = Program::from_source(&display, include_str!("background_shaders/vert.vert"), include_str!("background_shaders/frag.frag"), None).unwrap();

    let mut running = true;
    let mut framecounter = 0;
    let mut frame_thingy = SystemTime::now();
    while running {
        let frame_render_time = SystemTime::now();
        events_loop.poll_events(|event| {
            //println!("{:?}", event);
            match event {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    _ => {},
                }
                _ => {},
            }
        });
        let grid_uniforms = uniform! {
            texData: tex_data,
            perspective: perspective,
            model: grid_transform,
            backColour: back_colour,
            time: (SystemTime::now().duration_since(program_start_time).unwrap().as_micros() as f32 / 1000000.0)  % 1.0,
        };
        framecounter += 1;
        if frame_render_time.duration_since(frame_thingy).unwrap() >= Duration::from_secs(1) {
            println!("{}", (framecounter as f64) / 1.0);
            framecounter = 0;
            frame_thingy = SystemTime::now();
        }
        //frame.clear_color(0.01, 0.0, 0.01, 1.0);
        frame.draw(&background_vbo, &ebo, &background_shader, &background_uniforms, &Default::default()).unwrap();
        frame.draw(&grid_vbo, &ebo, &grid_shader, &grid_uniforms, &Default::default()).unwrap();
        display.swap_buffers().unwrap();
    }
    frame.finish().unwrap();
}
                    