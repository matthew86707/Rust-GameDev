#version 330

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform float shading_intensity;

in float glow_time;
in vec3 glow_position;

out float shading_i;
out vec4 positionCoord;
in vec3 position;
in vec3 normal;
in vec2 uv;
out vec2 fragment_uv;
out vec3 norm;

void main() {
	shading_i = shading_intensity;
	norm = normal;
    fragment_uv = uv;
    gl_Position = projection_matrix * inverse(view_matrix) * transform * vec4(position, 1);
    positionCoord = view_matrix * inverse(projection_matrix) * gl_Position;
    //positionCoord = gl_Position;
}