extern crate glium;
//extern crate glutin;

use glium::implement_vertex;
use glium::*;
use glium::texture::RawImage2d;
//use glutin::{ContextBuilder};

#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 3],
    texcoords: [f32; 2],
}
implement_vertex!(MyVertex, position, texcoords);


fn main() {
    let mut dim = glutin::dpi::LogicalSize::new(1024.0, 768.0);
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(dim)
        .with_title("Hello world");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertices = &[
        MyVertex {
            position: [-0.5, -0.5, 0.0],
            texcoords: [0.0,1.0],
        },
        MyVertex {
            position: [0.5, -0.5, 0.0],
            texcoords: [1.0,1.0],
        },
        MyVertex {
            position: [-0.5, 0.5, 0.0],
            texcoords: [0.0,0.0],
        },
        MyVertex {
            position: [0.5, 0.5, 0.0],
            texcoords: [1.0,0.0],
        },
    ];
    
    let vbo = vertex::VertexBuffer::new(&display, vertices).unwrap();
    let mut frame = display.draw();
    let indices = [0 as u32, 1, 2, 3];
    let ebo = index::IndexBuffer::new(&display, index::PrimitiveType::TriangleStrip, &indices).unwrap();
    let img = image::load_from_memory(include_bytes!("texture.jpg")).unwrap().to_rgba();
    let tex_dim: (u32, u32) = (img.width(), img.height());
    let tex = Texture2d::new(&display, RawImage2d::from_raw_rgba(img.into_vec(), tex_dim)).unwrap();
    let tex_data = glium::uniforms::Sampler::new(&tex)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
    let uniforms = uniform! {
        texData: glium::uniforms::Sampler::new(&tex)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
    };

    let shader = Program::from_source(&display, include_str!("shaders/vert.vert"), include_str!("shaders/frag.frag"), None).unwrap();

    let mut running = true;
    while running {
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
        frame.clear_color(0.0, 0.2, 0.1, 1.0);
        frame.draw(&vbo, &ebo, &shader, &uniforms, &Default::default()).unwrap();
        display.swap_buffers().unwrap();
    }
    frame.finish().unwrap();
}
                    