#version 330 core

layout (location = 0) in vec2 position;

out vec2 TexCoord;

void main()
{
    TexCoord = position * 0.5 + 0.5;
    gl_Position = vec4(position, 1.0, 1.0);
}