//reminder: need to include documentation when adding new features
/// # Initialization the vertex shader.
/// specify that the Vertex parameters is called position, and the Vertex data will be the Vertex struct which is located in this file
/// <br/>
/// <br/>
/// ### #Note that calling the vertex shader parameter other than position will break the program, since it will be looking for the specific named parameter to pass in the vertex position.
pub fn initialization_vertex(){

    use glium::implement_vertex;

    implement_vertex!(Vertex,position);

}

/// # Struct for storing the vertex position to pass to the vertex shader.
///
/// contains a parameter called position which present the vertex position (x position, y position)
#[derive(Copy, Clone)]
pub struct Vertex{
    position : [f32;2],

}

impl Vertex {
    pub fn new(x: f32, y : f32) -> Vertex{

        let return_vertex = Vertex{
            position: [x, y],
        };

        return_vertex
    }
}