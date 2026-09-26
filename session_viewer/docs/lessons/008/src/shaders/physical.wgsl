// `@location(n)` on an output = the n-th colour target in the pipeline's list.
// Output of a face fragment: color and triangle id.
struct PhysicalColor {
    @location(0) color: vec4<f32>, // rgba
}
