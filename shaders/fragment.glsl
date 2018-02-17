#version 330

in vec2 fragment_uv;
in float shading_i;
out vec4 color;
uniform float glowEffect;
uniform vec3 glowPosition;
uniform sampler2D sampler;
uniform sampler2D rockSampler;
uniform sampler2D snowSampler;

uniform float time;
uniform float value;
uniform vec2 screen_size;


in vec4 positionCoord;
in vec3 norm;

void main() {
    float glowEffectNew = clamp(glowEffect * (5.0 / distance(vec4(glowPosition, 0.0), positionCoord)), 0.5, 5.0);
    // Get a lighting direction vector from the light to the vertex.
    vec3 lightVector = normalize(vec3(35.0, 0.0, 500.0) - positionCoord.xyz);
 
    vec3 newNormal = normalize(positionCoord).xyz;

    // Calculate the dot product of the light vector and vertex normal. If the normal and light vector are
    // pointing in the same direction then it will get max illumination.
    float diffuse = max(dot(newNormal, lightVector), 0.1);
    //float diffuse = clamp(max(dot(newNormal, lightVector), 0.1) / 4, 0.3, 1.2); 

    float dist = distance(positionCoord, vec4(0.0, 0.0, 0.0, 0.0));

	vec4 snowRockMixedTex = mix(texture(rockSampler, fragment_uv), texture(snowSampler, fragment_uv), 0.0);
	vec4 mixedTex = mix(texture(sampler, fragment_uv), snowRockMixedTex, clamp((dist - 29), 0.0, 1.0));
	//if(shading_i == 0.0){
	//	color =  mixedTex * vec4(glowEffectNew, glowEffectNew, glowEffectNew, 1.0) * vec4(0.5, 0.5, 0.5, 1.0);
	//}else{
    	color = vec4(diffuse, diffuse, diffuse, 1.0) * mixedTex * vec4(glowEffectNew, glowEffectNew, glowEffectNew, 1.0) * vec4(0.5, 0.5, 0.5, 1.0);
    //}
}