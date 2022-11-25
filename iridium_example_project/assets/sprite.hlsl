struct VertexOut {
    float4 position : SV_POSITION;
    float2 uv_coords;
};

Texture2D steak_tex : register(t0, space1);
SamplerState steak_sam : register(s1, space1);

float4 fs_main(VertexOut vertex) : SV_TARGET {
    return steak_tex.Sample(steak_sam, vertex.uv_coords);
}
