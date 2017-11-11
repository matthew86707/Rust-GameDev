#version 140

uniform mat4 projection_matrix;
uniform mat4 view_matrix;

out vec3 ReflectDir;
in vec3 position;


void main() {
    ReflectDir = position;
    gl_Position = projection_matrix * view_matrix * vec4(position, 1);
}