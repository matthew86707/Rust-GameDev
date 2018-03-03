#version 330

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

in vec2 uv;
in vec3 position;

out vec2 fragment_uv;
out vec4 positionCoord;

void main() {
	
    gl_Position = projection_matrix * inverse(view_matrix) * transform * vec4(position, 1);
    positionCoord = view_matrix * inverse(projection_matrix) * gl_Position;
    fragment_uv = uv;
}