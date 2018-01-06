#version 330

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform float shading_intensity;

in vec3 position;

void main() {
	
    gl_Position = projection_matrix * inverse(view_matrix) * transform * vec4(position, 1);

}