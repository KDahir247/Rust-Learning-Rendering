#version 140

in vec3 position;

uniform mat4 matrix;
in vec2 tex_coord;
out vec2 v_tex_coord;

void main() {

    v_tex_coord = tex_coord;
    gl_Position = matrix * vec4(position, 1.0f);

}