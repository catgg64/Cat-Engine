#version 330 core
out vec4 FragColor;
in vec2 TexCoord;          // ← must have this incoming from vertex shader

uniform sampler2D tex;     // name must match what you set with set_int("tex", 0)

void main()
{
    FragColor = texture(tex, TexCoord);
    // For quick UV debug: FragColor = vec4(TexCoord, 0.0, 1.0);
    // For quick texture debug: FragColor = vec4(1.0, 0.0, 1.0, 1.0); // magenta if texture fails
}