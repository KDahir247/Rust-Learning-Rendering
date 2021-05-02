#version 140

out vec4 color;
in  vec3 my_attr;
void main(){
    color = vec4(my_attr,1.0);
}