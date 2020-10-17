#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec3 Color;

uniform mat4 proj_mat_location;
uniform mat4 model_mat_location;

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main()
{
    gl_Position = proj_mat_location * model_mat_location * vec4(Position, 0.0, 1.0);
    OUT.Color = vec4(Color, 1.0);
}