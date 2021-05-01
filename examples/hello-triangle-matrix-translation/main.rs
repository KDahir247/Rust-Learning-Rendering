use util;
use glium::backend::glutin::glutin::event::{Event, WindowEvent};
use glium::{Surface};

fn main() {

    println!("SuperLuminal Performance Enabled: {}", superluminal_perf::enabled());
    // Set name of current thread for superluminal profiler
    superluminal_perf::set_current_thread_name("Main Thread");

    let mut t = 0.0;

    superluminal_perf::begin_event("initialization_application");

    let event_loop = glium::glutin::event_loop::EventLoop::new();

    let wb = glium::glutin::window::WindowBuilder::new()
        .with_title("Triangle Matrix Manipulation");

    let cb = glium::glutin::ContextBuilder::new()
        .with_hardware_acceleration(Some(true))
        .with_double_buffer(Some(true));

    let display = glium::Display::new(wb, cb, &event_loop)
        .expect("Failed on creating display");

    superluminal_perf::end_event();

    superluminal_perf::begin_event("creating_vertices");

    util::vertex::initialization_vertex();

    util::vert!([0., 0.5]);

    let vertex1 = util::vert!([-0.5,-0.5]);
    let vertex2 = util::vert!([0.5, -0.5]);
    let vertex3 = util::vert!([0., 0.5]);


    let triangle = vec![vertex1, vertex2, vertex3];

    superluminal_perf::end_event();

    superluminal_perf::begin_event("creating_vertex_and_index_buffer");
    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle)
        .expect("failed to create vertex buffer");

    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    superluminal_perf::end_event();

    superluminal_perf::begin_event("linking_shader");

    let vertex_shader = util::shader::read_shader("shader/matrix-triangle/matrix-triangle.vert")
        .expect("Failed to read or load vertex shader");

    let fragment_shader = util::shader::read_shader("shader/matrix-triangle/matrix-triangle.frag")
        .expect("Failed to read or load fragment shader");

    let program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(),None)
        .expect("failed to crete shader program");

    superluminal_perf::end_event();

    event_loop.run(move |evt, _, control_flow|{

        superluminal_perf::begin_event("math_calculations");

        let lpp = util::math::lerp(-0.5,0.5, util::math::ping_pong(t, 1.));
        t += 0.0001;

        superluminal_perf::end_event();

        superluminal_perf::begin_event("draw_call");

        let mut frame = display.draw();
        frame.clear_color(0.3,0.2,0.1,1.0);

        let uniform  = glium::uniform! {
            matrix : [
                [lpp.cos(), lpp.sin(), 0.0, 0.0],
                [-lpp.sin(),lpp.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0., 0.0, 0.0, 1.0f32] //we need the f32, or it will interpret it as f64
            ]
        };

        frame.draw(&vertex_buffer,&index_buffer,&program, &uniform, &Default::default())
            .unwrap();

        frame.finish()
            .unwrap();

        superluminal_perf::end_event();

        let next_frame_time : std::time::Instant = std::time::Instant::now() + std::time::Duration::from_nanos(1_666_667);

        superluminal_perf::begin_event("window_handle");

        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match evt {
            Event::WindowEvent { event, .. } => {

                match event{
                    WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        superluminal_perf::end_event();
    })
}

