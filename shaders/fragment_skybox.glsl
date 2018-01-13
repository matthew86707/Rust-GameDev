#version 330

out vec4 color;
in vec3 fragment_uv;
uniform float time;
uniform samplerCube skybox;

void main() {
    color = texture(skybox, fragment_uv) * vec4(0.5, 0.5, 0.5, 1.0);
}