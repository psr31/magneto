#version 330 core
out vec4 FragColor;

in vec2 pTex;

uniform sampler2D uTex;

void main()
{
        FragColor = texture(uTex, pTex);
}
