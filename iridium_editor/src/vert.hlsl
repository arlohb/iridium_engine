struct Transform {
  float3 position;
  float3 scale;
  float rotation;
};

Transform transform;

float4 vs_main(float3 position : POSITION) : SV_POSITION {
  float3x3 rotation = float3x3(cos(transform.rotation), -sin(transform.rotation), 0.,
                               sin(transform.rotation),  cos(transform.rotation), 0.,
                               0.,                       0.,                      1.);

  position = mul(rotation, position);
  position *= transform.scale;
  position += transform.position;

  return float4(position, 1.0);
}
