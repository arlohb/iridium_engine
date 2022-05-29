float4 fs_main(float4 position : SV_POSITION) : SV_TARGET {
  return float4(position.x, position.y, 0.0, 1.0);
}
