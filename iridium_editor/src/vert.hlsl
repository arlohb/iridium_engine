// float3 position;

float4 vs_main(float3 vertex_position : POSITION) : SV_POSITION {
  return float4(vertex_position, 1.0);
}
