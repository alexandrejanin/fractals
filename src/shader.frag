#version 330 core
out vec4 pixelColor;
in vec2 coord;

vec3 hsv2rgb(vec3 c) {
  vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
  vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
  return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    float   real  = coord.x;
    float   imag  = coord.y;
    float   Creal = real;
    float   Cimag = imag;

    float r2 = 0.0;

    int iter;

    for (iter = 0; iter < 1000 && r2 < 4.0; ++iter) {
        float tempreal = real;

        real = (tempreal * tempreal) - (imag * imag) + Creal;
        imag = 2.0 * tempreal * imag + Cimag;
        r2   = (real * real) + (imag * imag);
    }

    vec3 color;
    float hue = (iter % 100) / 100.0; 
    
    if (r2 < 4.0)
        color = vec3(0.0f, 0.0f, 0.0f);
    else
        color = hsv2rgb(vec3(hue, 1.0f, 1.0f));

    pixelColor = vec4(color, 1.0f);
}
