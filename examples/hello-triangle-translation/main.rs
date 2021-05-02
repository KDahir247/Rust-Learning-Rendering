use util;
use glium::backend::glutin::glutin::event::{Event, WindowEvent};
use glium::{Surface};

fn main() {

    println!("SuperLuminal Performance Enabled: {}", superluminal_perf::enabled());
    // Set name of current thread for superluminal profiler
    superluminal_perf::set_current_thread_name("Main Thread");

    //specify the "custom" elapsed time.
    let mut elapsed_time: f32 = 0.;


    superluminal_perf::begin_event("Initialization_OpenGL");

    let mut event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_title("Moving Triangle");
    let cb = glium::glutin::ContextBuilder::new()
        .with_hardware_acceleration(Option::from(true));

    let mut display = glium::Display::new(wb,cb,&event_loop)
        .expect("Failed to create display");

    superluminal_perf::end_event();


    superluminal_perf::begin_event("creating_vertices");
    //creating vertex buffer
    util::vertex::initialization_vertex();


    let vertex1 = util::vert!([-0.5, 0.25, 0.0]);;
    let vertex2 = util::vert!([0.5, -0.5, 0.0]);
    let vertex3 = util::vert!([0., 0.5, 0.0]);;

    let shapes = vec![vertex1, vertex2, vertex3];

    superluminal_perf::end_event();

    superluminal_perf::begin_event("creating_indices_and_vert_buff");

    let vertex_buffer = glium::VertexBuffer::new(&display, &shapes)
        .unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    superluminal_perf::end_event();


    superluminal_perf::begin_event("loading_shaders");

    let vertex_shader = util::shader::read_shader("shader/moving-triangle/moving-triangle.vert")
        .expect("Failed to parse or locate vertex shader");


    let fragment_shader = util::shader::read_shader("shader/moving-triangle/moving-triangle.frag")
        .expect("Failed to parse or locate fragment shader");

    superluminal_perf::end_event();

    superluminal_perf::begin_event("creating_program");

    let program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(), None)
        .unwrap();

    superluminal_perf::end_event();

    event_loop.run(move |evt, _,  control_flow| {

        superluminal_perf::begin_event("lerp_ping_pong");

        let t = util::math::lerp(-0.5, 0.5, util::math::ping_pong(elapsed_time, 1.0));
        elapsed_time += 0.0007;

        superluminal_perf::end_event();


        superluminal_perf::begin_event("draw_call");

        let mut frame = display.draw();
        frame.clear_color(0.3,0.1,0.2, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &glium::uniform! {t: t}, &Default::default());
        frame.finish();

        superluminal_perf::end_event();

        let next_frame_time  = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        superluminal_perf::begin_event("window_event_handle");

        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match evt{
            Event::WindowEvent {event, .. } => {

                match event{
                    WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    }
                    _ =>{}
                }
            }
            _ =>{}
        }

        superluminal_perf::end_event();

    });
}

