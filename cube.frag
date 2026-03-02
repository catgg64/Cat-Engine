#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D tex;

void main()
{
    FragColor = texture(tex, TexCoord);
    // optional: slight tint for visibility during debug
    FragColor *= vec4(1.2, 1.2, 1.2, 1.0);
}