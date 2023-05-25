#version 450 core

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

out vec4 FragColor;

layout(location=0) in vec3 frag_pos;
layout(location=1) in vec3 ambient_material;
layout(location=2) in vec3 diffuse_material;
layout(location=3) in vec3 specular_material;
layout(location=4) in float shininess_material;
layout(location=5) in vec3 normalVector;


uniform layout(location=11) mat4 model_matrix;
uniform layout(location=12) vec3 camera_position;
uniform layout(location=13) Light light;

void main()
{
    mat3 scale_rotate_matrix = mat3(model_matrix);

    vec3 actual_normal = normalize(normalVector * scale_rotate_matrix);
    vec3 light_direction = normalize(light.position - frag_pos);

    //Ambient component
    vec3 ambient = ambient_material * light.ambient;

    //Diffuse component
    vec3 diffuse = (max(0, dot(actual_normal, light_direction)) * diffuse_material) * light.diffuse;

    //Specular component
    vec3 camera_direction = normalize(camera_position - frag_pos);
    vec3 reflection_direction = reflect(-light_direction, actual_normal);
    float spec = pow(max(dot(camera_direction, reflection_direction), 0.0), shininess_material);
    vec3 specular = (specular_material * spec) * light.specular;


    vec3 color =  (ambient + diffuse + specular);
    FragColor = vec4(color, 1.0);
}






