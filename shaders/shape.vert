#version 450 core

struct Noise {
    float strength;
    float base_roughness;
    float roughness;
    float persistence;
    vec3 center;
    int num_layers;
    float min_value;
};

layout(location=0) in vec3 position;
layout(location=0) out vec3 frag_pos_out;

layout(location=1) in vec3 ambient_material;
layout(location=1) out vec3 ambient_material_out;

layout(location=2) in vec3 diffuse_material;
layout(location=2) out vec3 diffuse_material_out;

layout(location=3) in vec3 specular_material;
layout(location=3) out vec3 specular_material_out;

layout(location=4) in float shininess;
layout(location=4) out float shininess_out;

layout(location=5) in vec3 normalVector;
layout(location=5) out vec3 normal_vector_out;


uniform layout(location=10) mat4 transform_matrix;
uniform layout(location=11) mat4 model_matrix;


void main()
{
   

    const vec4 transformed_pos = vec4(position, 1) * transform_matrix;
    gl_Position = transformed_pos;
    
    frag_pos_out = vec3(vec4(position, 1) * model_matrix);

    normal_vector_out = normalize(normalVector);
    ambient_material_out = ambient_material;
    diffuse_material_out = diffuse_material;
    specular_material_out = specular_material;
    shininess_out = shininess;
}