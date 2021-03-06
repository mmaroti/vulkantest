#version 450

layout(local_size_x = 8, local_size_y = 1, local_size_z = 1) in;

layout(constant_id = 0) const uint OFFSET = 100;

layout(set = 0, binding = 0) restrict writeonly buffer Data {
        uint data[];
};

void main() {
    uint i = gl_GlobalInvocationID.x;
    data[i] = i + OFFSET;
}
