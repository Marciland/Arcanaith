#version 450
#define MAX_INSTANCES 10 // TODO

struct ModelViewProjection {
  mat4 model;
  mat4 view;
  mat4 projection;
};

layout(set = 0, binding = 0) uniform MVPs { ModelViewProjection mvpMatrices[MAX_INSTANCES]; };
layout(set = 0, binding = 1) uniform InstanceCount { int numInstances; };

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec2 inTextureCoordinates;

layout(location = 0) out vec2 fragTextureCoordinates;
layout(location = 1) out int instanceIndex;

void main() {
  if (gl_InstanceIndex < numInstances) {
    gl_Position = mvpMatrices[gl_InstanceIndex].projection *
                  mvpMatrices[gl_InstanceIndex].view *
                  mvpMatrices[gl_InstanceIndex].model *
                  vec4(inPosition, 0.0, 1.0);
    fragTextureCoordinates = inTextureCoordinates;
    instanceIndex = gl_InstanceIndex;
  }
}