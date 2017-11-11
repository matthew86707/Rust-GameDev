#version 330

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

out vec4 positionCoord;
in vec3 position;
in vec2 uv;
out vec2 fragment_uv;

void main() {
    fragment_uv = uv;
    gl_Position = projection_matrix * view_matrix * transform * vec4(position, 1);
    positionCoord = inverse(view_matrix) * inverse(projection_matrix) * gl_Position;
    //positionCoord = gl_Position;
}