#version 140

in vec3 position;
out  vec3 my_attr;
uniform mat4 matrix;

void main(){
    my_attr = position;
    gl_Position = matrix * vec4(position,1.0);
}