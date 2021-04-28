use glium;
use glium::backend::glutin::glutin::event::{Event, WindowEvent};
use glium::{Surface, implement_vertex};

use util;
fn main() {
    use glium::glutin;

    println!("SuperLuminal Performance Enabled: {}", superluminal_perf::enabled());

    // Set name of current thread for superluminal profiler
    superluminal_perf::set_current_thread_name("Main Thread");

    superluminal_perf::begin_event("Initialization OpenGL");
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Drawing Triangle");
    let cb = glutin::ContextBuilder::new()
        .with_hardware_acceleration(Some(true));

    let mut display = glium::Display::new(wb,cb,&event_loop)
        .expect("failed to create OpenGL Display");

    superluminal_perf::end_event();


    superluminal_perf::begin_event("setup_triangle_draw");

    #[derive(Copy, Clone)]
    struct Vertex {
        position : [f32;2]
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex{ position: [-0.5, -0.5] };
    let vertex2 = Vertex{ position:[0.0, 0.5]};
    let vertex3 = Vertex{ position: [0.5, -0.25]};
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape)
        .unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader = util::read_shader("shader/triangle/triangle.vert")
        .expect("Failed to read vertex shader");

    let fragment_shader  = util::read_shader("shader/triangle/triangle.frag")
        .expect("Failed to read fragment shader");


    let program =
        glium::Program::from_source(&display,vertex_shader.as_str(), fragment_shader.as_str(), None)
            .unwrap();

    superluminal_perf::end_event();


    event_loop.run(move|evt,_, control_flow|{

        superluminal_perf::begin_event("clear_color");

        let mut frame = display.draw();
        frame.clear_color(0.5,0.2,0.1,1.0);

        frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default())
            .unwrap();

        frame.finish();

        superluminal_perf::end_event();

        superluminal_perf::begin_event("window_handle");

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match evt{
            Event::WindowEvent {event, .. } => {

                match event{
                    WindowEvent::CloseRequested => {

                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                    _ =>{}
                }
            }

            _ => {}
        }

        superluminal_perf::end_event();
    });
}

