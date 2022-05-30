float4 vs_main(float3 position : POSITION) : SV_POSITION {
  return float4(position, 1.0);
}
