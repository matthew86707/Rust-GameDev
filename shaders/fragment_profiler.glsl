#version 140

in vec2 fragment_uv;
out vec4 color;

uniform float time_passed;
uniform float mouseX;
uniform float mouseY;
uniform float zoom_uniform;

void main() {

    float isShaded = 0.0;
    float zoom = zoom_uniform;
    float x = fragment_uv.x;
    float y = fragment_uv.y;
    //isShaded = 1.0 - step(pow(time_passed, 2), (x*x) + (y*y));

    vec2 z = vec2(0.0, 0.0);
    vec2 c = vec2(x, y);
    float iterations = 0;
    float MAX_ITERATIONS = time_passed * 3.0;

    while(length(z) <= 2.0 && iterations <= MAX_ITERATIONS){
        iterations++;
        vec2 newZ = vec2((pow(z.x, 2) - pow(z.y, 2) + c.x), 2.0 * z.x * z.y + c.y);
        z = newZ; 
    }

    isShaded = iterations / MAX_ITERATIONS;

    color = vec4(0.0, 0.0, isShaded, 1.0);
}