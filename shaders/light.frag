#version 450 core

out vec4 FragColor;

layout(location=1) in vec3 ambient_material;


void main()
{
    vec3 color = ambient_material;
    FragColor = vec4(color, 1.0);
}


