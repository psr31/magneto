#version 330 core
out vec4 FragColor;

in vec2 pTexc;

uniform sampler2D uTex;
uniform vec3 color;

void main()
{
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(uTex, pTexc).r);
    FragColor = vec4(color, 1.0) * sampled;
}