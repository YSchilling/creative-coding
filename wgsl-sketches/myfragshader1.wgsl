struct VertexOutput {
    @location(0) coord: vec2<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    

    
    return vec4<f32>(1.0,0., 0., 1.0);
}