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

Transform transform;

VertexOut vs_main(VertexIn vertex) {
  float3x3 rotation = float3x3(cos(-transform.rotation), -sin(-transform.rotation), 0.,
                               sin(-transform.rotation),  cos(-transform.rotation), 0.,
                               0.,                       0.,                      1.);

  vertex.position *= transform.scale;
  vertex.position = mul(rotation, vertex.position);
  vertex.position += transform.position;

  return VertexOut(float4(vertex.position, 1.), vertex.uv_coords);
}
