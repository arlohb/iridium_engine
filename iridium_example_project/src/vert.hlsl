struct VertexIn {
    float3 position;
    float2 uv_coords;
};

struct VertexOut {
    float4 position : SV_POSITION;
    float2 uv_coords;
};

struct Transform {
    float3 position;
    float3 scale;
    float rotation;
};

struct Camera {
    float2 position;
    float min_depth;
    float max_depth;
    float rotation;
    float scale;
    float aspect_ratio;
};

Transform transform;
ConstantBuffer<Camera> camera : register(b0, space2);

float map(float value, float in_min, float in_max, float out_min, float out_max) {
    return ((value - in_min) / (in_max - in_min)) * (out_max - out_min) + out_min;
}

VertexOut vs_main(VertexIn vertex) {
    // === === === Place the vertex into world space === === ===

    // Scale the vertex
    vertex.position *= transform.scale;
    // Rotate the vertex
    vertex.position = mul(
        float3x3(cos(-transform.rotation), -sin(-transform.rotation), 0.,
                 sin(-transform.rotation),  cos(-transform.rotation), 0.,
                 0.,                        0.,                       1.),
        vertex.position
    );
    // Offset the vertex
    vertex.position += transform.position;


    // === === === Transform from world space to camera space === === ===

    // Offset the world
    vertex.position -= float3(camera.position, 0.);
    // Scale the world
    vertex.position.xy /= camera.scale;
    // Rotate the world
    vertex.position = mul(
        float3x3(cos(-camera.rotation), -sin(-camera.rotation), 0.,
                 sin(-camera.rotation),  cos(-camera.rotation), 0.,
                 0.,                     0.,                    1.),
        vertex.position
    );
    // Fix the aspect ratio
    vertex.position.x /= camera.aspect_ratio;

    // Remap the depth
    vertex.position.z = map(vertex.position.z, camera.min_depth, camera.max_depth, 0., 1.);

    // Usually depth is stored in W, but we're using Z
    return VertexOut(float4(vertex.position, 1.), vertex.uv_coords);
}
