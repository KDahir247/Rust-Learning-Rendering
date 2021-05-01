#version 140

out vec4 color;
in  vec2 my_attr;
void main(){
    color = vec4(my_attr,0.0,1.0);
}