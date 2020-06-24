#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTex;

uniform mat4 model;
uniform mat4 proj;

out vec2 pTex;

void main()
{
    gl_Position = proj * model * vec4(aPos, 0.0, 1.0);
    pTex = aTex;
}
