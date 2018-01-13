#version 330

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

in vec2 uv;
out vec3 fragment_uv;
in vec3 position;

void main() {
    gl_Position = projection_matrix * inverse(view_matrix) * vec4(position * 2000.0, 1.0);
    fragment_uv = position;
}