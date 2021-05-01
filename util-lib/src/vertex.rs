//reminder: need to include documentation when adding new features

/// # Initialization the vertex shader.
/// specify that the Vertex parameters is called position, and tex_coords, and the Vertex data will be the Vertex struct which is located in this file
/// <br/>
/// <br/>
/// ### #Note that calling the vertex shader parameter other than the parameter names in Vertex will break the program, since it will be looking for the specific named parameter to pass in the vertex position.
pub fn initialization_vertex(){

    use glium::implement_vertex;

    implement_vertex!(Vertex,position, tex_coord);

}

/// # Struct for storing the vertex position to pass to the vertex shader.
///
/// contains a parameter called position which present the vertex position (x position, y position)
#[derive(Copy, Clone,Debug)]
pub struct Vertex{
    position : [f32;2],
    tex_coord : [f32;2]

}

impl Vertex {
    pub fn new(position: [f32;2], tex_coord : [f32;2],) -> Vertex{

        let return_vertex = Vertex{
            position,
            tex_coord
        };

        return_vertex
    }

    pub fn has_tex_coords(&self) -> bool{
        (self.tex_coord[0] != 0.) || (self.tex_coord[1] != 0.)
    }

}

/// #Macro for easily handling Vertex creation
///
/// The macro has two variants, one for populating the vertex with the position and the tex_coords.
/// The other variant is used to only populate the position of the vertex.
///
/// required parameter;
///
/// Variant:1 Vertex position, Vertex tex coords
///
/// Variant:2 Vertex position
///
///
/// return a Vertex.
/// <br/>
/// <br/>
/// ##Ex 1. Position
/// ```
/// let vertex = util::vert!([2.23, 3.12]);
/// assert!(!vertex.has_tex_coords());
///
/// ```
/// ##Ex 2. Position and Texture Coords
/// ```
/// // order for arguments matter (position, tex_coord)
/// let vertex = util::vert!([12.123, 2.01], [0.0, 1.0]);
/// assert!(vertex.has_tex_coords());
/// ```
#[macro_export]
 macro_rules! vert {
    ($position : expr, $tex_coords : expr) => {
        $crate::vertex::Vertex::new($position, $tex_coords)
    };
    ($position : expr) => {
        $crate::vertex::Vertex::new($position, [0.,0.])
    }
}