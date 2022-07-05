struct VertexOut {
    float4 position : SV_POSITION;
    float2 uv_coords;
};

float4 fs_main(VertexOut vertex) : SV_TARGET {
    return float4(float3(vertex.uv_coords, 0.), 0.5);
}
