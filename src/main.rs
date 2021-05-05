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

    superluminal_perf::begin_event("load_model_data");

    let path=  util::core::get_relative_path_file("./model/RubyRose/untitled.obj")
        .expect("failed to get relative path");

    let models = util::core::load_wavefront_obj(&display, std::path::Path::new(path.as_str()), false);

    superluminal_perf::end_event();

    superluminal_perf::begin_event("load_texture_image");

    let mut diffuse_textures = Vec::new();
    let mut normal_textures = Vec::new();

    let path = util::core::get_relative_path("./model/RubyRose");

    for index in 0..models.len(){

        let texture = models[index]
            .texture
            .as_ref()
            .unwrap();

        if !texture.diffuse_texture.is_empty() {

            let absolute_diffuse_path =  format!("{}//{}", path, texture.diffuse_texture);

            let texture_path = std::path::Path::new(absolute_diffuse_path.as_str());

            let image = image::open(texture_path)
                .unwrap()
                .to_rgba16();

            let image_dimensions = image.dimensions();

            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            diffuse_textures
                .push(glium::texture::SrgbTexture2d::new(&display, image).unwrap());
        }

        if !texture.normal_texture.is_empty(){

            let absolute_normal_path = format!("{}//{}", path, texture.normal_texture);

            let texture_path = std::path::Path::new(absolute_normal_path.as_str());

            let image = image::open(texture_path)
                .unwrap()
                .to_rgba16();

            let image_dimensions = image.dimensions();

            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            normal_textures.push(glium::texture::Texture2d::new(&display, image).unwrap());
        }
    }



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
        [1., 0.0, 0.0, 0.0],
        [0.0, 1., 0.0, 0.0],
        [0.0, 0.0, 1., 0.0],
        [0.0, -1.0, -7.0, 1.0f32]
    ];

    let light = [-0.5, -0.4, 0.5f32];

    event_loop.run(move |evt, _, control_flow|{

        superluminal_perf::begin_event("depth_test");

        let params = glium::DrawParameters{
            depth: glium::Depth{
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling : glium::BackfaceCullingMode::CullClockwise,

            ..Default::default()
        };

        superluminal_perf::end_event();

        superluminal_perf::begin_event("draw_call");

        let mut frame = display.draw();

        frame.clear_color_and_depth((0.3, 0.2, 0.1, 1.0), 1.0);

        let aspect_ratio = frame.get_dimensions();

        let perspective = {

            let (width, height) = frame.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;

            let fov : f32 = std::f32::consts::PI / 3.0; //60 fov
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            let perspective_matrix =  nalgebra_glm::perspective(aspect_ratio, fov,znear,zfar);

            perspective_matrix.data.0
        };

        let view = {
            let v = nalgebra_glm::make_vec3(&[0.,0.0, 0.1f32]);
            let view_matrix = nalgebra_glm::translation(&v);

            [
                view_matrix.data.0[0],
                view_matrix.data.0[1],
                view_matrix.data.0[2],
                view_matrix.data.0[3]
            ]
        };
        for index in 0..models.len() {

            //some model might not have a diffuse texture, so we clamp the index to the maximum diffuse_textures length - 1
            let texture_index = util::math::clamp(index as f32, 0., (diffuse_textures.len() - 1) as f32) as usize;

            frame.draw(&models[index].vertex,
                       &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                       &program,
                       &glium::uniform! {
                           matrix : matrix,
                           perspective: perspective,
                           view: view,
                           u_light: light,
                           diffuse_tex: &diffuse_textures[texture_index],
                           normal_tex : &normal_textures[texture_index]
                       },
                       &params)
                .unwrap();
        }
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

