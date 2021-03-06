#version 140

uniform mat4 transform;

in vec3 position;
in vec2 uv;
out vec2 fragment_uv;
uniform float zoom_uniform;
uniform float mouseX;
uniform float mouseY;

void main() {
    fragment_uv = uv;
    gl_Position = vec4(position, 1);
}