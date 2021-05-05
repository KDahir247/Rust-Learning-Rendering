
//reminder: need to include documentation when adding new features
/// # Retrieve the absolute path for the specified file relative to the current directory.
///
/// return the absolute path for the specified file if the file exists otherwise it will be `None`.
///
/// Exception/panic are not allowed and are not thrown to the user if it failed to parse the file.
/// <br/>
/// <br/>
/// ## Ex 1. Valid file to get relative absolute path
/// ```
/// use util::core::get_relative_path_file;
///
/// let valid_path = get_relative_path_file("image/Deku.png");
/// assert!(valid_path.is_some());
///
/// assert!(valid_path.unwrap().contains(env!("CARGO_MANIFEST_DIR")));
/// ```
/// ## Ex 2. Invalid file to get relative absolute path
/// ```
///use util::core::get_relative_path_file;
///
/// let invalid_path = get_relative_path_file("image/some_image.png");
/// assert!(invalid_path.is_none());
///
/// ```
/// ```rust,should_panic
/// use util::core::get_relative_path_file;
///
/// let invalid_path2 = get_relative_path_file("image/some_other_image.jpg");
///
/// //Should panic since it is unwrapping a None
/// invalid_path2.unwrap();
///
///
/// ```
pub fn get_relative_path_file(file: &str) -> Option<String>{

    let current_directory = env!("CARGO_MANIFEST_DIR")
        .parse::<String>()
        .expect("parsing failed : CARGO_MANIFEST_DIR failed to parse!!");

    let relative_path = format!("{}/{}", current_directory, file);

    match  std::fs::File::open(&relative_path){
        Ok(_) => { Some(relative_path) }
        Err(_) => { return None; }
    }
}

/// # Retrieve the absolute folder path within this project directory
///
/// return the absolute folder path
///
/// similar to `get_relative_path_file(file_path : &str)`, but does not return a specific file in the path just the folder path.
/// It does not check if the folder exists it just returns the path to the "folder"
pub fn get_relative_path(folder : &str) -> String{
    let current_directory = env!("CARGO_MANIFEST_DIR")
        .parse::<String>()
        .expect("parsing failed : CARGO_MANIFEST_DIR failed to parse!!");

    let relative_path = format!("{}/{}", current_directory, folder);

    relative_path
}

/// # Parses WaveFront Obj files and return a list of detailed parameters for the model. such as positions, normals, specular, shininess, texture_detail (diffuse, ambient, normal, etc...)
///
/// requires the display to create the VertexBuffer, file path to the file, and boolean to specify whether the vertex data should triangulate faces before creating a VertexBuffer from the vertex data.
///
/// return a `VertexBuffer<T>` where T can be anything, since the struct type VertexBufferAny does not know the type of data struct in the buffer
/// <br/>
/// <br/>
/// ## Ex 1. A valid obj to VertexBuffer conversion
/// ```
/// let event_loop = glium::glutin::event_loop::EventLoop::new();
/// let wb = glium::glutin::window::WindowBuilder::new()
///     .with_title("3D Model");
/// let cb = glium::glutin::ContextBuilder::new()
///     .with_double_buffer(Some(true))
///     .with_hardware_acceleration(Some(true))
///     .with_depth_buffer(24);
/// let display = glium::Display::new(wb, cb, &event_loop)
///     .expect("failed to create opengl display");
/// let valid = util::core::load_wavefront_obj(&display, std::path::Path::new("model/RubyRose/untitled.obj"), true);
///
/// ```
/// ## Ex 2. A invalid obj to VertexBuffer conversion
/// ``` rust, should_panic
///
/// let event_loop = glium::glutin::event_loop::EventLoop::new();
/// let wb = glium::glutin::window::WindowBuilder::new()
///     .with_title("3D Model");
/// let cb = glium::glutin::ContextBuilder::new()
///     .with_double_buffer(Some(true))
///     .with_hardware_acceleration(Some(true))
///     .with_depth_buffer(24);
/// let display = glium::Display::new(wb, cb, &event_loop)
///     .expect("failed to create opengl display");
///
/// let invalid = util::core::load_wavefront_obj(&display, std::path::Path::new( "model/invalid.obj"), true);
///
/// ```
pub fn load_wavefront_obj(display : &glium::Display, path : &std::path::Path, triangulate_face: bool) -> Vec<super::model::Model> {

    let mut vertex_data = Vec::new();
    let mut texture_data = Vec::default();
    let mut material_data = Vec::default();
    let mut model_container  = Vec::new();
    let mut vertex_buffer_any_array= Vec::new();

    match tobj::load_obj(path, triangulate_face) {
        Ok((models, material)) => {

            for model in &models {
                let mesh = &model.mesh;

                vertex_data.clear();

                for index in &mesh.indices {
                    let i = *index as usize;


                    let pos = [
                        mesh.positions[3 * i],
                        mesh.positions[3 * i + 1],
                        mesh.positions[3 * i + 2]
                    ];

                    let normal = if !mesh.normals.is_empty() {
                        [
                            mesh.normals[3 * i],
                            mesh.normals[3 * i + 1],
                            mesh.normals[3 * i + 2]
                        ]
                    } else {
                        [0.0, 0.0, 0.0]
                    };


                    let tex_coords = if !mesh.texcoords.is_empty() {
                        [
                            mesh.texcoords[i * 2],
                            mesh.texcoords[i * 2 + 1],
                        ]
                    } else {
                        [0.0, 0.0]
                    };

                    vertex_data.push(crate::vert!(pos, tex_coords, normal));
                }

                let current_model = glium::VertexBuffer::new(display, &vertex_data)
                    .unwrap()
                    .into();

                vertex_buffer_any_array.push(current_model);
            }

            for mat in material {

                texture_data.push(super::texture::Texture{
                    diffuse_texture:   mat.diffuse_texture,
                    ambient_texture:   mat.ambient_texture,
                    specular_texture:  mat.specular_texture,
                    shininess_texture: mat.shininess_texture,
                    normal_texture:    mat.normal_texture,
                    dissolve_texture:  mat.dissolve_texture,
                });

                material_data.push(super::material::Material{
                    diffuse:         mat.diffuse,
                    ambient:         mat.ambient,
                    specular:        mat.specular,
                    shininess:       mat.shininess,
                    dissolve:        mat.dissolve,
                    optical_density: mat.optical_density,
                });
            }

            loop{
                match vertex_buffer_any_array.pop(){
                    Some(vertex_buffer_any) => {
                        model_container.push(super::model::Model{
                            vertex: vertex_buffer_any,
                            texture: texture_data.pop(),
                            material: material_data.pop(),
                        })
                    }
                    None => {break;}
                }
            }
        }
        Err(err) => panic!("error code : {}", err),
    }

    model_container
}