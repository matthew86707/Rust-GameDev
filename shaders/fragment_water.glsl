#version 330

in vec2 fragment_uv;
in vec4 positionCoord;

out vec4 color;

uniform sampler2D sampler;
uniform vec3 light_position;
uniform float time;

void main() {
	vec3 lightVector = normalize(light_position - positionCoord.xyz);
    vec3 newNormal = normalize(positionCoord).xyz;

    float diffuse = max(dot(newNormal, lightVector), 0.1);

    color = vec4(1.0, 1.0, 1.0, 0.5) * mix(vec4(0.0, 0.0, 1.0, 0.2), texture(sampler, fragment_uv * 150), 0.5) * vec4(diffuse, diffuse, diffuse, 1.0);
    //color = vec4(1.0, 1.0, 1.0, 1.0);
}