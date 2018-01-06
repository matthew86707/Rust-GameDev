#version 330

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

in vec2 uv;
out vec2 fragment_uv;
in vec3 position;

void main() {
	
    gl_Position = projection_matrix * inverse(view_matrix) * transform * vec4(position, 1);
    fragment_uv = uv;
}