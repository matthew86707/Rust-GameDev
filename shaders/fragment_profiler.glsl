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

    vec2 z = vec2(x, y);
    vec2 c = vec2(cos(time_passed * 5.0), sin(time_passed));
    float iterations = 0;
    float MAX_ITERATIONS = 200.0;

    while(length(z) <= 2.0 && iterations <= MAX_ITERATIONS){
        vec2 newZ = vec2((pow(z.x, 2) - pow(z.y, 2) + c.x), 2.0 * z.x *  z.y + c.y);
        z = newZ; 
        iterations++;
    }

    if(iterations < MAX_ITERATIONS){
        float log_zn = log( x*x + y*y + 1.0) / 2.0;
        float nu = log( log_zn / log(2.0) ) / log(2.0);
        iterations = iterations + 1 - nu;
    }

    isShaded = iterations / MAX_ITERATIONS;
    //isShaded = ((isShaded + 10.0) / 10.0);
    color = vec4(0.0, isShaded, fract(isShaded + time_passed), 1.0);
}