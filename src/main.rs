use glium::backend::glutin::glutin::event::{Event, WindowEvent};
use glium::{Surface, Display};
use std::io::Read;

fn main() {

    println!("SuperLuminal Performance Enabled: {}", superluminal_perf::enabled());

    // Set name of current thread for superluminal profiler
    superluminal_perf::set_current_thread_name("Main Thread");


    superluminal_perf::begin_event("initialization");

    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_title("texture triangle");
    let cb = glium::glutin::ContextBuilder::new()
        .with_double_buffer(Some(true))
        .with_hardware_acceleration(Some(true));

    let display =  glium::Display::new(wb, cb, &event_loop)
        .expect("failed to create OpenGL Display");

    superluminal_perf::end_event();

    //load our image

    superluminal_perf::begin_event("load_image_and_texture");

    let absolute_path_to_image = util::core::get_relative_path_file("/image/Deku.png").unwrap();

    let image = image::open(&std::path::Path::new(absolute_path_to_image.as_str())).unwrap().to_rgba16();

    let image_dimensions = image.dimensions();

    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image)
        .unwrap();

    superluminal_perf::end_event();


    superluminal_perf::begin_event("creating_vertices");
    //create our vertex
    util::vertex::initialization_vertex();

    let vertex1  = util::vert!([-0.5, -0.5], [0.0, 0.0]);
    let vertex2 = util::vert!([0.5, -0.5], [1.0, 0.0]);
    let vertex3 = util::vert!([0.0, 0.5], [0.0, 1.0]);

    let shape = vec![vertex1,vertex2,vertex3];

    superluminal_perf::end_event();

    //create our vertex buffer and index buffer

    superluminal_perf::begin_event("creating_vertex_and_index_buffer");

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape)
        .expect("Failed to create vertex buffer");

    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    superluminal_perf::end_event();

    superluminal_perf::begin_event("linking_shaders");

    //link our vertex_shader and fragment_shader

    let vertex_shader = util::shader::read_shader("shader/texture/texture.vert")
        .expect("failed to load or read vertex shader");

    let fragment_shader = util::shader::read_shader("shader/texture/texture.frag")
        .expect("failed to load or read fragment shader");

    superluminal_perf::end_event();

    superluminal_perf::begin_event("creating_program");

    //create our shader program

    let program = glium::program::Program::from_source(&display, &vertex_shader.as_str(), fragment_shader.as_str(),None)
        .expect("failed to create program");

    superluminal_perf::end_event();

    event_loop.run(move |evt, _, control_flow|{

        superluminal_perf::begin_event("crating_uniforms");

        let uniforms = glium::uniform! {
        matrix : [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex : &texture
    };

        superluminal_perf::end_event();


        superluminal_perf::begin_event("draw_call");

        let mut frame = display.draw();
        frame.clear_color(0.3,0.2,0.1, 1.0);
        frame.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default());
        frame.finish();

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

    });

}

