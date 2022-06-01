struct Transform {
  float3 position;
  float3 scale;
};

Transform transform;

float4 vs_main(float3 vertex_position : POSITION) : SV_POSITION {
  float3 scaled = vertex_position * transform.scale;
  float3 translated = scaled + transform.position;

  return float4(translated, 1.0);
}
