#version 140

out vec4 color;
in vec3 ReflectDir;
uniform samplerCube cubetex;

void main() {
    color = texture(cubetex, ReflectDir);
}