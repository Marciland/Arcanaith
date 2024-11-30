#version 450

struct MVP {
  mat4 model;
  mat4 view;
  mat4 projection;
};

layout(set = 0, binding = 0) readonly buffer MVPs {
  MVP mvpMatrices[];
};

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec2 inTextureCoordinates;

layout(location = 0) out vec2 fragTextureCoordinates;
layout(location = 1) flat out int instanceIndex;

void main() {
  gl_Position = mvpMatrices[gl_InstanceIndex].projection *
                mvpMatrices[gl_InstanceIndex].view *
                mvpMatrices[gl_InstanceIndex].model *
                vec4(inPosition, 0.0, 1.0);

  fragTextureCoordinates = inTextureCoordinates;
  instanceIndex = gl_InstanceIndex;
}