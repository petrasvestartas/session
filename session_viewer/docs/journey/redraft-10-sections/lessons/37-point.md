# 5.3 · Original point markers and curve control identities

**Steps:** 7 · **Source:** [app/walk/curves.rs](../../../../src/app/walk/curves.rs) · [engine/gpu/segments.rs](../../../../src/engine/gpu/segments.rs) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl)

## <a id="checkpoint-37-point"></a>1. Prepare an point for a screen marker

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-2e93db36" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-2e93db36" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-2e93db36" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-2e93db36" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-2e93db36" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">37-POINT · SDF MARKER DISC</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Screen-Aligned Point Quad Expansion (±r_px) &amp; Analytic @fragment SDF Circle + Border Ring</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">d = length(uv) * r_px · ring: r_in &lt; d &lt;= r_out</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-2e93db36)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · 6-Vertex Screen Quad Around 3D Point P</text>
  <rect x="122" y="72" width="128" height="112" fill="#f8fafc" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <circle cx="186" cy="128" r="44" fill="#fef08a" stroke="#dc2626" stroke-width="3.5"/>
  <circle cx="186" cy="128" r="3.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="122" y="66" font-family="monospace" font-size="8.5" fill="#111111">uv=(-1,+1)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="218" y="196" font-family="monospace" font-size="8.5" fill="#111111">uv=(+1,-1)</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-2e93db36)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-2e93db36)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Radial SDF Fill + Dark Contrast Outline</text>
  <rect x="414" y="76" width="292" height="104" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">let r = length(in.uv); // r ∈ [0, √2]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="122" text-anchor="middle" font-family="monospace" font-size="9" fill="#dc2626">if r &gt; 1.0 { discard; }</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">r &lt; r_inner =&gt; interior fill color</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="166" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">r ∈ [r_inner, 1.0] =&gt; dark border ring</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-2e93db36)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-2e93db36)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Depth-Tested CAD Point Markers</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <circle cx="876" cy="138" r="10" fill="#60a5fa" stroke="#111111" stroke-width="2.5"/>
  <circle cx="934" cy="96" r="10" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
  <circle cx="994" cy="144" r="10" fill="#4ade80" stroke="#111111" stroke-width="2.5"/>
</svg>
</div>

### 1. `src/marker.rs`

Create the file and type:

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Marker {
    pub center: [f32; 3],
    pub diameter: f32,
    pub colour: [f32; 4],
}

#[derive(Clone)]
pub struct PreparedPoint {
    pub source: Rc<session_rust::Point>,
    pub display: Marker,
}

impl PreparedPoint {
    pub fn new(source: Rc<session_rust::Point>) -> Result<Self, &'static str> {
        let _ = source.guid();
        let diameter = if source.width.is_finite()
            && source.width > 0.0
            && (source.width - 1.0).abs() > 1e-9
        {
            source.width as f32
        } else {
            6.0
        };
        let diameter = if diameter == 0.0 {
            6.0
        } else {
            diameter
        };
        let display = Marker {
            center: source.to_f32(),
            diameter,
            colour: source.pointcolor.to_f32(),
        };
        if display
            .values()
            .iter()
            .any(|v| !v.is_finite())
        {
            return Err("Point marker exceeds the display range");
        }
        Ok(Self { source, display })
    }
}

impl Marker {
    pub fn values(&self) -> [f32; 8] {
        let [x, y, z] = self.center;
        let [r, g, b, a] = self.colour;
        [x, y, z, self.diameter, r, g, b, a]
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.values()
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect()
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod load_phase_tests;
pub mod memory;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod load_phase_tests;
pub mod marker;
#[cfg(test)]
mod marker_tests;
pub mod memory;
```

### 3. `src/marker_tests.rs`

Create the file and type:

```rust
use crate::marker::PreparedPoint;
use session_rust::{Color, Point};
use std::rc::Rc;

#[test]
fn point_preparation_preserves_original_coordinates_style_and_shared_owner() {
    let mut point = Point::new(0.123456789012345, 0.0, 0.0);
    point.width = 9.0;
    point.pointcolor = Color::new(0.2, 0.4, 0.6, 0.5);
    let source = Rc::new(point);
    let weak = Rc::downgrade(&source);
    let guid = source.guid().to_owned();
    let prepared = PreparedPoint::new(Rc::clone(&source)).unwrap();
    assert_eq!(prepared.source[0], 0.123456789012345);
    assert_eq!(prepared.source.guid(), guid);
    assert_ne!(prepared.display.center[0] as f64, source[0]);
    assert_eq!(prepared.display.diameter, 9.0);
    let bytes = prepared.display.bytes();
    assert_eq!(bytes.len(), 32);
    assert_eq!(
        f32::from_ne_bytes(bytes[12..16].try_into().unwrap()),
        9.0
    );
    let shared = prepared.clone();
    assert!(Rc::ptr_eq(&prepared.source, &shared.source));
    drop(source);
    drop(prepared);
    assert!(weak.upgrade().is_some());
    drop(shared);
    assert!(weak.upgrade().is_none());
}

#[test]
fn marker_default_width_and_display_refusal_match_their_source_policy() {
    for width in [
        0.0,
        1.0,
        -1.0,
        f64::NAN,
        f64::INFINITY,
        1e-100,
    ] {
        let mut point = Point::new(0.0, 0.0, 0.0);
        point.width = width;
        assert_eq!(
            PreparedPoint::new(Rc::new(point))
                .unwrap()
                .display
                .diameter,
            6.0
        );
    }
    for x in [f64::NAN, f64::INFINITY, 1e100] {
        assert!(PreparedPoint::new(Rc::new(Point::new(x, 0.0, 0.0))).is_err());
    }
    let mut point = Point::new(0.0, 0.0, 0.0);
    point.width = 1e100;
    assert!(PreparedPoint::new(Rc::new(point)).is_err());
}
```

## <a id="checkpoint-37a-disc"></a>2. Extrude a circular marker in screen pixels

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Profile Curve C(u) + Extrude Vector d / Revolve Axis A</text>
  <path d="M 64 138 Q 114 82 164 138" fill="none" stroke="#dc2626" stroke-width="3"/>
  <line x1="114" y1="110" x2="114" y2="48" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="234" y1="46" x2="234" y2="156" stroke="#2563eb" stroke-width="2" stroke-dasharray="5 3"/>
  <path d="M 254 136 Q 282 100 254 64" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">S_ext(u,v) = C(u) + v*d  |  S_rev(u,v) = Rot_A(v) * C(u)</text>
  <polygon points="426,138 476,92 546,92 496,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="426,96 476,50 546,50 496,96" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="426" y1="138" x2="426" y2="96" stroke="#16a34a" stroke-width="2"/>
  <ellipse cx="638" cy="72" rx="42" ry="14" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <ellipse cx="638" cy="132" rx="42" ry="14" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="596" y1="72" x2="596" y2="132" stroke="#111111" stroke-width="1.75"/>
  <line x1="680" y1="72" x2="680" y2="132" stroke="#111111" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Extruded Prism &amp; Revolved Surface of Revolution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="832,138 866,76 924,76 890,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <ellipse cx="992" cy="74" rx="36" ry="12" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <path d="M 956 74 L 956 134 A 36 12 0 0 0 1028 134 L 1028 74" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

### 1. `src/marker.wgsl`

Create the file and type:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
}
@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {
    let coverage = clamp(input.radius + 0.5 - length(input.offset), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

### 2. `src/marker.rs`

**Locate this existing block:**

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const SHADER: &str = include_str!("marker.wgsl");
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
```

## <a id="checkpoint-37b-lane"></a>3. Draw markers with the shared instance lane

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D Point Center P -&gt; Screen Quad Corner uv ∈ [-1, 1]²</text>
  <rect x="114" y="48" width="112" height="108" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <circle cx="170" cy="102" r="5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="112" y="42" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(-1,+1)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="228" y="164" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(+1,-1)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Fragment SDF: r = length(uv); smoothstep(1 - w, 1, r)</text>
  <rect x="498" y="46" width="124" height="112" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
  <circle cx="560" cy="102" r="46" fill="#fef08a" stroke="#111111" stroke-width="3"/>
  <circle cx="560" cy="102" r="32" fill="#dc2626"/>
  <line x1="560" y1="102" x2="606" y2="102" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="583" y="94" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">r=1</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Anti-Aliased Control Points in Viewport</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <circle cx="866" cy="122" r="9" fill="#dc2626" stroke="#111111" stroke-width="2"/>
  <circle cx="934" cy="74" r="9" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
  <circle cx="1002" cy="118" r="9" fill="#2563eb" stroke="#111111" stroke-width="2"/>
</svg>
</div>

### 1. `src/stroke_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32,
                4 => Float32x3, 5 => Float32x3, 6 => Uint32],
        )
```

Replace it with:

```rust
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32,
                4 => Float32x3, 5 => Float32x3, 6 => Uint32],
        )
    }

    pub fn markers(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(
            device,
            format,
            crate::marker::SHADER,
            32,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32x4],
        )
```

**Locate this block:**

```rust
    }

    fn set_bytes(
        &mut self,
        device: &wgpu::Device,
```

Replace it with:

```rust
    }

    pub fn set_points(
        &mut self,
        device: &wgpu::Device,
        markers: impl IntoIterator<Item = crate::marker::Marker>,
    ) -> Result<(), &'static str> {
        if self.stride != 32 {
            return Err("Marker data needs the marker layout");
        }
        let mut data = Vec::new();
        for marker in markers {
            if marker.diameter <= 0.0
                || marker
                    .values()
                    .iter()
                    .any(|v| !v.is_finite())
            {
                return Err("Invalid marker display data");
            }
            data.extend(marker.bytes());
        }
        self.set_bytes(device, data)
    }

    fn set_bytes(
        &mut self,
        device: &wgpu::Device,
```

## <a id="checkpoint-37c-document"></a>4. Retain point rows and placement

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn clear(&mut self) {
        self.paths = Vec::new();
```

Replace it with:

```rust
    }

    pub fn clear(&mut self) {
        self.points = Vec::new();
        self.paths = Vec::new();
```

**Locate this block:**

```rust
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn paths(&self) -> &[PathObject] {
        &self.paths
    }
```

Replace it with:

```rust
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn points(&self) -> &[PointObject] {
        &self.points
    }

    pub fn point_capacity_bytes(&self) -> usize {
        self.points.capacity() * std::mem::size_of::<PointObject>()
    }

    pub fn insert_point(
        &mut self,
        prepared: crate::marker::PreparedPoint,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self
            .points
            .iter()
            .any(|row| row.guid == guid)
            || self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self
                .lines
                .iter()
                .any(|row| row.prepared.source.guid() == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.points.push(PointObject {
            id,
            guid,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_point(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid point placement");
        }
        let row = self
            .points
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) {
            return Err("Point placement exceeds the display range");
        }
        row.model = model;
        Ok(())
    }

    pub fn paths(&self) -> &[PathObject] {
        &self.paths
    }
```

**Locate this block:**

```rust
                }
            }
        }
        bounds
    }
```

Replace it with:

```rust
                }
            }
        }
        for row in &self.points {
            let p = row.point();
            let p = [p[0], p[1], p[2]];
            match &mut bounds {
                Some(bounds) => bounds.include(p),
                None => bounds = Some(crate::bounds::Bounds::point(p)),
            }
        }
        bounds
    }
```

**Locate this block:**

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            paths: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
```

Replace it with:

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
```

**Locate this block:**

```rust
#[derive(Clone)]
pub struct Scene {
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
```

Replace it with:

```rust
#[derive(Clone)]
pub struct PointObject {
    pub id: ObjectId,
    pub guid: String,
    pub prepared: crate::marker::PreparedPoint,
    pub model: session_rust::Xform,
}

impl PointObject {
    pub fn point(&self) -> session_rust::Point {
        self.model
            .transform_point(&self.prepared.source)
    }
    pub fn marker(&self) -> crate::marker::Marker {
        crate::marker::Marker {
            center: self.point().to_f32(),
            ..self.prepared.display
        }
    }
}

#[derive(Clone)]
pub struct Scene {
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
```

### 2. `src/document.rs`

**Locate this existing block:**

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if !scene.lines().is_empty() || !scene.paths().is_empty() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if !scene.points().is_empty() {
        return Err("This checkpoint cannot save point objects yet");
    }
    if !scene.lines().is_empty() || !scene.paths().is_empty() {
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod placement;
pub mod prepared;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod placement;
#[cfg(test)]
mod point_document_tests;
pub mod prepared;
```

### 4. `src/point_document_tests.rs`

Create the file and type:

```rust
use crate::{history::History, marker::PreparedPoint, scene::Scene};
use session_rust::{Point, Xform};
use std::rc::Rc;

#[test]
fn original_point_rows_keep_precise_placement_shared_history_and_release() {
    let mut scene = Scene::demo();
    let mut history = History::default();
    let source = Rc::new(Point::new(2.123456789, 0.0, 0.0));
    let weak = Rc::downgrade(&source);
    let prepared = PreparedPoint::new(Rc::clone(&source)).unwrap();
    let first = scene.insert_point(prepared.clone()).unwrap();
    let second = scene.insert_point(prepared).unwrap();
    assert_ne!(first, second);
    assert_ne!(
        scene.points()[0].guid,
        scene.points()[1].guid
    );
    history
        .try_edit(&mut scene, |scene| {
            scene.place_point(first, Xform::translation(10.0, 0.0, 0.0))
        })
        .unwrap();
    assert!((scene.points()[0].point()[0] - 12.123456789).abs() < 1e-12);
    assert_eq!(source[0], 2.123456789);
    assert_eq!(scene.bounds().unwrap().max[0], 12.123456789);
    let model = scene.points()[0].model.clone();
    let mut invalid = Xform::identity();
    invalid.m[0] = f64::NAN;
    assert!(scene.place_point(first, invalid).is_err());
    assert_eq!(scene.points()[0].model.m, model.m);
    assert!(history.undo(&mut scene));
    assert_eq!(scene.points()[0].point()[0], source[0]);
    assert!(history.redo(&mut scene));
    assert_eq!(scene.points()[0].id, first);
    assert!(Rc::ptr_eq(
        &scene.points()[0].prepared.source,
        &history.scenes().next().unwrap().points()[0]
            .prepared
            .source
    ));
    assert!(crate::document::snapshot(&scene).is_err());
    drop(source);
    scene.clear();
    assert!(weak.upgrade().is_some());
    history.clear();
    assert!(weak.upgrade().is_none());
    assert!(scene.points().is_empty());
    assert!(scene.bounds().is_none());
}
```

## <a id="checkpoint-37d-render"></a>5. Draw point rows and count their resources

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
        }
```

Replace it with:

```rust
            }
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
        }
```

**Locate this block:**

```rust
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

Replace it with:

```rust
            self.size,
            self.density,
        );
        self.markers.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

**Locate this block:**

```rust
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
    }

    pub fn stats(&self) -> [usize; 4] {
```

Replace it with:

```rust
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
            + self.markers.uploaded_bytes
    }

    pub fn stats(&self) -> [usize; 4] {
```

**Locate this block:**

```rust
        ]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.paths
            .set_chains(
                &self.device,
```

Replace it with:

```rust
        ]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [
            count,
            bytes,
            self.markers.uploads,
            self.markers.uploaded_bytes,
        ]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .map(crate::scene::PointObject::marker),
            )
            .expect("Prepared point placement fits the display range");
        self.paths
            .set_chains(
                &self.device,
```

**Locate this block:**

```rust
            depth,
            strokes,
            paths,
            size: [640, 480],
            density: 1.0,
        };
```

Replace it with:

```rust
            depth,
            strokes,
            paths,
            markers,
            size: [640, 480],
            density: 1.0,
        };
```

**Locate this block:**

```rust
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let mut renderer = Self {
            device,
            queue,
```

Replace it with:

```rust
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let mut renderer = Self {
            device,
            queue,
```

**Locate this block:**

```rust
    paths: crate::stroke_gpu::Lane,
    size: [u32; 2],
    density: f32,
}
```

Replace it with:

```rust
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    size: [u32; 2],
    density: f32,
}
```

### 2. `src/memory.rs`

**Locate this existing block:**

```rust
}

pub fn paths<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 5] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn points<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 3] {
    let mut sources = HashSet::new();
    let mut usage = [0; 3];
    for scene in scenes {
        usage[0] += scene.points().len();
        usage[2] += scene.point_capacity_bytes();
        for row in scene.points() {
            sources.insert(Rc::as_ptr(&row.prepared.source));
        }
    }
    usage[1] = sources.len();
    usage
}

pub fn paths<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 5] {
```

### 3. `src/browser.rs`

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-path-gpu",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-point-gpu",
        &serde_json::to_string(&renderer.point_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-point-cpu",
        &serde_json::to_string(&crate::memory::points(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-path-gpu",
```

## <a id="checkpoint-37da-input"></a>6. Insert points through the real dock

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
```

Replace it with:

```rust
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0;
                point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared =
                    crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_point(prepared).map(|_| ())
                    })?;
```

**Locate this block:**

```rust
    AddLine,
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
```

Replace it with:

```rust
    AddLine,
    AddPoint,
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    let action = match line.as_str() {
                        "example box" => Action::AddBox,
                        "example line" => Action::AddLine,
                        "example polyline" => Action::AddPolyline,
```

Replace it with:

```rust
                    let action = match line.as_str() {
                        "example box" => Action::AddBox,
                        "example line" => Action::AddLine,
                        "example point" => Action::AddPoint,
                        "example polyline" => Action::AddPolyline,
```

**Locate this block:**

```rust
            "Example Line",
            "Example Polyline",
            "Example Arrow",
            "Example Triangle",
```

Replace it with:

```rust
            "Example Line",
            "Example Point",
            "Example Polyline",
            "Example Arrow",
            "Example Triangle",
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
mod point_document_tests;
pub mod prepared;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod point_document_tests;
#[cfg(test)]
mod point_input_tests;
pub mod prepared;
```

### 4. `src/point_input_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    memory,
};

#[test]
fn point_command_preserves_source_identity_in_one_history_step() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddPoint).unwrap();
    let id = editor.scene.points()[0].id;
    let row = &editor.scene.points()[0];
    let weak = std::rc::Rc::downgrade(&row.prepared.source);
    assert_eq!(row.prepared.source.width, 12.0);
    assert_eq!(
        row.prepared.source.pointcolor.to_f32(),
        [0.0, 0.0, 0.0, 0.5]
    );
    assert_eq!(row.guid, row.prepared.source.guid());
    editor.apply(Action::Zoom(2.0)).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.points().is_empty());
    assert!(weak.upgrade().is_some());
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.points()[0].id, id);
    assert!(
        crate::document::snapshot(&editor.scene)
            .unwrap_err()
            .contains("cannot save point objects yet")
    );
    editor.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    assert_eq!(memory::points(editor.scenes()), [0; 3]);
}
```

## <a id="checkpoint-37e-controls"></a>7. Identify controls on the curve owner

### 1. `src/controls.rs`

Create the file and type:

```rust
use crate::marker::Marker;
use session_rust::NurbsCurve;
use std::rc::Rc;

#[derive(Clone)]
pub struct Control {
    pub source: Rc<NurbsCurve>,
    pub index: usize,
}

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index };
        control.position()?;
        Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        let c = &self.source;
        if !(2..=3).contains(&c.m_dim)
            || !(1..=10_000).contains(&c.m_cv_count)
            || self.index >= c.m_cv_count
        {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size {
            return Err("Control stride is too short");
        }
        let start = self
            .index
            .checked_mul(c.m_cv_stride)
            .ok_or("Control offset overflows")?;
        let end = start
            .checked_add(size)
            .ok_or("Control range overflows")?;
        let values = c
            .m_cv
            .get(start..end)
            .ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat {
            values[c.m_dim]
        } else {
            1.0
        };
        if !weight.is_finite() || weight == 0.0 {
            return Err("Control weight must be finite and nonzero");
        }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim {
            point[axis] = values[axis] / weight;
        }
        if point.iter().any(|v| !v.is_finite()) {
            return Err("Control position must be finite");
        }
        Ok(point)
    }

    pub fn marker(&self) -> Result<Marker, &'static str> {
        let marker = Marker {
            center: self.position()?.map(|v| v as f32),
            diameter: 6.0,
            colour: [0.0, 0.0, 0.0, 1.0],
        };
        if marker
            .values()
            .iter()
            .any(|v| !v.is_finite())
        {
            return Err("Control marker exceeds the display range");
        }
        Ok(marker)
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod close_report_tests;
pub mod diagnostic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod close_report_tests;
#[cfg(test)]
mod control_tests;
pub mod controls;
pub mod diagnostic;
```

### 3. `src/control_tests.rs`

Create the file and type:

```rust
use crate::controls::Control;
use session_rust::NurbsCurve;
use std::rc::Rc;

#[test]
fn rational_controls_share_original_identity_and_keep_homogeneous_precision() {
    let mut curve = NurbsCurve::new(3, true, 2, 2);
    curve.m_cv = vec![
        0.24691357802469,
        0.8,
        0.0,
        2.0,
        2.0,
        0.0,
        0.0,
        1.0,
    ];
    let source = Rc::new(curve);
    let guid = source.guid().to_owned();
    let weak = Rc::downgrade(&source);
    let control = Control::new(Rc::clone(&source), 0).unwrap();
    let other = Control::new(Rc::clone(&source), 1).unwrap();
    assert!(Rc::ptr_eq(&control.source, &other.source));
    assert_eq!(control.index, 0);
    assert_eq!(other.index, 1);
    assert_eq!(
        control.position().unwrap(),
        [0.123456789012345, 0.4, 0.0]
    );
    assert_eq!(control.source.m_cv[0], 0.24691357802469);
    assert_eq!(control.source.guid(), guid);
    assert_ne!(
        control.marker().unwrap().center[0] as f64,
        control.position().unwrap()[0]
    );
    drop(source);
    drop(control);
    assert!(weak.upgrade().is_some());
    drop(other);
    assert!(weak.upgrade().is_none());
}

#[test]
fn control_layout_checks_refuse_malformed_ranges_without_panicking() {
    for mutate in [0, 1, 2, 3, 4, 5, 6] {
        let mut curve = NurbsCurve::new(3, true, 2, 2);
        curve.m_cv = vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0];
        match mutate {
            0 => curve.m_dim = 4,
            1 => curve.m_cv_stride = 2,
            2 => curve.m_cv.clear(),
            3 => curve.m_cv_stride = usize::MAX,
            4 => curve.m_cv[7] = 0.0,
            5 => curve.m_cv[7] = f64::NAN,
            _ => curve.m_cv[4] = f64::INFINITY,
        }
        assert!(Control::new(Rc::new(curve), 1).is_err());
    }
    let source = Rc::new(NurbsCurve::new(3, false, 2, 2));
    assert!(Control::new(source, 2).is_err());
    let mut curve = NurbsCurve::new(2, false, 2, 2);
    curve.m_cv = vec![1.0, 2.0, 3.0, 4.0];
    assert_eq!(
        Control::new(Rc::new(curve), 1)
            .unwrap()
            .position()
            .unwrap(),
        [3.0, 4.0, 0.0]
    );
    let mut curve = NurbsCurve::new(3, false, 2, 2);
    curve.m_cv[0] = 1e100;
    let control = Control::new(Rc::new(curve), 0).unwrap();
    assert!(control.marker().is_err());
}
```

## Build & Run

![Recorded visual output for Lesson 5.3 (37e-controls)](../../../screenshots/journey/37e-controls-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 5.3
npm --prefix ../session_tests run course -- save 5.3
```

Continue with [5.4 · NURBS source ownership and derived samples](../../consolidation/38-layout.md).
