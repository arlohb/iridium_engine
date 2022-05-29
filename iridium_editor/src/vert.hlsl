float4 vs_main(uint input : SV_VERTEXID) : SV_POSITION {
  float x = (float)(1 - (int)input) * 0.5;
  float y = (float)((int)(input & 1) * 2 - 1) * 0.5;
  return float4(x, y, 0., 1.);
}
