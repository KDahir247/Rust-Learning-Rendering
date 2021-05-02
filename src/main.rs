use glium::Surface;
use glium::backend::glutin::glutin::event::{Event, WindowEvent};

fn main() {
    println!("SuperLuminal Performance Enabled: {}", superluminal_perf::enabled());

    // Set name of current thread for superluminal profiler
    superluminal_perf::set_current_thread_name("Main Thread");

    superluminal_perf::begin_event("initialization");

    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_title("3D Model");
    let cb = glium::glutin::ContextBuilder::new()
        .with_double_buffer(Some(true))
        .with_hardware_acceleration(Some(true))
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop)
        .expect("failed to create opengl display");

    superluminal_perf::end_event();

    superluminal_perf::begin_event("getting_model_path");

    let path=  util::core::get_relative_path_file("model/SRT_Damon/dragon.obj").expect("failed to get relative path");

    superluminal_perf::end_event();

    superluminal_perf::begin_event("loading_model_to_vertex_buffer");
    //load model
    let vertex_buffer = util::core::load_wavefront_obj(&display, path.as_str(), false);

    superluminal_perf::end_event();

    superluminal_perf::begin_event("creating_vertex_shader");

    let vertex_shader = util::shader::read_shader("shader/model/model.vert")
        .expect("failed to load vertex shader");

    superluminal_perf::end_event();

    superluminal_perf::begin_event("crating_fragment_shader");

    let fragment_shader = util::shader::read_shader("shader/model/model.frag")
        .expect("failed to load fragment shader");

    superluminal_perf::end_event();

    superluminal_perf::begin_event("creating_program");

    let program = glium::Program::from_source(&display,vertex_shader.as_str(), fragment_shader.as_str(), None)
        .expect("failed to create program");

    superluminal_perf::end_event();

    let matrix = [
        [0.005, 0.0, 0.0, 0.0],
        [0.0, 0.005, 0.0, 0.0],
        [0.0, 0.0, 0.005, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    let light = [1.0, 0.4, 0.9f32];

    event_loop.run(move |evt, _, control_flow|{

        superluminal_perf::begin_event("depth_test");

        let params = glium::DrawParameters{
            depth: glium::Depth{
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },

            ..Default::default()
        };

        superluminal_perf::end_event();

        superluminal_perf::begin_event("draw_call");

        let mut frame = display.draw();
        frame.clear_color_and_depth((0.3, 0.2, 0.1, 1.0), 1.0);
        frame.draw(&vertex_buffer, &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &program,&glium::uniform! {matrix : matrix, u_light: light}, &params).unwrap();
        frame.finish();

        superluminal_perf::end_event();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(1_666_667);

        superluminal_perf::begin_event("window_handle");

        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match evt{
            Event::WindowEvent { event, .. } => {

                match event{
                    WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    },
                    _=>{}
                };
            },
            _ => {}
        }
        superluminal_perf::end_event();
    });
}

