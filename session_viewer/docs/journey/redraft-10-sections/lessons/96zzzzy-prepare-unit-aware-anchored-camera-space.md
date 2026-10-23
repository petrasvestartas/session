# 21.1 · Units and one precise render origin

**Goal:** Check that measurement, projection and original history use the shared camera unit. · **Steps:** 11 · **Source:** [camera.rs](../../../../src/camera.rs) · [state.rs](../../../../src/state.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs) · [engine/gpu/targets.rs](../../../../src/engine/gpu/targets.rs) · [shaders/grid.wgsl](../../../../src/shaders/grid.wgsl) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [app/cplane.rs](../../../../src/app/cplane.rs) (`session_viewer/src/lib.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 21.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/measure_mark.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">11 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Prepare unit-aware anchor…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Two-Pass CommandEncoder</text>
  <rect x="594" y="48" width="212" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Pass 1: LoadOp::Clear</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="80" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Clear Canvas + Pink Triangle</text>
  <line x1="700" y1="88" x2="700" y2="102" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="594" y="104" width="212" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="121" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Pass 2: LoadOp::Load</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="136" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Overlay Command Dock UI</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Units and one precise rende…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Clip Space (-1..+1) Output</text>
  <rect x="900" y="46" width="148" height="96" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="900" y1="94" x2="1048" y2="94" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="974" y1="46" x2="974" y2="142" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <polygon points="974,56 926,132 1022,132" fill="#ed5c87" stroke="#111111" stroke-width="2"/>
  <circle cx="974" cy="56" r="3.5" fill="#111111"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="982" y="59" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,+0.5)</text>
  <circle cx="926" cy="132" r="3.5" fill="#111111"/>
  <circle cx="1022" cy="132" r="3.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Verify the shared unit th…</text>
</svg>
</div>

## <a id="checkpoint-96zzzzy-prepare-unit-aware-anchored-camera-space"></a>1. Prepare unit-aware anchored camera space

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/camera.rs, src/measure_…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Scene coordinate − scene…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">camera view</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">clip coordinate</text>
</svg>
</div>

**What this code does:** Keep scene coordinates precise while expressing the camera projection in physical metres.

### 1. `src/camera.rs`

Move the existing measurement Unit and suffix method to the camera; this body was already typed.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
pub const NEAR_FRACTION:f64=1.0e-4;
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzy-prepare-unit-aware-anchored-camera-space-consolidated-1.rs"
```

### 2. `src/measure_mark.rs`

Keep the existing measurement name as an alias to the shared unit.

**Locate this existing block:**

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
            Self::Meters => ["m", "m²", "m³"],
        };
        values[power.clamp(1, 3) - 1]
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzy-prepare-unit-aware-anchored-camera-space-consolidated-2.rs"
```

### 3. `src/camera.rs`

Add conversion to metres to the moved unit while retaining its existing suffix method. Retain the file unit with camera state; source geometry stays in scene units. Start with the maintained viewer’s millimetre file unit. Build one unit-aware matrix, subtracting the anchor before narrowing any coordinate. Look at the target relative to the shared scene anchor in metres. Keep eye and target in the same physical coordinate space. Scale the near plane with the physical eye distance. Convert the shared scene/grid reach once. Express the orthographic image size in metres. Retain the existing scene-unit minimum reach during this coordinate preparation. Convert incoming anchor-relative scene vertices to metres in the same matrix.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.orientation = (tilt * turn).normalized();
    }

    pub fn view_projection(&self) -> Xform {
        let forward = self.orientation.rotate_vector(Vector::y_axis());
        let up = self.orientation.rotate_vector(Vector::z_axis());
        let target = Point::new(self.target[0], self.target[1], self.target[2]);
        let eye = Point::new(
            self.target[0] - forward[0] * self.distance,
            self.target[1] - forward[1] * self.distance,
            self.target[2] - forward[2] * self.distance,
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = self.distance * NEAR_FRACTION;
        let reach = self.grid.and_then(|grid| grid.reach_on(self.grid_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.orientation.rotate_vector(Vector::y_axis()))),self.target).ok()).map_or(self.radius, |reach| self.radius.max(reach));
        let far = (self.distance * 10.0).max(self.distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => Xform::perspective(2.0 * HALF_FOV, self.aspect, near, far),
            Projection::Orthographic => {
                let h = self.distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (self.distance + 2.0 * reach).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
            }
        };
        let shift=self.shift();
        Xform::translation(shift[0],shift[1],0.) * projection * view
    }

    pub fn shift(&self)->[f64;2] { [self.frame[0]+self.frame[2]-1.,1.-self.frame[1]-self.frame[3]] }
```

Replace it with:

```rust
        self.orientation = (tilt * turn).normalized();
    }

    pub fn view_projection(&self)->Xform {self.view_projection_anchored([0.;3])}

    pub fn view_projection_anchored(&self,anchor:[f64;3])->Xform {
        let scale=self.unit.to_meters();
        let local: [f64;3]=std::array::from_fn(|i|(self.target[i]-anchor[i])*scale);
        let distance=self.distance*scale;
        let forward = self.orientation.rotate_vector(Vector::y_axis());
        let up = self.orientation.rotate_vector(Vector::z_axis());
        let target = Point::new(local[0], local[1], local[2]);
        let eye = Point::new(
            local[0] - forward[0] * distance,
            local[1] - forward[1] * distance,
            local[2] - forward[2] * distance,
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = distance * NEAR_FRACTION;
        let reach = self.grid.and_then(|grid| grid.reach_on(self.grid_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.orientation.rotate_vector(Vector::y_axis()))),self.target).ok()).map_or(self.radius, |reach| self.radius.max(reach));
        let reach=reach*scale;
        let far = (distance * 10.0).max(distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => Xform::perspective(2.0 * HALF_FOV, self.aspect, near, far),
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6*scale);
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
            }
        };
        let shift=self.shift();
        let mut matrix=Xform::translation(shift[0],shift[1],0.) * projection * view * Xform::scale_xyz(scale,scale,scale);
        if self.projection==Projection::Perspective {for value in &mut matrix.m {*value/=scale;}}
        matrix
    }

    pub fn shift(&self)->[f64;2] { [self.frame[0]+self.frame[2]-1.,1.-self.frame[1]-self.frame[3]] }
```

**Locate this block:**

```rust
impl Default for Camera {
    fn default() -> Self {
        Self {
            target: [0.0; 3],
            distance: 3.0,
            radius: 1.0,
```

Replace it with:

```rust
impl Default for Camera {
    fn default() -> Self {
        Self {
            unit:Unit::Millimeters,
            target: [0.0; 3],
            distance: 3.0,
            radius: 1.0,
```

**Locate this block:**

```rust


pub struct Camera {
    pub target: [f64; 3],
    pub distance: f64,
    pub radius: f64,
```

Replace it with:

```rust


pub struct Camera {
    pub unit:Unit,
    pub target: [f64; 3],
    pub distance: f64,
    pub radius: f64,
```

**Locate this block:**

```rust
    }
}

pub const NEAR_FRACTION:f64=1.0e-4;

const HALF_FOV: f64 = std::f64::consts::FRAC_PI_6;
```

Replace it with:

```rust
    }
}


pub const NEAR_FRACTION:f64=1.0e-4;

const HALF_FOV: f64 = std::f64::consts::FRAC_PI_6;
```

**Locate this block:**

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
```

Replace it with:

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Unit {Millimeters,Meters}
impl Unit {pub fn to_meters(self)->f64 {match self {Self::Millimeters=>0.001,Self::Meters=>1.}}
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
```

### 4. `src/lib.rs`

Keep the optional coordinate fixtures outside required coding.

**Locate this existing block:**

```rust
pub mod camera;
```

After that block, type:

```rust
#[cfg(test)] mod camera_space_tests;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/camera_space_tests.rs`

Supply equivalent physical mm/metre projection/rays and distant submillimetre detail after f32 camera upload.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzy-prepare-unit-aware-anchored-camera-space-15.rs"
```

### 6. `src/camera.rs`

Invert around the precise camera target so large absolute translations cannot spoil ray construction. Compute ray direction and extent locally, then restore the precise scene-space origin.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }
        Some(Ray { origin, direction: delta.normalized(), max_distance })
    }
}
```

Replace it with:

```rust
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }
        let origin=Point::new(origin[0]+self.target[0],origin[1]+self.target[1],origin[2]+self.target[2]);
        Some(Ray { origin, direction: delta.normalized(), max_distance })
    }
}
```

**Locate this block:**

```rust

    pub fn ray_unbounded(&self,screen:[f32;2])->Option<Ray> {
        if screen.iter().any(|v|!v.is_finite()) {return None;}
        let inverse = self.view_projection().inverse()?;
        let origin = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, 0.0));
        let end = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, 1.0));
        let delta = &end - &origin;
```

Replace it with:

```rust

    pub fn ray_unbounded(&self,screen:[f32;2])->Option<Ray> {
        if screen.iter().any(|v|!v.is_finite()) {return None;}
        let inverse = self.view_projection_anchored(self.target).inverse()?;
        let origin = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, 0.0));
        let end = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, 1.0));
        let delta = &end - &origin;
```

### 7. `src/editor.rs`

Remove the independent editor unit value. Use the camera’s existing default scene unit. Measure selected originals using the same scene unit as projection. Give distance annotations that same unit interpretation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    pub fn guide_overlay(&self)->crate::guide_gpu::Overlay{
```

Replace it with:

```rust
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.camera.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.camera.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    pub fn guide_overlay(&self)->crate::guide_gpu::Overlay{
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            element:None,element_attributes:false,element_interactions:true,opacity:1.,arctic:false,outlines:false,clipping_enabled:true,clipping_solid:true,interaction_epoch:0,lasso:None,trim:None,last_text_height:0.,mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            element:None,element_attributes:false,element_interactions:true,opacity:1.,arctic:false,outlines:false,clipping_enabled:true,clipping_solid:true,interaction_epoch:0,lasso:None,trim:None,last_text_height:0.,mark:None,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
```

**Locate this block:**

```rust
    pub lasso:Option<crate::lasso::Lasso>,
    pub trim:Option<crate::trim_draft::Draft>,
    pub last_text_height:f64,
    pub mark:Option<crate::measure_mark::Mark>,pub unit:crate::measure_mark::Unit,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
```

Replace it with:

```rust
    pub lasso:Option<crate::lasso::Lasso>,
    pub trim:Option<crate::trim_draft::Draft>,
    pub last_text_height:f64,
    pub mark:Option<crate::measure_mark::Mark>,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/measure_tests.rs`

Retain all existing area, volume, original-face and open-shape assertions under the shared unit owner.

**Locate this existing block:**

```text
    e.unit = Unit::Meters;
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzi-share-one-camera-and-measurement-unit-07.rs"
```

## <a id="checkpoint-96zzzzz-prepare-precise-anchor-relative-object-records"></a>2. Prepare precise anchor-relative object records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/render_space.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original f64 placement</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">twelve GPU translation by…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing object record co…</text>
</svg>
</div>

**What this code does:** Retain precise placements while updating only their GPU translation.

### 1. `src/render_space.rs`

Share double-precision anchor subtraction before GPU narrowing.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzz-prepare-precise-anchor-relative-object-records-01.rs"
```

### 2. `src/lib.rs`

Keep anchor-relative GPU conversion under one coordinate owner.

**Locate this existing block:**

```rust
pub mod camera;
```

After that block, type:

```rust
mod render_space;
```

### 3. `src/gpu_mesh.rs`

Retain precise placement translation and the anchor used by its GPU mirror. Make the placement anchor explicit at the one record-packing boundary. Narrow translation only after subtracting the precise anchor; retain the original linear transform. New isolated GPU geometry begins in the ordinary zero-anchor frame. Retain source placement independently of its narrowed uniform and shared table. Rebase only the three translation values; an unchanged relative translation writes nothing. Refresh the precise retained translation for ordinary object edits. Keep selection, styles and placement edits in the current anchor frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
```

Replace it with:

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
```

**Locate this block:**

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}
```

Replace it with:

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { translation:object.model.m[12..15].try_into().unwrap(),anchor:[0.;3],geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
        assert!(anchor.iter().all(|v|v.is_finite()));self.anchor=anchor;
        let bytes=crate::render_space::relative(self.translation,anchor).into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
        if self.previous[48..60]==bytes {return [0,0];}
        self.previous[48..60].copy_from_slice(&bytes);queue.write_buffer(&self.model,48,&bytes);[1,12]
    }
    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}
```

**Locate this block:**

```rust
    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style,0,0,0,geometry.index_start);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

Replace it with:

```rust
    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style,0,0,0,geometry.index_start,[0.;3]);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

**Locate this block:**

```rust
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
```

Replace it with:

```rust
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
    let mut model=object.model.to_f32();
    model[12..15].copy_from_slice(&crate::render_space::relative(object.model.m[12..15].try_into().unwrap(),anchor));
    for (index, value) in model.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply actual distant/local pixel equality, exact mapped object records, unchanged geometry uploads, original picks and selection, repeated-anchor zero writes and precise placement edits. Derived ink is omitted explicitly in this isolated surface fixture.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzz-prepare-precise-anchor-relative-object-records-11.rs" skip=40415
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/render_space.rs`

Supply exact threshold, strict throttle boundary, pending/settled state, upper clamp, invalid-input atomicity and reset coverage for the prepared origin policy.

**Locate this existing block:**

```text
pub fn relative(translation:[f64;3],anchor:[f64;3])->[f32;3] {std::array::from_fn(|i|(translation[i]-anchor[i])as f32)}
#[derive(Default)]
pub struct Origin {anchor:Option<[f64;3]>,last_ms:f64}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rebase {pub anchor:[f64;3],pub moved:bool,pub pending:bool}
impl Origin {
 pub fn forget(&mut self){self.anchor=None;}
 pub fn update(&mut self,target:[f64;3],distance:f64,now:f64)->Option<Rebase>{
  if target.iter().any(|v|!v.is_finite())||!distance.is_finite()||distance<=0.||!now.is_finite(){return None;}
  let threshold=(distance*0.25).clamp(1000.,100000.);
  let need=self.anchor.is_none_or(|a|(0..3).map(|i|(a[i]-target[i]).powi(2)).sum::<f64>().sqrt()>threshold);
  let moved=need&&(self.anchor.is_none()||now-self.last_ms>200.);
  if moved {self.anchor=Some(target);self.last_ms=now;}
  Some(Rebase {anchor:self.anchor.unwrap(),moved,pending:need&&!moved})
 }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzz-prepare-precise-anchor-relative-object-records-12.rs" skip=914
```

## <a id="checkpoint-96zzzzza-prepare-anchor-relative-original-ink"></a>3. Prepare anchor-relative ink

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retained original/sample</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">subtract scene anchor</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing packed ink record</text>
</svg>
</div>

**What this code does:** Pack original ink positions only after subtracting the shared scene origin.

### 1. `src/scene.rs`

Retain the existing line API while making its packing origin explicit. Subtract the anchor from placed precise endpoints before conversion to GPU floats. Reuse one path-neighbour algorithm for zero and explicit origins. Filter local GPU duplicates after the retained precise sample and original placement. Read the original point and placement before subtracting the anchor; retain its current pixel style. Share the existing original control collector under an explicit packing origin. Retain precise collected control positions through the GPU boundary. Keep original parent-only net ownership while packing its links in the chosen frame. Use the same precise conversion for both ends of a control net.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        markers
    }

    pub fn control_nets(&self) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
                let Some((a, b)) = controls.points.get(a).zip(controls.points.get(b)) else { continue };
                let start = a.position.map(|v| v as f32);
                let end = b.position.map(|v| v as f32);
                if start.iter().chain(&end).any(|v| !v.is_finite()) { continue; }
                nets.push((owner, crate::stroke::Stroke { start, end, colour: [102./255., 136./255., 204./255., 1.], width: 1. }));
            }
```

Replace it with:

```rust
        markers
    }

    pub fn control_nets(&self)->Vec<(ObjectId,crate::stroke::Stroke)> {self.control_nets_at([0.;3])}
    pub fn control_nets_at(&self,anchor:[f64;3])->Vec<(ObjectId,crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
                let Some((a, b)) = controls.points.get(a).zip(controls.points.get(b)) else { continue };
                let start = crate::render_space::relative(a.position,anchor);
                let end = crate::render_space::relative(b.position,anchor);
                if start.iter().chain(&end).any(|v| !v.is_finite()) { continue; }
                nets.push((owner, crate::stroke::Stroke { start, end, colour: [102./255., 136./255., 204./255., 1.], width: 1. }));
            }
```

**Locate this block:**

```rust
        }).collect()
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = control.position.map(|v| v as f32);
                if center.iter().any(|v| !v.is_finite()) { continue; }
                let part = match (legacy, control.id) {
                    (true, crate::source_controls::ControlId::Curve { point, .. }) => crate::pick_sources::Part::Control(point),
```

Replace it with:

```rust
        }).collect()
    }

    pub fn control_markers(&self)->Vec<ControlMarker> {self.control_markers_at([0.;3])}
    pub fn control_markers_at(&self,anchor:[f64;3])->Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = crate::render_space::relative(control.position,anchor);
                if center.iter().any(|v| !v.is_finite()) { continue; }
                let part = match (legacy, control.id) {
                    (true, crate::source_controls::ControlId::Curve { point, .. }) => crate::pick_sources::Part::Control(point),
```

**Locate this block:**

```rust
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self) -> crate::marker::Marker { crate::marker::Marker { center: self.point().to_f32(), ..self.prepared.display } }
}

#[derive(Clone)]
```

Replace it with:

```rust
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self)->crate::marker::Marker {self.marker_at([0.;3])}
    pub fn marker_at(&self,anchor:[f64;3])->crate::marker::Marker {crate::marker::Marker {center:crate::render_space::relative(crate::clipping_geometry::array(&self.point()),anchor),..self.prepared.display}}
}

#[derive(Clone)]
```

**Locate this block:**

```rust
impl PathObject {
    pub fn visible(&self) -> bool { self.prepared.source.visible() }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = std::array::from_fn(|i| p[i] as f32);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
```

Replace it with:

```rust
impl PathObject {
    pub fn visible(&self) -> bool { self.prepared.source.visible() }

    pub fn segments(&self)->Vec<crate::chain::Segment> {self.segments_at([0.;3])}
    pub fn segments_at(&self,anchor:[f64;3])->Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = crate::render_space::relative(crate::clipping_geometry::array(&p),anchor);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
```

**Locate this block:**

```rust
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32), ..self.prepared.display }
    }
}
```

Replace it with:

```rust
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self)->crate::stroke::Stroke {self.stroke_at([0.;3])}
    pub fn stroke_at(&self,anchor:[f64;3])->crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: crate::render_space::relative(crate::clipping_geometry::array(&start),anchor),
            end: crate::render_space::relative(crate::clipping_geometry::array(&end),anchor), ..self.prepared.display }
    }
}
```

### 2. `src/lib.rs`

Keep optional precise source-packing fixtures outside required learner coding.

**Locate this existing block:**

```rust
mod render_space;
```

After that block, type:

```rust
#[cfg(test)] mod scene_origin_tests;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/scene_origin_tests.rs`

Supply actual original line/path/point placement, fractional bytes, chain neighbours, parent/control IDs, net styles and retained original/cache pointers at a large coordinate.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzza-prepare-anchor-relative-original-ink-11.rs"
```

### 4. `src/pick_sources.rs`

Reuse the original source-table owner in the same coordinate frame as its packed ink. Pack placed precise mesh boundaries relative to the same anchor without changing their original edge keys. Build compact path addresses from the same local duplicate predicate as the drawable chain while retaining each raw original edge index. Keep control-net positions and original parent addresses in one frame. Keep control marker addresses aligned with their anchor-relative original positions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets() {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers() {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
```

Replace it with:

```rust
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
```

**Locate this block:**

```rust
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points: Vec<_> = row.points().iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
```

Replace it with:

```rust
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points().iter().map(|p|crate::render_space::relative(crate::clipping_geometry::array(p),anchor)).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
```

**Locate this block:**

```rust
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends = ends.map(|p| row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
```

Replace it with:

```rust
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends=ends.map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));crate::render_space::relative(crate::clipping_geometry::array(&p),anchor)});
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
```

**Locate this block:**

```rust

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene: &Scene) -> Result<Self, &'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
```

Replace it with:

```rust

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
```

## <a id="checkpoint-96zzzzzb-prepare-shared-origin-frame-inputs"></a>4. Prepare shared-origin frame inputs

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/render_space.rs, src/re…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">One renderer origin</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local GPU clipping + prec…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local GPU clipping + prec…</text>
</svg>
</div>

**What this code does:** Keep derived rendering inputs consistent with one precise scene origin.

### 1. `src/render_space.rs`

Expose the current origin once, using the existing zero frame before its first update.

**Locate this existing block:**

```rust
 pub fn forget(&mut self){self.anchor=None;}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub fn anchor(&self)->[f64;3] {self.anchor.unwrap_or([0.;3])}
 pub fn forget(&mut self){self.anchor=None;}
```

### 2. `src/renderer.rs`

Keep one origin policy under the renderer owner. Begin in the existing zero-origin frame. Let every derived consumer read the same precise origin.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
```

Replace it with:

```rust
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
```

**Locate this block:**

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
```

Replace it with:

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
```

**Locate this block:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    opacity:std::cell::Cell<f32>,
    ambient_enabled:std::cell::Cell<bool>,
    ambient:std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
```

Replace it with:

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    space:crate::render_space::Origin,
    opacity:std::cell::Cell<f32>,
    ambient_enabled:std::cell::Cell<bool>,
    ambient:std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
```

### 3. `src/clipping_gpu.rs`

Reuse one clipping conversion with an explicit anchor while retaining zero-origin callers. Use the existing precise plane/hatch conversion relative to the declared frame origin.

Replace the entire file with:

```rust
use crate::{clipping_rows::Row,clipping_uniform::{ClipUniform,ClipView,clip_uniform}};
use wgpu::util::DeviceExt;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout{crate::gpu_layout::uniform(device,448)}
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),crate::object_record::shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
pub fn bytes(uniform:&ClipUniform)->Vec<u8>{
    let mut bytes=uniform.planes.iter().chain(&uniform.screen).chain(&uniform.hatch).chain(&uniform.hatch_w).chain(&uniform.sides).flatten().copied().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
    bytes.extend([uniform.count,uniform.samples,uniform.fill].into_iter().flat_map(u32::to_ne_bytes));bytes.extend([uniform.spacing,uniform.width,uniform.outline,uniform.pad[0],uniform.pad[1]].into_iter().flat_map(f32::to_ne_bytes));assert_eq!(bytes.len(),448);bytes
}
pub struct Clipping{pub buffer:wgpu::Buffer,pub group:wgpu::BindGroup,pub rows:Vec<Row>,pub error:Option<&'static str>,pub enabled:bool,pub solid:bool,key:Vec<u8>,pub writes:u64}
impl Clipping{
    pub fn new(device:&wgpu::Device)->Self{let key=bytes(&ClipUniform::default());let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("current original clipping planes"),contents:&key,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&layout(device),&buffer);Self{buffer,group,rows:Vec::new(),error:None,enabled:true,solid:true,key,writes:0}}
    pub fn set(&mut self,scene:&crate::scene::Scene){match crate::clipping_rows::current(scene){Ok(rows)=>{self.rows=rows;self.error=None},Err(error)=>{self.rows.clear();self.error=Some(error)}}}
    pub fn frame(&mut self,queue:&wgpu::Queue,matrix:[f32;16],height:u32,density:f32){self.frame_anchored(queue,matrix,[0.;3],height,density)}
    pub fn frame_anchored(&mut self,queue:&wgpu::Queue,matrix:[f32;16],anchor:[f64;3],height:u32,density:f32){let matrix=session_rust::Xform::from_matrix(matrix.map(f64::from));let planes=if self.enabled{self.rows.iter().map(|r|r.plane).collect::<Vec<_>>()}else{Vec::new()};let uniform=clip_uniform(&planes,u32::from(self.solid),&ClipView{view_proj:&matrix,anchor,height,pixel_scale:f64::from(density),samples:1,forward_depth:true});let bytes=bytes(&uniform);if bytes!=self.key{queue.write_buffer(&self.buffer,0,&bytes);self.key=bytes;self.writes+=1;}}
}
```

### 4. `src/renderer.rs`

Prepare pick clipping in the renderer’s current frame. Keep the first-pick crossing preparation on the same origin. Use the same origin for visible clipping and cap inputs. Provide the precise text origin; retain world planes for the text engine’s pre-subtraction CPU visibility test.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame(&self.queue,*transform,self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
```

Replace it with:

```rust
    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
```

**Locate this block:**

```rust
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
```

Replace it with:

```rust
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:self.anchor(),framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
```

**Locate this block:**

```rust
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame(&self.queue,transform,self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame(&self.queue,transform,self.size[1],self.density);
            let mut bytes=transform.iter().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();bytes.resize(80,0);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
```

Replace it with:

```rust
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let mut bytes=transform.iter().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();bytes.resize(80,0);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
```

### 5. `src/ambient_rows.rs`

Retain precise world floor height with the ambient source owner. Subtract the frame origin before narrowing ambient floor height; radius remains a distance. Avoid losing a small floor displacement during source preparation.

Replace the entire file with:

```rust
#[derive(Default)]
pub struct Rows{bounds:crate::cap_source_bounds::Bounds,pub specs:Vec<(u32,f32)>,pub ground:[f64;2]}
impl Rows{
 pub fn ground_at(&self,anchor:[f64;3])->[f32;2] {[(self.ground[0]-anchor[2])as f32,self.ground[1]as f32]}
 pub fn set(&mut self,scene:&crate::scene::Scene){self.specs.clear();let mut floor=f64::INFINITY;let mut radius=0f32;for object in scene.objects().iter().filter(|r|scene.visible(r.id)&&!r.mesh.indices().is_empty()){if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let r=crate::ambient_radius::ambient_radius(&bounds);self.specs.push((object.id.value(),r));floor=floor.min(bounds.cz-bounds.hz);radius=radius.max(r);}}self.ground=[if floor.is_finite(){floor}else{0.},f64::from(radius)];self.bounds.prune();if self.specs.is_empty(){self.specs.shrink_to_fit();}}
}
```

### 6. `src/renderer.rs`

Feed ambient preparation the same anchor-relative ground height.

**Locate this existing block:**

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Supply actual local/distant clipped cap pixels, shared object records, world-space text predicates, exact relative ground height, unchanged source uploads and repeated-uniform reuse.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzb-prepare-shared-origin-frame-inputs-15.rs" skip=42732
```

## <a id="checkpoint-96zzzzzc-prepare-cloud-and-sheet-frame-placements"></a>5. Prepare cloud and sheet frame placements

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/render_space.rs, src/gp…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original f64 placement −…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local model/eye</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing LOD, records and…</text>
</svg>
</div>

**What this code does:** Preserve cached cloud and sheet sources while their frame origin changes.

### 1. `src/render_space.rs`

Subtract the origin in double precision before GPU narrowing or matrix composition.

**Locate this existing block:**

```rust
#[derive(Default)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn placed(place:&session_rust::Xform,anchor:[f64;3])->session_rust::Xform {
 let mut local=place.clone();for i in 0..3 {local.m[12+i]-=anchor[i];}local
}
#[derive(Default)]
```

### 2. `src/gpu_mesh.rs`

Reuse the same precise model conversion for existing object records.

**Locate this existing block:**

```rust
    let mut model=object.model.to_f32();
    model[12..15].copy_from_slice(&crate::render_space::relative(object.model.m[12..15].try_into().unwrap(),anchor));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let model=crate::render_space::placed(&object.model,anchor).to_f32();
```

### 3. `src/cloud_gpu.rs`

Carry the declared origin with cloud view inputs. Retain the earlier default cloud frame. Keep zero-origin callers while preparing one explicit camera-relative cloud view. Retain the exact origin alongside the narrowed eye. Subtract the anchor before the eye offset and narrowing, preserving local camera detail. Include origin identity independently of matrix and eye values. Invalidate retained cloud records when only the frame origin changes. Select cloud detail with eye and source placement in the same local frame. Prepare one precise local placement for cloud records and normal/position inputs. Compose projection after subtracting the precise source placement, then narrow once.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
  ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
  let row = &draw.row;
  let model = row.model.to_f32();
  let scale = row.model.uniform_scale();
  let mut total = 0u32;
  let m = (&session_rust::Xform::from_matrix(transform.map(f64::from)) * &row.model).to_f32();
  let rot = [
    model[0], model[1], model[2], model[12], model[4], model[5], model[6], model[13], model[8], model[9], model[10],
    model[14],
```

Replace it with:

```rust
  ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
  let row = &draw.row;
  let local=crate::render_space::placed(&row.model,view.anchor);
  let model=local.to_f32();
  let scale = row.model.uniform_scale();
  let mut total = 0u32;
  let m=(&session_rust::Xform::from_matrix(transform.map(f64::from))*&local).to_f32();
  let rot = [
    model[0], model[1], model[2], model[12], model[4], model[5], model[6], model[13], model[8], model[9], model[10],
    model[14],
```

**Locate this block:**

```rust
    view: View,
  ) -> Result<(), &'static str> {
    let key = Key {
      transform,
      size,
      density: density.to_bits(),
```

Replace it with:

```rust
    view: View,
  ) -> Result<(), &'static str> {
    let key = Key {
      anchor:view.anchor.map(f64::to_bits),
      transform,
      size,
      density: density.to_bits(),
```

**Locate this block:**

```rust
        continue;
      }
      let row = &draw.row;
      let model = row.model.to_f32();
      let projection = Projection {
        eye: view.eye,
        ortho_h: view.ortho_h,
```

Replace it with:

```rust
        continue;
      }
      let row = &draw.row;
      let model=crate::render_space::placed(&row.model,view.anchor).to_f32();
      let projection = Projection {
        eye: view.eye,
        ortho_h: view.ortho_h,
```

**Locate this block:**

```rust
}
#[derive(PartialEq)]
struct Key {
  transform: [f32; 16],
  size: [u32; 2],
  density: u32,
```

Replace it with:

```rust
}
#[derive(PartialEq)]
struct Key {
  anchor:[u64;3],
  transform: [f32; 16],
  size: [u32; 2],
  density: u32,
```

**Locate this block:**

```rust

#[derive(Clone, Copy)]
pub struct View {
  pub eye: [f32; 3],
  pub ortho_h: f32,
  pub lod_px: f32,
}
impl Default for View {
  fn default() -> Self {
    Self { eye: [0., 0., 3.], ortho_h: 0., lod_px: 2. }
  }
}
impl View {
  pub fn camera(camera: &crate::camera::Camera) -> Self {
    let forward = camera.orientation.rotate_vector(session_rust::Vector::y_axis());
    Self {
      eye: std::array::from_fn(|i| (camera.target[i] - forward[i] * camera.distance) as f32),
      ortho_h: if camera.projection == crate::camera::Projection::Orthographic {
        (camera.distance * std::f64::consts::FRAC_PI_6.tan()) as f32
      } else {
```

Replace it with:

```rust

#[derive(Clone, Copy)]
pub struct View {
  pub anchor:[f64;3],
  pub eye: [f32; 3],
  pub ortho_h: f32,
  pub lod_px: f32,
}
impl Default for View {
  fn default() -> Self {
    Self { anchor:[0.;3],eye: [0., 0., 3.], ortho_h: 0., lod_px: 2. }
  }
}
impl View {
  pub fn camera(camera:&crate::camera::Camera)->Self {Self::anchored(camera,[0.;3])}
  pub fn anchored(camera:&crate::camera::Camera,anchor:[f64;3])->Self {
    let forward = camera.orientation.rotate_vector(session_rust::Vector::y_axis());
    Self {
      anchor,
      eye: std::array::from_fn(|i| (camera.target[i] - anchor[i] - forward[i] * camera.distance) as f32),
      ortho_h: if camera.projection == crate::camera::Projection::Orthographic {
        (camera.distance * std::f64::consts::FRAC_PI_6.tan()) as f32
      } else {
```

### 4. `src/sheet_gpu.rs`

Reuse sheet geometry/cache ownership with an explicit placement origin. Build each original sheet placement in its declared frame. Make precise sheet placement conversion explicit without changing original entity addresses. Reuse the same double-precision local model before packing sheet uniforms.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
}

fn settings(scene: &Scene, row: &crate::sheet_source::Row, base: u32, selected: bool) -> Vec<u8> {
    let mut bytes = row
        .model
        .m
        .iter()
        .flat_map(|v| (*v as f32).to_le_bytes())
```

Replace it with:

```rust
    }
}

fn settings(scene: &Scene, row: &crate::sheet_source::Row, base: u32, selected: bool,anchor:[f64;3]) -> Vec<u8> {
    let mut bytes = crate::render_space::placed(&row.model,anchor)
        .m
        .iter()
        .flat_map(|v| (*v as f32).to_le_bytes())
```

**Locate this block:**

```rust
                        .ok_or("Missing sheet original entity addresses")?
                        .checked_add(rows.start)
                        .ok_or("Drawing pick address overflow")?;
                    let bytes = settings(scene, row, base, selected.contains(&row.id));
                    let draw = if let Some(i) = old
                        .iter()
                        .position(|d| d.id == row.id && Rc::ptr_eq(&d.geometry, &geometry))
```

Replace it with:

```rust
                        .ok_or("Missing sheet original entity addresses")?
                        .checked_add(rows.start)
                        .ok_or("Drawing pick address overflow")?;
                    let bytes = settings(scene, row, base, selected.contains(&row.id),anchor);
                    let draw = if let Some(i) = old
                        .iter()
                        .position(|d| d.id == row.id && Rc::ptr_eq(&d.geometry, &geometry))
```

**Locate this block:**

```rust
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
    ) -> Result<(), &'static str> {
        let mut old = std::mem::take(&mut self.draws);
        for row in scene
            .sheets()
```

Replace it with:

```rust
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
    ) -> Result<(), &'static str> {
        self.set_anchored(device,queue,scene,selected,bases,[0.;3])
    }
    pub fn set_anchored(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,scene:&Scene,selected:&[ObjectId],bases:&HashMap<(ObjectId,usize),u32>,anchor:[f64;3])->Result<(),&'static str>{
        let mut old = std::mem::take(&mut self.draws);
        for row in scene
            .sheets()
```

### 5. `src/renderer.rs`

Capture the renderer origin before mutable lane updates. Keep sheet source adoption in the renderer’s current frame. Keep selection updates in the retained frame. Preserve original sheet selection without reverting its local placement. Supply cloud eye, origin and record-cache inputs from one renderer frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::camera(camera));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
```

Replace it with:

```rust
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::anchored(camera,self.anchor()));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
```

**Locate this block:**

```rust
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }
```

Replace it with:

```rust
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }
```

**Locate this block:**

```rust
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
```

Replace it with:

```rust
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        let anchor=self.anchor();
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
```

**Locate this block:**

```rust
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
```

Replace it with:

```rust
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set_anchored(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases,anchor).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
```

**Locate this block:**

```rust
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
```

Replace it with:

```rust
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor=self.anchor();
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/cap_gpu_tests.rs`

Reuse the existing optional GPU readback fixture with an explicit matrix; retain every earlier cap pixel assertion.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzzc-prepare-cloud-and-sheet-frame-placements-22.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Supply actual local/distant cloud and sheet pixels, unchanged uploads, equal-matrix camera pan/rebase cache invalidation, exact restoration, original picks and empty release while sources remain retained.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzc-prepare-cloud-and-sheet-frame-placements-23.rs" skip=44675
```

## <a id="checkpoint-96zzzzzd-prepare-local-temporary-overlays"></a>6. Prepare local temporary overlays

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tool_preview.rs, src/gu…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Precise temporary input</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing ink lane; world…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing text frame</text>
</svg>
</div>

**What this code does:** Keep temporary ink and cached measurement positions precise across origin changes.

### 1. `src/tool_preview.rs`

Reuse the draft preview collector with an explicit frame origin. Retain precise draft outline positions until the local GPU boundary. Pack the precise cursor marker in the same frame.

Replace the entire file with:

```rust
use crate::{stroke_gpu::Lane, tool::Draft};

pub struct Preview { strokes: Lane, markers: Lane }

impl Preview {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self { strokes: Lane::new(device, format), markers: Lane::markers(device, format) }
    }
    pub fn set(&mut self,device:&wgpu::Device,draft:Option<&Draft>)->Result<(),&'static str>{self.set_at(device,draft,[0.;3])}
    pub fn set_at(&mut self,device:&wgpu::Device,draft:Option<&Draft>,anchor:[f64;3])->Result<(),&'static str>{
        let yellow = crate::colour::selection(); let mut strokes = Vec::new(); let mut markers = Vec::new();
        if let Some(draft) = draft {
            for wire in if draft.extend.is_some(){Vec::new()}else{draft.outlines()} {for pair in wire.windows(2) {if pair[0]!=pair[1] {strokes.push(crate::stroke::Stroke {start:crate::render_space::relative(pair[0],anchor),end:crate::render_space::relative(pair[1],anchor),width:2.,colour:yellow});}}}
            if let Some(cursor) = draft.cursor {
                markers.push(crate::marker::Marker { center: crate::render_space::relative(cursor,anchor), diameter: 8., colour: yellow });

        } }
        self.strokes.set(device, strokes)?; self.markers.set_points(device, markers)?; Ok(())
    }
    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        self.strokes.view(queue, transform, size, density); self.markers.view(queue, transform, size, density);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) { self.strokes.draw(pass); self.markers.draw(pass); }
    pub fn bytes(&self) -> u64 { self.strokes.usage()[1] + self.markers.usage()[1] }
}
```

### 2. `src/guide_gpu.rs`

Retain the collector origin as part of guide cache identity. Share precise marker packing across trim and extension guides. Narrow guide wires only after subtracting their collector origin.

Replace the entire file with:

```rust
use crate::{stroke::Stroke,marker::Marker,stroke_gpu::Lane};
#[derive(Clone,Debug,PartialEq)]
pub struct Segment{pub stroke:Stroke,pub dashed:bool}
#[derive(Clone,Debug,Default,PartialEq)]
pub struct Overlay{pub anchor:[f64;3],pub segments:Vec<Segment>,pub marks:Vec<Marker>,pub label:Option<([f64;3],String)>}
impl Overlay{
    pub fn at(anchor:[f64;3])->Self {Self {anchor,..Default::default()}}
    pub fn mark(&mut self,point:[f64;3],diameter:f32,colour:[f32;4]){self.marks.push(Marker{center:crate::render_space::relative(point,self.anchor),diameter,colour});}
    pub fn wire(&mut self,points:&[[f64;3]],colour:[f32;4],dashed:bool){for p in points.windows(2){if p[0]!=p[1]{self.segments.push(Segment{stroke:Stroke{start:crate::render_space::relative(p[0],self.anchor),end:crate::render_space::relative(p[1],self.anchor),colour,width:2.},dashed})}}}
    pub fn labels(&self,existing:&[crate::text::TextLabel])->Vec<crate::text::TextLabel>{self.label.as_ref().map_or_else(Vec::new,|(point,label)|crate::measure_mark::Mark{points:vec![*point],label:label.clone()}.labels(existing))}
}
pub struct Guide{pub overlay:Overlay,segments:Lane,marks:Lane}
impl Guide{
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{overlay:Default::default(),segments:Lane::guides(device,format),marks:Lane::markers(device,format)}}
    pub fn set(&mut self,device:&wgpu::Device,overlay:&Overlay)->Result<bool,&'static str>{if &self.overlay==overlay{return Ok(false)}self.segments.set_guides(device,&overlay.segments)?;self.marks.set_points(device,overlay.marks.iter().copied())?;self.overlay=overlay.clone();Ok(true)}
    pub fn view(&self,queue:&wgpu::Queue,matrix:&[f32;16],size:[u32;2],density:f32){self.segments.view(queue,matrix,size,density);self.marks.view(queue,matrix,size,density)}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>){self.segments.draw(pass);self.marks.draw(pass)}
    pub fn bytes(&self)->u64{self.segments.usage()[1]+self.marks.usage()[1]}
}
```

### 3. `src/editor.rs`

Keep the legacy collector and expose the shared frame to guide producers. Route trim guidance through the same origin-aware collector. Collect extension wires under the declared frame. Keep the precise guide point and world label before their separate GPU boundaries.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.camera.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    pub fn guide_overlay(&self)->crate::guide_gpu::Overlay{
        if let Some(trim)=&self.trim{return trim.overlay()}
        let mut overlay=crate::guide_gpu::Overlay::default();let red=[210./255.,40./255.,40./255.,1.];
        let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref())else{return overlay};
        let label=if let Some(hover)=&extend.hover{
            overlay.wire(&hover.tail,red,true);hover.tail.last().map(|point|{let added=hover.tail.windows(2).map(|p|(p[1][0]-p[0][0]).hypot(p[1][1]-p[0][1]).hypot(p[1][2]-p[0][2])).sum::<f64>();(*point,format!("+{added:.3}"))})
        }else if !extend.error.is_empty(){extend.last.and_then(|(id,end)|{
            let owner=&extend.candidates.iter().find(|c|c.0.id==id)?.0;let (tip,_)=crate::extend_reach::tip(&owner.source,end)?;let p=tip.transformed(&owner.model);Some(([p[0],p[1],p[2]],extend.error.clone()))
        })}else{None};
        if let Some((point,word))=label{overlay.marks.push(crate::marker::Marker{center:point.map(|v|v as f32),diameter:6.,colour:red});overlay.label=Some((point,word));}overlay
    }

    pub fn trimming_picks(&self)->bool{self.trim.as_ref().is_some_and(|t|t.picks())}
```

Replace it with:

```rust
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.camera.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    pub fn guide_overlay(&self)->crate::guide_gpu::Overlay {self.guide_overlay_at([0.;3])}
    pub fn guide_overlay_at(&self,anchor:[f64;3])->crate::guide_gpu::Overlay{
        if let Some(trim)=&self.trim{return trim.overlay_at(anchor)}
        let mut overlay=crate::guide_gpu::Overlay::at(anchor);let red=[210./255.,40./255.,40./255.,1.];
        let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref())else{return overlay};
        let label=if let Some(hover)=&extend.hover{
            overlay.wire(&hover.tail,red,true);hover.tail.last().map(|point|{let added=hover.tail.windows(2).map(|p|(p[1][0]-p[0][0]).hypot(p[1][1]-p[0][1]).hypot(p[1][2]-p[0][2])).sum::<f64>();(*point,format!("+{added:.3}"))})
        }else if !extend.error.is_empty(){extend.last.and_then(|(id,end)|{
            let owner=&extend.candidates.iter().find(|c|c.0.id==id)?.0;let (tip,_)=crate::extend_reach::tip(&owner.source,end)?;let p=tip.transformed(&owner.model);Some(([p[0],p[1],p[2]],extend.error.clone()))
        })}else{None};
        if let Some((point,word))=label{overlay.mark(point,6.,red);overlay.label=Some((point,word));}overlay
    }

    pub fn trimming_picks(&self)->bool{self.trim.as_ref().is_some_and(|t|t.picks())}
```

### 4. `src/trim_draft.rs`

Preserve one trim overlay algorithm while making its local origin explicit. Retain precise trim intersection points until guide marker packing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let mut preview=scene.clone();for cut in &self.cuts{cut.owner.valid(scene)?;if !cut.removed.is_empty(){let pieces=crate::trim_parts::kept(&cut.owner.source,&cut.parts,&cut.removed)?;crate::split_draft::adopt(&mut preview,cut.owner.id,pieces)?;}}
        self.preview=Some(preview);Ok(())
    }
    pub fn overlay(&self)->crate::guide_gpu::Overlay{let mut overlay=crate::guide_gpu::Overlay::default();if self.phase!=Phase::Parts{return overlay}let red=[210./255.,40./255.,40./255.,1.];let blue=[30./255.,110./255.,170./255.,1.];for(i,cut)in self.cuts.iter().enumerate(){for(part,line)in cut.outlines.iter().enumerate(){if cut.removed.contains(&part){continue}let hovered=self.hover==Some((i,part));let points=line.iter().map(|p|[p[0],p[1],p[2]]).collect::<Vec<_>>();overlay.wire(&points,if hovered{red}else{blue},hovered);if hovered{if let Some(point)=points.get(points.len()/2){overlay.label=Some((*point,"remove".into()))}}}for line in &cut.section{overlay.wire(&line.iter().map(|p|[p[0],p[1],p[2]]).collect::<Vec<_>>(),red,false)}overlay.marks.extend(cut.marks.iter().map(|p|crate::marker::Marker{center:p.to_f32(),diameter:6.,colour:blue}));}overlay}
    pub fn changed(&self)->bool{self.cuts.iter().any(|c|!c.removed.is_empty())}
    pub fn prepared_edits(&self,scene:&Scene)->Result<Vec<(ObjectId,Vec<Geometry>)>,String>{self.cuts.iter().filter(|c|!c.removed.is_empty()).map(|c|{c.owner.valid(scene)?;Ok((c.owner.id,crate::trim_parts::kept(&c.owner.source,&c.parts,&c.removed)?))}).collect()}
}
```

Replace it with:

```rust
        let mut preview=scene.clone();for cut in &self.cuts{cut.owner.valid(scene)?;if !cut.removed.is_empty(){let pieces=crate::trim_parts::kept(&cut.owner.source,&cut.parts,&cut.removed)?;crate::split_draft::adopt(&mut preview,cut.owner.id,pieces)?;}}
        self.preview=Some(preview);Ok(())
    }
    pub fn overlay(&self)->crate::guide_gpu::Overlay {self.overlay_at([0.;3])}
    pub fn overlay_at(&self,anchor:[f64;3])->crate::guide_gpu::Overlay{let mut overlay=crate::guide_gpu::Overlay::at(anchor);if self.phase!=Phase::Parts{return overlay}let red=[210./255.,40./255.,40./255.,1.];let blue=[30./255.,110./255.,170./255.,1.];for(i,cut)in self.cuts.iter().enumerate(){for(part,line)in cut.outlines.iter().enumerate(){if cut.removed.contains(&part){continue}let hovered=self.hover==Some((i,part));let points=line.iter().map(|p|[p[0],p[1],p[2]]).collect::<Vec<_>>();overlay.wire(&points,if hovered{red}else{blue},hovered);if hovered{if let Some(point)=points.get(points.len()/2){overlay.label=Some((*point,"remove".into()))}}}for line in &cut.section{overlay.wire(&line.iter().map(|p|[p[0],p[1],p[2]]).collect::<Vec<_>>(),red,false)}for p in &cut.marks{overlay.mark(crate::clipping_geometry::array(p),6.,blue);}}overlay}
    pub fn changed(&self)->bool{self.cuts.iter().any(|c|!c.removed.is_empty())}
    pub fn prepared_edits(&self,scene:&Scene)->Result<Vec<(ObjectId,Vec<Geometry>)>,String>{self.cuts.iter().filter(|c|!c.removed.is_empty()).map(|c|{c.owner.valid(scene)?;Ok((c.owner.id,crate::trim_parts::kept(&c.owner.source,&c.parts,&c.removed)?))}).collect()}
}
```

### 5. `src/measure_gpu.rs`

Keep the packing origin beside the retained precise measurement. Begin with the earlier zero-origin measurement cache. Report actual measurement changes while retaining existing zero-origin callers. Observe origin changes even when precise measurement points are unchanged. Pack both precise measurement endpoints in the declared frame. Publish the cache witness only after successful derived-ink adoption.

Replace the entire file with:

```rust
use crate::{measure_mark::Mark, stroke_gpu::Lane};
pub struct Measurement {
    pub mark: Option<Mark>,
    anchor:[f64;3],
    strokes: Lane,
}
impl Measurement {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            mark: None,anchor:[0.;3],
            strokes: Lane::overlay(device, format),
        }
    }
    pub fn set(&mut self,device:&wgpu::Device,mark:Option<&Mark>)->Result<(),&'static str>{self.set_at(device,mark,[0.;3]).map(|_|())}
    pub fn set_at(&mut self,device:&wgpu::Device,mark:Option<&Mark>,anchor:[f64;3])->Result<bool,&'static str>{
        if self.mark.as_ref() == mark&&self.anchor==anchor {
            return Ok(false);
        }
        let mut strokes = Vec::new();
        if let Some(mark) = mark {
            for p in mark.points.windows(2) {
                if p[0] != p[1] {
                    strokes.push(crate::stroke::Stroke {
                        start: crate::render_space::relative(p[0],anchor),
                        end: crate::render_space::relative(p[1],anchor),
                        width: 1.5,
                        colour: [20. / 255., 20. / 255., 20. / 255., 1.],
                    });
                }
            }
        }
        self.strokes.set(device, strokes)?;
        self.mark = mark.cloned();self.anchor=anchor;
        Ok(true)
    }
    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        self.strokes.view(queue, transform, size, density)
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass)
    }
    pub fn bytes(&self) -> u64 {
        self.strokes.usage()[1]
    }
}
```

### 6. `src/renderer.rs`

Use the renderer origin for temporary draft geometry. Refresh measurement labels only after source or frame changes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{if self.measurement.mark.as_ref()==mark{return Ok(())}self.measurement.set(&self.device,mark)?;self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
```

Replace it with:

```rust

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{let anchor=self.anchor();if self.measurement.set_at(&self.device,mark,anchor)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")?;}Ok(())}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self,draft:Option<&crate::tool::Draft>)->Result<(),&'static str>{let anchor=self.anchor();self.preview.set_at(&self.device,draft,anchor)}
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
```

### 7. `src/browser.rs`

Collect browser guide coordinates using the renderer’s current origin.

**Locate this existing block:**

```rust
        if let Err(error)=renderer.set_guides(&editor.guide_overlay()){panel.result(error);}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let guides=editor.guide_overlay_at(renderer.anchor());
        if let Err(error)=renderer.set_guides(&guides){panel.result(error);}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/global_outline_tests.rs`

Supply actual local/distant temporary ink and glyph pixels, origin-only invalidation of unchanged measurements, exact restoration, no invented pick ownership and complete temporary GPU release.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera::Camera::default();far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzd-prepare-local-temporary-overlays-22.rs" skip=48325
```

## <a id="checkpoint-96zzzzze-preserve-local-precision-before-source-placement"></a>7. Preserve local precision before source placement

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/pick_sour…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original placement − scen…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">transform original local…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">narrow once</text>
</svg>
</div>

**What this code does:** Avoid a large world-coordinate intermediate when packing placed source data.

### 1. `src/scene.rs`

Retain world endpoints and expose one precise local endpoint path. Subtract the placement origin before adding original local endpoint coordinates. Narrow already-local precise endpoints once. Use one precise path position owner before GPU duplicate filtering. Reuse retained precise samples through an anchor-relative placement instead of a rounded world intermediate. Keep world point callers while sharing a precise local point transform. Narrow original point markers after their local transform. Make the collected control coordinate frame explicit. Transform controls through the precise local placement. Collect original markers in the declared local frame. Keep original net links in the same local frame. Collected control positions are already local; narrow without subtracting twice. Pack local control links without a second origin subtraction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn control_nets(&self)->Vec<(ObjectId,crate::stroke::Stroke)> {self.control_nets_at([0.;3])}
    pub fn control_nets_at(&self,anchor:[f64;3])->Vec<(ObjectId,crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
                let Some((a, b)) = controls.points.get(a).zip(controls.points.get(b)) else { continue };
                let start = crate::render_space::relative(a.position,anchor);
                let end = crate::render_space::relative(b.position,anchor);
                if start.iter().chain(&end).any(|v| !v.is_finite()) { continue; }
                nets.push((owner, crate::stroke::Stroke { start, end, colour: [102./255., 136./255., 204./255., 1.], width: 1. }));
            }
```

Replace it with:

```rust
    pub fn control_nets(&self)->Vec<(ObjectId,crate::stroke::Stroke)> {self.control_nets_at([0.;3])}
    pub fn control_nets_at(&self,anchor:[f64;3])->Vec<(ObjectId,crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources_at(anchor) {
            for [a, b] in controls.links {
                let Some((a, b)) = controls.points.get(a).zip(controls.points.get(b)) else { continue };
                let start=a.position.map(|v|v as f32);
                let end=b.position.map(|v|v as f32);
                if start.iter().chain(&end).any(|v| !v.is_finite()) { continue; }
                nets.push((owner, crate::stroke::Stroke { start, end, colour: [102./255., 136./255., 204./255., 1.], width: 1. }));
            }
```

**Locate this block:**

```rust
    pub fn control_markers(&self)->Vec<ControlMarker> {self.control_markers_at([0.;3])}
    pub fn control_markers_at(&self,anchor:[f64;3])->Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = crate::render_space::relative(control.position,anchor);
                if center.iter().any(|v| !v.is_finite()) { continue; }
                let part = match (legacy, control.id) {
                    (true, crate::source_controls::ControlId::Curve { point, .. }) => crate::pick_sources::Part::Control(point),
```

Replace it with:

```rust
    pub fn control_markers(&self)->Vec<ControlMarker> {self.control_markers_at([0.;3])}
    pub fn control_markers_at(&self,anchor:[f64;3])->Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources_at(anchor) {
            for control in controls.points {
                let center=control.position.map(|v|v as f32);
                if center.iter().any(|v| !v.is_finite()) { continue; }
                let part = match (legacy, control.id) {
                    (true, crate::source_controls::ControlId::Curve { point, .. }) => crate::pick_sources::Part::Control(point),
```

**Locate this block:**

```rust
        for row in &mut self.paths { row.controls_visible = visible && matches!(row.prepared.source, crate::chain::Source::Curve(..)); }
    }

    fn control_sources(&self) -> Vec<(ObjectId, bool, crate::source_controls::Controls)> {
        let curves = self.paths.iter().filter(|row| row.controls_visible).map(|row| (row.id, true));
        let originals = self.control_owners.iter().copied().filter(|id| !self.paths.iter().any(|r|r.id==*id && r.controls_visible)).map(|id| (id, false));
        curves.chain(originals).filter(|(id, _)| self.visible(*id)).filter_map(|(id, legacy)| {
            let source = crate::source_edit::original(self, id)?;
            let model = self.placement(id)?;
            let mut controls = crate::source_controls::Controls::from_geometry(&source);
            for control in &mut controls.points {
                let p = model.transform_point(&session_rust::Point::new(control.position[0], control.position[1], control.position[2]));
```

Replace it with:

```rust
        for row in &mut self.paths { row.controls_visible = visible && matches!(row.prepared.source, crate::chain::Source::Curve(..)); }
    }

    fn control_sources_at(&self,anchor:[f64;3]) -> Vec<(ObjectId, bool, crate::source_controls::Controls)> {
        let curves = self.paths.iter().filter(|row| row.controls_visible).map(|row| (row.id, true));
        let originals = self.control_owners.iter().copied().filter(|id| !self.paths.iter().any(|r|r.id==*id && r.controls_visible)).map(|id| (id, false));
        curves.chain(originals).filter(|(id, _)| self.visible(*id)).filter_map(|(id, legacy)| {
            let source = crate::source_edit::original(self, id)?;
            let model=crate::render_space::placed(&self.placement(id)?,anchor);
            let mut controls = crate::source_controls::Controls::from_geometry(&source);
            for control in &mut controls.points {
                let p = model.transform_point(&session_rust::Point::new(control.position[0], control.position[1], control.position[2]));
```

**Locate this block:**

```rust
impl PointObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self)->crate::marker::Marker {self.marker_at([0.;3])}
    pub fn marker_at(&self,anchor:[f64;3])->crate::marker::Marker {crate::marker::Marker {center:crate::render_space::relative(crate::clipping_geometry::array(&self.point()),anchor),..self.prepared.display}}
}

#[derive(Clone)]
```

Replace it with:

```rust
impl PointObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self)->session_rust::Point {self.point_at([0.;3])}
    pub fn point_at(&self,anchor:[f64;3])->session_rust::Point {crate::render_space::placed(&self.model,anchor).transform_point(&self.prepared.source)}
    pub fn marker(&self)->crate::marker::Marker {self.marker_at([0.;3])}
    pub fn marker_at(&self,anchor:[f64;3])->crate::marker::Marker {crate::marker::Marker {center:self.point_at(anchor).to_f32(),..self.prepared.display}}
}

#[derive(Clone)]
```

**Locate this block:**

```rust
            self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.coordinates().into_iter().map(|p| self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }
```

Replace it with:

```rust
            self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

    pub fn points(&self)->Vec<session_rust::Point> {self.points_at([0.;3])}
    pub fn points_at(&self,anchor:[f64;3])->Vec<session_rust::Point> {
        let model=crate::render_space::placed(&self.model,anchor);
        self.prepared.source.coordinates().into_iter().map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]))).collect()
    }
```

**Locate this block:**

```rust
    pub fn segments(&self)->Vec<crate::chain::Segment> {self.segments_at([0.;3])}
    pub fn segments_at(&self,anchor:[f64;3])->Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = crate::render_space::relative(crate::clipping_geometry::array(&p),anchor);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
```

Replace it with:

```rust
    pub fn segments(&self)->Vec<crate::chain::Segment> {self.segments_at([0.;3])}
    pub fn segments_at(&self,anchor:[f64;3])->Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points_at(anchor) {
            let p=p.to_f32();
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
```

**Locate this block:**

```rust
impl LineObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self)->crate::stroke::Stroke {self.stroke_at([0.;3])}
    pub fn stroke_at(&self,anchor:[f64;3])->crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: crate::render_space::relative(crate::clipping_geometry::array(&start),anchor),
            end: crate::render_space::relative(crate::clipping_geometry::array(&end),anchor), ..self.prepared.display }
    }
}
```

Replace it with:

```rust
impl LineObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn endpoints(&self)->[session_rust::Point;2] {self.endpoints_at([0.;3])}
    pub fn endpoints_at(&self,anchor:[f64;3])->[session_rust::Point;2] {
        let line = &self.prepared.source;let model=crate::render_space::placed(&self.model,anchor);
        [model.transform_point(&line.start()),model.transform_point(&line.end())]
    }

    pub fn stroke(&self)->crate::stroke::Stroke {self.stroke_at([0.;3])}
    pub fn stroke_at(&self,anchor:[f64;3])->crate::stroke::Stroke {
        let [start,end]=self.endpoints_at(anchor);
        crate::stroke::Stroke {start:start.to_f32(),end:end.to_f32(), ..self.prepared.display }
    }
}
```

### 2. `src/pick_sources.rs`

Use one precise local placement for each original boundary owner. Avoid a large intermediate world coordinate for original mesh/CAD/surface pick ink. Keep compact pick addresses on the exact same local sample path as drawn ink.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points().iter().map(|p|crate::render_space::relative(crate::clipping_geometry::array(p),anchor)).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
```

Replace it with:

```rust
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
```

**Locate this block:**

```rust
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends=ends.map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));crate::render_space::relative(crate::clipping_geometry::array(&p),anchor)});
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
```

Replace it with:

```rust
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
```

**Locate this block:**

```rust
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
```

Replace it with:

```rust
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
```

### 3. `src/cloud_pick.rs`

Retain original world control positions and prepare an explicit local lookup. Apply the local placement for streamed original cloud IDs. Use the same placement for resident original cloud IDs.

Replace the entire file with:

```rust
use crate::{pick_sources::{Part,Target},scene::{ObjectId,Scene}};

// Storage indices address current GPU columns; original IDs survive octree reordering and may use every bit of a u32.
pub fn resolve(scene:&Scene,owner:ObjectId,index:usize)->Option<Target> {
    if let Some(row)=scene.streams().iter().find(|r|r.id==owner&&r.controls_visible) {
        scene.pick_id(owner.value())?;let mut address=0usize;
        for page in row.prepared.pages() {let count=page.value.ids.len();if index>=address&&index-address<count {return Some(Target {owner,part:Part::CloudPoint(page.value.ids[index-address])});}address=address.checked_add(count)?;}return None;
    }
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    if scene.pick_id(owner.value()).is_none() || index>=row.prepared.source.point_count() {return None;}
    let target=Target {owner,part:Part::CloudPoint(row.prepared.source.point_id(index))};
    Some(target)
}

pub fn position(scene:&Scene,owner:ObjectId,id:u32)->Option<session_rust::Point> {position_at(scene,owner,id,[0.;3])}
pub fn position_at(scene:&Scene,owner:ObjectId,id:u32,anchor:[f64;3])->Option<session_rust::Point> {
    if let Some(row)=scene.streams().iter().find(|r|r.id==owner&&r.controls_visible) {
        scene.pick_id(owner.value())?;let p=row.prepared.position(id)?;return Some(crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])));
    }
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    scene.pick_id(owner.value())?;
    let index=row.prepared.source.index_of_id(id)?;
    Some(crate::render_space::placed(&row.model,anchor).transform_point(&row.prepared.source.get_point(index)))
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/scene_origin_tests.rs`

Supply exact nonbinary local line/path/point/control/cloud and pick-table data through large original placements without changing source IDs.

**Locate this existing block:**

```text
use std::rc::Rc;
#[test]
fn placed_original_ink_keeps_fractional_positions_controls_and_sources(){
 let mut scene=crate::scene::Scene::empty();let line=Rc::new(session_rust::Line::new(-0.25,0.125,0.,0.25,0.125,0.));let line_id=scene.insert_line(crate::stroke::PreparedLine::new(line.clone()).unwrap()).unwrap();let polyline=Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(0.,-0.25,0.),session_rust::Point::new(0.125,0.25,0.),session_rust::Point::new(0.375,-0.25,0.)]));let path_id=scene.insert_path(crate::chain::PreparedChain::polyline(polyline.clone()).unwrap()).unwrap();let point=Rc::new(session_rust::Point::new(0.25,0.375,0.));let point_id=scene.insert_point(crate::marker::PreparedPoint::new(point.clone()).unwrap()).unwrap();scene.owner_controls(path_id).unwrap();
 let line_bytes=scene.lines()[0].stroke().bytes();let path_bytes=scene.paths()[0].segments().iter().flat_map(|s|s.bytes()).collect::<Vec<_>>();let marker=scene.points()[0].marker().values();let controls=scene.control_markers().iter().map(|c|(c.parent,c.part,c.display.values())).collect::<Vec<_>>();let nets=scene.control_nets();assert!(!controls.is_empty());assert!(!nets.is_empty());
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [line_id,path_id,point_id]{far.translate(id,anchor).unwrap();}let collapsed=far.lines()[0].stroke();assert_eq!(collapsed.start,collapsed.end);assert_eq!(far.lines()[0].stroke_at(anchor).bytes(),line_bytes);assert_eq!(far.paths()[0].segments_at(anchor).iter().flat_map(|s|s.bytes()).collect::<Vec<_>>(),path_bytes);assert_eq!(far.points()[0].marker_at(anchor).values(),marker);assert_eq!(far.control_markers_at(anchor).iter().map(|c|(c.parent,c.part,c.display.values())).collect::<Vec<_>>(),controls);assert_eq!(far.control_nets_at(anchor),nets);
 assert!(Rc::ptr_eq(&far.lines()[0].prepared.source,&line));assert!(matches!(&far.paths()[0].prepared.source,crate::chain::Source::Polyline(source) if Rc::ptr_eq(source,&polyline)));assert!(Rc::ptr_eq(&far.points()[0].prepared.source,&point));assert!(Rc::ptr_eq(&far.paths()[0].prepared.points,&scene.paths()[0].prepared.points));assert_eq!(line.start()[0],-0.25);assert_eq!(polyline.coords[3],0.125);assert_eq!(point[0],0.25);
}

#[test]
fn anchored_source_tables_keep_original_edge_and_control_addresses(){
 let mut scene=crate::scene::Scene::empty();let mut ids=Vec::new();ids.push(scene.insert(crate::prepared::PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap()).unwrap());ids.push(scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(0.,0.,0.,0.25,0.,0.))).unwrap()).unwrap());ids.push(scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(0.,0.,0.),session_rust::Point::new(0.125,0.25,0.),session_rust::Point::new(0.25,0.,0.)]))).unwrap()).unwrap());ids.push(scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.125,0.25,0.))).unwrap()).unwrap());scene.owner_controls(ids[0]).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let baseline=crate::pick_sources::Sources::new(&scene).unwrap();assert!(!baseline.faces.is_empty());assert!(!baseline.controls.is_empty());assert_eq!(baseline.path_ids.len(),2);
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in ids{far.translate(id,anchor).unwrap();}assert!(crate::pick_sources::Sources::new(&far).unwrap().path_ids.len()<baseline.path_ids.len());let current=crate::pick_sources::Sources::anchored(&far,anchor).unwrap();assert_eq!(current.faces,baseline.faces);assert_eq!(current.edges,baseline.edges);assert_eq!(current.controls,baseline.controls);assert_eq!(current.mesh_bases,baseline.mesh_bases);assert_eq!(current.mesh_ink,baseline.mesh_ink);assert_eq!(current.mesh_ids,baseline.mesh_ids);assert_eq!(current.line_ids,baseline.line_ids);assert_eq!(current.path_ids,baseline.path_ids);assert_eq!(current.marker_ids,baseline.marker_ids);assert_eq!(current.sheet_bases,baseline.sheet_bases);assert_eq!(current.path_ids.len(),far.paths().iter().flat_map(|row|row.segments_at(anchor)).count());assert_eq!(current.marker_ids.len(),far.points().len()+far.control_markers_at(anchor).len());assert_eq!(current.mesh_ids.len(),current.mesh_ink.len());assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),original);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzze-preserve-local-precision-before-source-placement-20.rs" skip=4340
```

## <a id="checkpoint-96zzzzzf-prepare-precise-construction-grid-origins"></a>8. Prepare precise construction-grid origins

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cplane.rs, src/grid.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Construction plane − scen…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local grid vertices</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">GPU cache witness</text>
</svg>
</div>

**What this code does:** Pack a construction grid in the same precise local frame as original geometry.

### 1. `src/cplane.rs`

Retain one construction-plane matrix recipe in double precision; existing GPU callers narrow at their boundary.

**Locate this existing block:**

```rust
    pub fn matrix(self) -> [f32; 16] {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let o = self.origin();
        [
            x[0] as f32,
            x[1] as f32,
            x[2] as f32,
            0.0,
            y[0] as f32,
            y[1] as f32,
            y[2] as f32,
            0.0,
            z[0] as f32,
            z[1] as f32,
            z[2] as f32,
            0.0,
            o[0] as f32,
            o[1] as f32,
            o[2] as f32,
            1.0,
        ]
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn matrix(self)->[f32;16] {self.matrix64().to_f32()}
    pub fn matrix64(self)->session_rust::Xform {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let o = self.origin();
        session_rust::Xform::from_matrix([
            x[0],
            x[1],
            x[2],
            0.0,
            y[0],
            y[1],
            y[2],
            0.0,
            z[0],
            z[1],
            z[2],
            0.0,
            o[0],
            o[1],
            o[2],
            1.0,
        ])
    }
```

### 2. `src/grid.rs`

Reuse grid generation in a precise declared frame. Apply local grid positions through the origin-relative precise plane matrix before narrowing.

Replace the entire file with:

```rust
use crate::stroke::Stroke;

pub const AXIS_COLOURS: [[f32; 4]; 3] = [
    [0.910, 0.278, 0.545, 1.0], [0.604, 0.804, 0.196, 1.0], [0.129, 0.588, 0.918, 1.0],
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings { pub spacing: f64, pub half_cells: u32 }

impl Default for Settings {
    fn default() -> Self { Self { spacing: 1.0, half_cells: 5 } }
}

impl Settings {
    pub fn half(self) -> Result<f64, &'static str> {
        let half = self.spacing * self.half_cells as f64;
        if !self.spacing.is_finite() || self.spacing <= 0.0 || self.half_cells == 0 || self.half_cells > 500
            || !(half as f32).is_finite() || self.spacing as f32 == 0.0 { return Err("Grid spacing or extent is invalid"); }
        Ok(half)
    }

    pub fn points(self) -> Result<Vec<[f64; 3]>, &'static str> {
        let h = self.half()?;
        Ok(vec![[-h, -h, 0.0], [-h, h, 0.0], [h, -h, 0.0], [h, h, 0.0],
            [h, 0.0, 0.0], [0.0, h, 0.0], [0.0, 0.0, self.spacing]])
    }

    pub fn reach(self,target:[f64;3])->Result<f64,&'static str>{self.reach_on(crate::cplane::CPlane::Xy,target)}
    pub fn reach_on(self,plane:crate::cplane::CPlane,target:[f64;3])->Result<f64,&'static str>{
        if target.iter().any(|v|!v.is_finite()){return Err("Grid target must be finite")}
        let reach=self.points()?.into_iter().map(|p|{let p=plane.world(p);(p[0]-target[0]).hypot(p[1]-target[1]).hypot(p[2]-target[2])}).fold(0.,f64::max);
        if !reach.is_finite(){return Err("Grid target exceeds the depth range")}Ok(reach)
    }
    pub fn strokes_on(self,plane:crate::cplane::CPlane)->Result<Vec<Stroke>,&'static str>{self.strokes_on_at(plane,[0.;3])}
    pub fn strokes_on_at(self,plane:crate::cplane::CPlane,anchor:[f64;3])->Result<Vec<Stroke>,&'static str>{
        let mut strokes=self.strokes()?;let model=crate::render_space::placed(&plane.matrix64(),anchor);
        for stroke in &mut strokes{for p in [&mut stroke.start,&mut stroke.end]{let values=model.transform_point(&session_rust::Point::new(f64::from(p[0]),f64::from(p[1]),f64::from(p[2]))).to_f32();if values.iter().any(|v|!v.is_finite()){return Err("Grid plane exceeds display range")}*p=values;}}
        Ok(strokes)
    }

    pub fn strokes(self) -> Result<Vec<Stroke>, &'static str> {
        let half = self.half()? as f32; let n = self.half_cells as i32; let mut strokes = Vec::new();
        for k in -n..=n {
            let t = (k as f64 * self.spacing) as f32;
            for (start, end) in [([-half, t, 0.0], [half, t, 0.0]), ([t, -half, 0.0], [t, half, 0.0])] {
                strokes.push(Stroke { start, end, colour: [0.55, 0.55, 0.55, 1.0], width: 1.0 });
            }
        }
        for (end, colour) in [[half, 0.0, 0.0], [0.0, half, 0.0], [0.0, 0.0, self.spacing as f32]].into_iter().zip(AXIS_COLOURS) {
            strokes.push(Stroke { start: [0.0; 3], end, colour, width: 2.0 });
        }
        Ok(strokes)
    }
}
```

### 3. `src/renderer.rs`

Retain the origin used by grid vertices. Begin with the existing grid frame. Include origin changes when deciding whether cached grid data is current. Pack the grid through the current renderer origin. Publish the complete grid witness after successful adoption.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        if settings==self.grid_settings&&plane==self.grid_plane{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on(plane)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
```

Replace it with:

```rust

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        let anchor=self.anchor();
        if settings==self.grid_settings&&plane==self.grid_plane&&anchor==self.grid_anchor{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on_at(plane,anchor)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;self.grid_anchor=anchor;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
```

**Locate this block:**

```rust
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
```

Replace it with:

```rust
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
```

**Locate this block:**

```rust
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
```

Replace it with:

```rust
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,grid_anchor:[f64;3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/scene_origin_tests.rs`

Supply an exact nonbinary grid comparison under a large independently placed construction plane.

**Locate this existing block:**

```text
use std::rc::Rc;
#[test]
fn placed_original_ink_keeps_fractional_positions_controls_and_sources(){
 let mut scene=crate::scene::Scene::empty();let line=Rc::new(session_rust::Line::new(-0.25,0.125,0.,0.25,0.125,0.));let line_id=scene.insert_line(crate::stroke::PreparedLine::new(line.clone()).unwrap()).unwrap();let polyline=Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(0.,-0.25,0.),session_rust::Point::new(0.125,0.25,0.),session_rust::Point::new(0.375,-0.25,0.)]));let path_id=scene.insert_path(crate::chain::PreparedChain::polyline(polyline.clone()).unwrap()).unwrap();let point=Rc::new(session_rust::Point::new(0.25,0.375,0.));let point_id=scene.insert_point(crate::marker::PreparedPoint::new(point.clone()).unwrap()).unwrap();scene.owner_controls(path_id).unwrap();
 let line_bytes=scene.lines()[0].stroke().bytes();let path_bytes=scene.paths()[0].segments().iter().flat_map(|s|s.bytes()).collect::<Vec<_>>();let marker=scene.points()[0].marker().values();let controls=scene.control_markers().iter().map(|c|(c.parent,c.part,c.display.values())).collect::<Vec<_>>();let nets=scene.control_nets();assert!(!controls.is_empty());assert!(!nets.is_empty());
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [line_id,path_id,point_id]{far.translate(id,anchor).unwrap();}let collapsed=far.lines()[0].stroke();assert_eq!(collapsed.start,collapsed.end);assert_eq!(far.lines()[0].stroke_at(anchor).bytes(),line_bytes);assert_eq!(far.paths()[0].segments_at(anchor).iter().flat_map(|s|s.bytes()).collect::<Vec<_>>(),path_bytes);assert_eq!(far.points()[0].marker_at(anchor).values(),marker);assert_eq!(far.control_markers_at(anchor).iter().map(|c|(c.parent,c.part,c.display.values())).collect::<Vec<_>>(),controls);assert_eq!(far.control_nets_at(anchor),nets);
 assert!(Rc::ptr_eq(&far.lines()[0].prepared.source,&line));assert!(matches!(&far.paths()[0].prepared.source,crate::chain::Source::Polyline(source) if Rc::ptr_eq(source,&polyline)));assert!(Rc::ptr_eq(&far.points()[0].prepared.source,&point));assert!(Rc::ptr_eq(&far.paths()[0].prepared.points,&scene.paths()[0].prepared.points));assert_eq!(line.start()[0],-0.25);assert_eq!(polyline.coords[3],0.125);assert_eq!(point[0],0.25);
}

#[test]
fn anchored_source_tables_keep_original_edge_and_control_addresses(){
 let mut scene=crate::scene::Scene::empty();let mut ids=Vec::new();ids.push(scene.insert(crate::prepared::PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap()).unwrap());ids.push(scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(0.,0.,0.,0.25,0.,0.))).unwrap()).unwrap());ids.push(scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(0.,0.,0.),session_rust::Point::new(0.125,0.25,0.),session_rust::Point::new(0.25,0.,0.)]))).unwrap()).unwrap());ids.push(scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.125,0.25,0.))).unwrap()).unwrap());scene.owner_controls(ids[0]).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let baseline=crate::pick_sources::Sources::new(&scene).unwrap();assert!(!baseline.faces.is_empty());assert!(!baseline.controls.is_empty());assert_eq!(baseline.path_ids.len(),2);
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in ids{far.translate(id,anchor).unwrap();}assert!(crate::pick_sources::Sources::new(&far).unwrap().path_ids.len()<baseline.path_ids.len());let current=crate::pick_sources::Sources::anchored(&far,anchor).unwrap();assert_eq!(current.faces,baseline.faces);assert_eq!(current.edges,baseline.edges);assert_eq!(current.controls,baseline.controls);assert_eq!(current.mesh_bases,baseline.mesh_bases);assert_eq!(current.mesh_ink,baseline.mesh_ink);assert_eq!(current.mesh_ids,baseline.mesh_ids);assert_eq!(current.line_ids,baseline.line_ids);assert_eq!(current.path_ids,baseline.path_ids);assert_eq!(current.marker_ids,baseline.marker_ids);assert_eq!(current.sheet_bases,baseline.sheet_bases);assert_eq!(current.path_ids.len(),far.paths().iter().flat_map(|row|row.segments_at(anchor)).count());assert_eq!(current.marker_ids.len(),far.points().len()+far.control_markers_at(anchor).len());assert_eq!(current.mesh_ids.len(),current.mesh_ink.len());assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn nonbinary_local_sources_and_pick_tables_survive_large_placements(){
 let mut scene=crate::scene::Scene::empty();let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[0.1,0.2,-0.5],[0.3,0.2,-0.5],[0.2,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(0.1,0.2,-0.5,0.3,0.4,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(0.1,0.2,-0.5),session_rust::Point::new(0.3,0.4,-0.5),session_rust::Point::new(0.5,0.2,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.1,0.2,-0.5))).unwrap()).unwrap();let mut proto=session_rust::PointCloud::from_coords(vec![0.1,0.2,-0.5],vec![],vec![]).to_proto();proto.point_ids=vec![u32::MAX];let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_proto(proto))).unwrap()).unwrap();scene.owner_controls(mesh).unwrap();scene.cloud_controls(Some(cloud));let baseline=crate::pick_sources::Sources::new(&scene).unwrap();let controls=scene.control_markers().iter().map(|c|(c.parent,c.part,c.display.values())).collect::<Vec<_>>();let nets=scene.control_nets();let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [mesh,line,path,point,cloud]{far.translate(id,anchor).unwrap();}
 assert_eq!(far.lines()[0].stroke_at(anchor).bytes(),scene.lines()[0].stroke().bytes());assert_eq!(far.paths()[0].segments_at(anchor).iter().flat_map(|s|s.bytes()).collect::<Vec<_>>(),scene.paths()[0].segments().iter().flat_map(|s|s.bytes()).collect::<Vec<_>>());assert_eq!(far.points()[0].marker_at(anchor).values(),scene.points()[0].marker().values());assert_eq!(far.control_markers_at(anchor).iter().map(|c|(c.parent,c.part,c.display.values())).collect::<Vec<_>>(),controls);assert_eq!(far.control_nets_at(anchor),nets);let current=crate::pick_sources::Sources::anchored(&far,anchor).unwrap();assert_eq!(current.mesh_ink,baseline.mesh_ink);assert_eq!(current.mesh_ids,baseline.mesh_ids);assert_eq!(current.path_ids,baseline.path_ids);assert_eq!(current.controls,baseline.controls);assert_eq!(crate::cloud_pick::position_at(&far,cloud,u32::MAX,anchor).unwrap().to_f32(),[0.1f32,0.2,-0.5]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzf-prepare-precise-construction-grid-origins-09.rs" skip=6676
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Supply actual grid/surface pixels, unchanged source geometry uploads, origin-only cache invalidation under an identical local matrix, exact restoration, original picks and empty release.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera::Camera::default();far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}

fn temporary_inputs(gpu:&mut super::Renderer,world:[f64;3]){
 let point=|p:[f64;3]|std::array::from_fn(|i|p[i]+world[i]);let mut guides=crate::guide_gpu::Overlay::at(gpu.anchor());guides.wire(&[point([-0.75,0.25,-0.5]),point([0.75,0.25,-0.5])],[210./255.,40./255.,40./255.,1.],true);guides.mark(point([0.5,0.25,-0.5]),6.,[0.,0.,1.,1.]);guides.label=Some((point([0.5,0.25,-0.5]),"guide".into()));gpu.set_guides(&guides).unwrap();let mark=crate::measure_mark::Mark{points:vec![point([-0.5,-0.25,-0.5]),point([0.5,-0.25,-0.5])],label:"distance".into()};gpu.set_measurement(Some(&mark)).unwrap();let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.submit(point([0.,0.,-0.5])).unwrap();draft.hover(Some(point([0.5,0.,-0.5]))).unwrap();gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release(){
 let scene=Scene::empty();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();temporary_inputs(&mut gpu,[0.;3]);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]>200&&p[2]<30));assert!(baseline.chunks_exact(4).any(|p|p[0]>150&&p[1]<80&&p[2]<80));assert!(baseline.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>200);let bytes=(gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes());assert!(bytes.0>0&&bytes.1>0&&bytes.2>0);assert_eq!(gpu.text_error(),None);
 let anchor=[1e9,-1e9,1e9];assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);camera.target=anchor;temporary_inputs(&mut gpu,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes()),bytes);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert!(scene.all_ids().is_empty());gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap(),None);
 gpu.set_preview(None).unwrap();gpu.set_measurement(None).unwrap();gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor)).unwrap();crate::cap_gpu_tests::pixels_at(&gpu,local);assert_eq!(gpu.preview_bytes(),0);assert_eq!(gpu.guide_bytes(),0);assert_eq!(gpu.measurement.bytes(),0);assert_eq!(gpu.text_usage()[0],0);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzf-prepare-precise-construction-grid-origins-10.rs" skip=51216
```

## <a id="checkpoint-96zzzzzg-initialize-object-records-in-the-local-frame"></a>9. Initialize object records in the local frame

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_mesh.rs, src/global…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original placement + curr…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing record encoder</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">first draw</text>
</svg>
</div>

**What this code does:** Create precise records before their first GPU draw.

### 1. `src/gpu_mesh.rs`

Initialize derived geometry records in their actual frame; retain zero-origin callers. Keep a single record initialization path for both shared geometry and derived faces. Retain the frame used by the initialized record.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { translation:object.model.m[12..15].try_into().unwrap(),anchor:[0.;3],geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
```

Replace it with:

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
```

**Locate this block:**

```rust

impl GpuMesh {
    pub fn upload(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, object: &Object, selected: bool) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(device, Rc::clone(&object.mesh)));
        Self::with_geometry(device, layout, object, Style {selected,pickable:!object.metadata.is_locked,colour:None}, geometry)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style,0,0,0,geometry.index_start,[0.;3]);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

Replace it with:

```rust

impl GpuMesh {
    pub fn upload(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, object: &Object, selected: bool) -> Self {
        Self::upload_at(device,layout,object,selected,[0.;3])
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{selected,pickable:!object.metadata.is_locked,colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        Self::with_geometry_at(device,layout,object,style,geometry,[0.;3])
    }
    pub fn with_geometry_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,style:Style,geometry:Rc<GpuGeometry>,anchor:[f64;3])->Self {
        let bytes=settings(object,style,0,0,0,geometry.index_start,anchor);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/global_outline_tests.rs`

Supply actual initial local/distant records and pixels, subsequent exact settings writes, shared geometry identity and unchanged sources.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera::Camera::default();far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}

fn temporary_inputs(gpu:&mut super::Renderer,world:[f64;3]){
 let point=|p:[f64;3]|std::array::from_fn(|i|p[i]+world[i]);let mut guides=crate::guide_gpu::Overlay::at(gpu.anchor());guides.wire(&[point([-0.75,0.25,-0.5]),point([0.75,0.25,-0.5])],[210./255.,40./255.,40./255.,1.],true);guides.mark(point([0.5,0.25,-0.5]),6.,[0.,0.,1.,1.]);guides.label=Some((point([0.5,0.25,-0.5]),"guide".into()));gpu.set_guides(&guides).unwrap();let mark=crate::measure_mark::Mark{points:vec![point([-0.5,-0.25,-0.5]),point([0.5,-0.25,-0.5])],label:"distance".into()};gpu.set_measurement(Some(&mark)).unwrap();let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.submit(point([0.,0.,-0.5])).unwrap();draft.hover(Some(point([0.5,0.,-0.5]))).unwrap();gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release(){
 let scene=Scene::empty();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();temporary_inputs(&mut gpu,[0.;3]);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]>200&&p[2]<30));assert!(baseline.chunks_exact(4).any(|p|p[0]>150&&p[1]<80&&p[2]<80));assert!(baseline.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>200);let bytes=(gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes());assert!(bytes.0>0&&bytes.1>0&&bytes.2>0);assert_eq!(gpu.text_error(),None);
 let anchor=[1e9,-1e9,1e9];assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);camera.target=anchor;temporary_inputs(&mut gpu,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes()),bytes);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert!(scene.all_ids().is_empty());gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap(),None);
 gpu.set_preview(None).unwrap();gpu.set_measurement(None).unwrap();gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor)).unwrap();crate::cap_gpu_tests::pixels_at(&gpu,local);assert_eq!(gpu.preview_bytes(),0);assert_eq!(gpu.guide_bytes(),0);assert_eq!(gpu.measurement.bytes(),0);assert_eq!(gpu.text_usage()[0],0);
}

#[test]
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let settings=Some(crate::grid::Settings{spacing:0.1,half_cells:5});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.space.update(anchor,2.,0.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let far_plane=plane([anchor[0],anchor[1],anchor[2]-0.75]);gpu.set_grid_on(settings,far_plane).unwrap();camera.target=anchor;let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);let grid_uploads=gpu.grid_usage()[2];gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(gpu.grid_usage()[2],grid_uploads);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];gpu.space.update(moved,2.,300.).unwrap();camera.target=moved;for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,moved);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert!(gpu.grid_usage()[2]>grid_uploads);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));gpu.space.update(anchor,2.,600.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_grid(None).unwrap();gpu.set_scene(&Scene::empty(),None);assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzg-initialize-object-records-in-the-local-frame-04.rs" skip=54271
```

## <a id="checkpoint-96zzzzzh-prepare-anchor-relative-original-highlights"></a>10. Prepare highlights in the shared local frame

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/highlight.rs, src/rende…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original selection</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing local source acc…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived highlight lanes</text>
</svg>
</div>

**What this code does:** Keep original face, edge, curve and control highlights aligned after rebasing.

### 1. `src/highlight.rs`

Expose the origin once at highlight preparation. Place derived face records precisely before their first draw. Apply the existing precise local placement before transforming original boundary coordinates. Reuse the already taught precise local source accessor for highlight presentation. Share the declared origin with whole-object ink selection.

Replace the entire file with:

```rust
use crate::{gpu_mesh::GpuMesh, pick_sources::{Part, Target}, scene::Scene, stroke_gpu::Lane};
use std::rc::Rc;

pub struct Highlight { mesh: Option<GpuMesh>, strokes: Lane, paths: Lane, markers: Lane }

impl Highlight {
    pub fn set_opacity(&self,value:f32){for lane in [&self.strokes,&self.paths,&self.markers]{lane.set_opacity(value);}}

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self { mesh: None, strokes: Lane::new(device, format), paths: Lane::joined(device, format), markers: Lane::markers(device, format) }
    }

    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){for lane in [&mut self.strokes,&mut self.paths,&mut self.markers]{lane.enable_finite(device,format)}}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {self.set_at(device,layout,scene,target,[0.;3])}
    pub fn set_at(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,scene:&Scene,target:Option<Target>,anchor:[f64;3])->Result<(),&'static str>{
        self.mesh = None; let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = target.filter(|t| t.valid(scene)) { match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let face = row.mesh.parts.faces.iter().position(|&f| f == key).unwrap() as u32;
                // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                let mut vertices = Vec::new(); let mut normals = Vec::new(); let mut indices = Vec::new();
                for (tri, &source) in row.mesh.indices().chunks_exact(3).zip(&row.mesh.parts.triangles) { if source == face {
                    for &index in tri { indices.push(vertices.len() as u32); vertices.push(row.mesh.vertices()[index as usize]); normals.push(row.mesh.normals()[index as usize]); }
                } }
                let display = crate::mesh::Mesh::new(vertices, indices)?.with_normals(normals)?;
                let mut object = row.clone(); object.mesh = Rc::new(display);
                self.mesh = Some(GpuMesh::upload_at(device,layout,&object,true,anchor));
            }
            Part::BRepEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let points=&row.cad.as_ref().unwrap().edges.iter().find(|border|border.edge==edge).unwrap().points;
                for pair in points.windows(2) {let ends=[pair[0],pair[1]].map(|p|crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::SurfaceEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let surface=row.surface.as_ref().unwrap();let boundary=surface.grid.boundaries.iter().find(|b|b.edge==edge).unwrap();
                for ends in &boundary.segments {let ends=ends.map(|index|{let p=surface.grid.samples[index as usize].point;crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32()});strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::MeshEdge(a, b) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let index = row.mesh.parts.edges.iter().position(|&edge| edge == (a, b)).unwrap();
                let ends = row.mesh.parts.endpoints[index].map(|p| crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                strokes.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: yellow, width: 3. });
            }
            Part::PathEdge(index) => {
                let row = scene.paths().iter().find(|r| r.id == target.owner).unwrap(); let points=row.points_at(anchor);
                strokes.push(crate::stroke::Stroke { start: points[index].to_f32(), end: points[index + 1].to_f32(), colour: yellow, width: row.prepared.width + 2. });
            }
            Part::Curve => {
                if let Some(row) = scene.lines().iter().find(|r| r.id == target.owner) {
                    strokes.push(crate::stroke::Stroke { colour: yellow, width: row.prepared.display.width + 2., ..row.stroke_at(anchor) });
                } else if let Some(row) = scene.paths().iter().find(|r| r.id == target.owner) {
                    paths.extend(row.segments_at(anchor).into_iter().map(|mut segment| { segment.stroke.colour = yellow; segment.stroke.width += 2.; segment }));
                }
            }
            Part::Control(_) | Part::SourceControl(_) => {
                let marker = scene.control_markers_at(anchor).into_iter().find(|m| m.parent == target.owner && m.part == target.part).unwrap();
                markers.push(crate::marker::Marker { colour: [1., 1., 0., 1.], ..marker.display });
            }
            Part::CloudPoint(id)=>{
                let p=crate::cloud_pick::position_at(scene,target.owner,id,anchor).ok_or("Cloud point is missing")?;
                markers.push(crate::marker::Marker {center:p.to_f32(),diameter:11.,colour:yellow});
            }
            Part::SheetEntity(_)|Part::Object => {}
        } }
        let flag=target.filter(|t|t.valid(scene)).map_or(0,|t|if crate::clipping_rows::owners(scene).contains(&t.owner){4096}else{0});let count=strokes.len();self.strokes.set(device, strokes)?;self.strokes.set_flags(device,vec![flag;count])?; self.paths.set_chains(device, paths)?; self.markers.set_points(device, markers)?; Ok(())
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {self.objects_at(device,scene,ids,[0.;3])}
    pub fn objects_at(&mut self,device:&wgpu::Device,scene:&Scene,ids:&[crate::scene::ObjectId],anchor:[f64;3])->Result<(),&'static str>{
        self.mesh = None; let yellow = crate::colour::selection();
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke_at(anchor) }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|crate::render_space::placed(&r.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
        self.paths.set_chains(device, scene.paths().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).flat_map(|r|
            r.segments_at(anchor).into_iter().map(|mut s| { s.stroke.colour = yellow; s.stroke.width += 2.; s })))?;
        self.markers.set_points(device, scene.points().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::marker::Marker { colour: yellow, diameter: r.prepared.display.diameter + 4., ..r.marker_at(anchor) }))?;
        Ok(())
    }

    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] { lane.view(queue, transform, size, density); }
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) { self.strokes.draw(pass); self.paths.draw(pass); }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>) { self.markers.draw(pass); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
}
```

### 2. `src/renderer.rs`

Prepare original part selection in the renderer frame. Prepare whole-object ink selection in that same frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
```

Replace it with:

```rust
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects_at(&self.device,scene,ids,anchor)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
```

**Locate this block:**

```rust
    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
```

Replace it with:

```rust
    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply actual local/distant face/edge/curve/control/high-bit cloud and object-highlight pixel comparisons with exact clearing and unchanged sources.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera::Camera::default();far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}

fn temporary_inputs(gpu:&mut super::Renderer,world:[f64;3]){
 let point=|p:[f64;3]|std::array::from_fn(|i|p[i]+world[i]);let mut guides=crate::guide_gpu::Overlay::at(gpu.anchor());guides.wire(&[point([-0.75,0.25,-0.5]),point([0.75,0.25,-0.5])],[210./255.,40./255.,40./255.,1.],true);guides.mark(point([0.5,0.25,-0.5]),6.,[0.,0.,1.,1.]);guides.label=Some((point([0.5,0.25,-0.5]),"guide".into()));gpu.set_guides(&guides).unwrap();let mark=crate::measure_mark::Mark{points:vec![point([-0.5,-0.25,-0.5]),point([0.5,-0.25,-0.5])],label:"distance".into()};gpu.set_measurement(Some(&mark)).unwrap();let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.submit(point([0.,0.,-0.5])).unwrap();draft.hover(Some(point([0.5,0.,-0.5]))).unwrap();gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release(){
 let scene=Scene::empty();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();temporary_inputs(&mut gpu,[0.;3]);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]>200&&p[2]<30));assert!(baseline.chunks_exact(4).any(|p|p[0]>150&&p[1]<80&&p[2]<80));assert!(baseline.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>200);let bytes=(gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes());assert!(bytes.0>0&&bytes.1>0&&bytes.2>0);assert_eq!(gpu.text_error(),None);
 let anchor=[1e9,-1e9,1e9];assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);camera.target=anchor;temporary_inputs(&mut gpu,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes()),bytes);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert!(scene.all_ids().is_empty());gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap(),None);
 gpu.set_preview(None).unwrap();gpu.set_measurement(None).unwrap();gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor)).unwrap();crate::cap_gpu_tests::pixels_at(&gpu,local);assert_eq!(gpu.preview_bytes(),0);assert_eq!(gpu.guide_bytes(),0);assert_eq!(gpu.measurement.bytes(),0);assert_eq!(gpu.text_usage()[0],0);
}

#[test]
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let settings=Some(crate::grid::Settings{spacing:0.1,half_cells:5});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.space.update(anchor,2.,0.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let far_plane=plane([anchor[0],anchor[1],anchor[2]-0.75]);gpu.set_grid_on(settings,far_plane).unwrap();camera.target=anchor;let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);let grid_uploads=gpu.grid_usage()[2];gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(gpu.grid_usage()[2],grid_uploads);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];gpu.space.update(moved,2.,300.).unwrap();camera.target=moved;for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,moved);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert!(gpu.grid_usage()[2]>grid_uploads);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));gpu.space.update(anchor,2.,600.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_grid(None).unwrap();gpu.set_scene(&Scene::empty(),None);assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_initial_object_records_keep_precise_local_pixels_updates_and_sources(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.2,-0.25,-0.5],[0.3,-0.25,-0.5],[0.1,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload(&gpu.device,&layout,&scene.objects()[0],false);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let original=gpu.meshes[0].packed();let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload_at(&gpu.device,&layout,&far.objects()[0],false,anchor);assert_eq!(gpu.meshes[0].packed(),original);let geometry=gpu.meshes[0].geometry.clone();let style=crate::gpu_mesh::Style{selected:false,pickable:true,colour:None};gpu.meshes[0]=crate::gpu_mesh::GpuMesh::with_geometry_at(&gpu.device,&layout,&far.objects()[0],style,geometry.clone(),anchor);assert_eq!(gpu.meshes[0].packed(),original);assert!(Rc::ptr_eq(&gpu.meshes[0].geometry,&geometry));gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));camera.target=anchor;let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[0,0]);let selected=crate::gpu_mesh::Style{selected:true,..style};assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],selected),[1,16]);assert_eq!(&gpu.meshes[0].packed()[..64],&original[..64]);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[1,16]);assert_eq!(gpu.meshes[0].packed(),original);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);assert_eq!(gpu.stats()[0],uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzh-prepare-anchor-relative-original-highlights-18.rs" skip=56825
```

## <a id="checkpoint-96zzzzzi-share-one-camera-and-measurement-unit"></a>11. Verify the shared unit through history

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 11</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_tests.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Shared camera unit</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measurements and projecti…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original history</text>
</svg>
</div>

**What this code does:** Check that measurement, projection and original history use the shared camera unit.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/measure_tests.rs`

Supply one unit/measurement/camera-inverse/original-source and exact Undo/Redo fixture while retaining earlier measurement assertions.

**Locate this existing block:**

```text
use crate::{
    editor::{Action, Editor},
    measure_mark::{Kind, Unit},
    scene::Scene,
};
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, text: &str) -> Result<(), String> {
    let action = if let Some(a) = e.tool_line(text) {
        a.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(a) = crate::commands::parse(text)? else {
            return Err("Editor action".into());
        };
        a
    };
    e.apply(action).map_err(|m| {
        if m == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            m.into()
        }
    })?;
    Ok(())
}
#[test]
fn bounding_box_is_an_original_wire_box_with_world_lengths_exact_archive_and_one_undo() {
    let mut e = editor();
    command(&mut e, "Line w1000000000.125,23,31 w1000000006.125,29,31").unwrap();
    let old = e.scene.lines()[0].prepared.source.clone();
    command(&mut e, "Select All").unwrap();
    command(&mut e, "Bounding Box").unwrap();
    let row = &e.scene.objects()[0];
    let target = row.id;
    let Some(Geometry::OBB(source)) = &row.original else {
        panic!("Original box")
    };
    assert_eq!(source.center[0], 1000000003.125);
    assert_eq!(
        [
            source.half_size[0],
            source.half_size[1],
            source.half_size[2]
        ],
        [3., 3., 0.]
    );
    assert_eq!(row.model.m, Xform::identity().m);
    assert!(row.mesh.indices().is_empty());
    assert_eq!(row.mesh.parts.edges.len(), 12);
    let proto = source.to_proto();
    command(&mut e, "Length BoundingBox").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "X 6 · Y 6 · Z 0 mm");
    assert_eq!(e.selected_objects(), [target]);
    assert!(Rc::ptr_eq(&old, &e.scene.lines()[0].prepared.source));
    let mut restored = Scene::empty();
    crate::scene_document::open(
        &mut restored,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    let Some(Geometry::OBB(source)) = &restored.objects()[0].original else {
        panic!("Reopened original box")
    };
    assert_eq!(source.to_proto(), proto);
    assert!(restored.objects()[0].mesh.indices().is_empty());
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
    assert!(e.mark.is_none());
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
}
#[test]
fn selection_measurements_use_placed_originals_and_preserve_scene_history_and_sources() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w3,4,0").unwrap();
    let line = e.scene.lines()[0].id;
    e.scene
        .place_owner(line, Xform::scale_xyz(2., 3., 1.))
        .unwrap();
    let old = e.scene.lines()[0].prepared.source.clone();
    command(&mut e, "Point w1,1,1").unwrap();
    command(&mut e, "Undo").unwrap();
    e.select_objects([line], false);
    command(&mut e, "Length").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "13.416 mm");
    assert!(Rc::ptr_eq(&old, &e.scene.lines()[0].prepared.source));
    command(&mut e, "Length BoundingBox").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "X 6 · Y 12 · Z 0 mm");
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    assert!(e.mark.is_none());
}
#[test]
fn area_volume_face_units_and_open_refusal_follow_original_geometry() {
    let mut e = editor();
    command(&mut e, "Box w0,0,0 2 2 0.4").unwrap();
    let target = e.scene.objects()[0].id;
    e.select_objects([target], false);
    command(&mut e, "Area").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "11.2 mm²");
    command(&mut e, "Volume").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 mm³");
    e.camera.unit = Unit::Meters;
    command(&mut e, "Volume").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 m³");
    let source = e.scene.objects()[0].cad.as_ref().unwrap().source.clone();
    e.apply(Action::PickedTarget(Some(crate::pick_sources::Target {
        owner: target,
        part: crate::pick_sources::Part::Face(0),
    })))
    .unwrap();
    command(&mut e, "Area").unwrap();
    let area =
        crate::measure_area::compute_area(&Geometry::BRep(source), &Xform::identity(), Some(0))
            .unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().label,
        format!("{} m²", crate::measure_math::to_text(area))
    );
    let surface = Rc::new(
        crate::patch_geometry::patch(&[
            Point::new(3., 0., 0.),
            Point::new(4., 0., 0.),
            Point::new(4., 1., 0.),
            Point::new(3., 1., 0.),
        ])
        .unwrap(),
    );
    let id = e.scene.insert_surface(surface).unwrap();
    e.select_objects([id], false);
    assert!(command(&mut e, "Volume").is_err());
    assert!(e.mark.is_none());
    assert!(
        e.scene
            .objects()
            .iter()
            .find(|r| r.id == id)
            .unwrap()
            .surface
            .is_some()
    );
}
#[test]
fn distance_accepts_world_relative_and_plane_inputs_without_creating_originals() {
    let mut e = editor();
    command(&mut e, "CPlane XZ").unwrap();
    command(&mut e, "Measure Distance w0,0,0 w3,4,0").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "5 mm");
    assert!(e.tool_status.contains("dx 3 · dy 4 · dz 0"));
    assert!(e.scene.lines().is_empty());
    assert!(e.scene.objects().is_empty());
    command(&mut e, "Snap Off").unwrap();
    command(&mut e, "Measure Distance").unwrap();
    assert!(e.snapping.objects);
    command(&mut e, "0,0,0").unwrap();
    command(&mut e, "@3,4,0").unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().points,
        vec![[0., 0., 0.], [3., 0., 4.]]
    );
    assert_eq!(e.mark.as_ref().unwrap().label, "5 mm");
    assert!(e.draft.is_none());
    e.apply(Action::Zoom(1.1)).unwrap();
    assert!(e.mark.is_some());
    command(&mut e, "Escape").unwrap();
    assert!(e.mark.is_none());
    assert!(e.scene.lines().is_empty());
}
#[test]
fn invalid_measurement_arguments_keep_pending_answers_and_redo_and_no_source_proxy_is_measured() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    command(&mut e, "Point w1,1,1").unwrap();
    command(&mut e, "Undo").unwrap();
    e.select_objects([], false);
    assert!(command(&mut e, "BoundingBox").is_err());
    assert!(command(&mut e, "Length").is_err());
    assert!(crate::commands::parse("Length BoundingBox extra").is_err());
    command(&mut e, "Curve").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "Measure Distance bad").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    let id = crate::bounds_original::insert(
        &mut e.scene,
        Rc::new(crate::bounds_original::box_geometry(
            &crate::bounds::Bounds {
                min: [0.; 3],
                max: [1.; 3],
            },
        )),
    )
    .unwrap();
    e.select_objects([id], false);
    assert!(command(&mut e, "Area").is_err());
    assert!(command(&mut e, "Volume").is_err());
    assert!(command(&mut e, "Length").is_err());
}
#[test]
fn temporary_annotation_ids_do_not_collide_with_scene_labels_or_gain_pick_owners() {
    let (mark, _) = crate::measure_mark::distance([0.; 3], [3., 4., 0.], Unit::Millimeters);
    let mut existing = mark.labels(&[]);
    existing[0].id = u32::MAX;
    let labels = mark.labels(&existing);
    assert_eq!(labels.len(), 3);
    assert!(labels.iter().all(|l| l.object.is_none()));
    let ids = labels
        .iter()
        .map(|l| l.id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(ids.len(), 3);
    assert!(
        labels
            .iter()
            .all(|l| existing.iter().all(|old| old.id != l.id))
    );
    assert_eq!(labels[0].text, "5 mm");
    assert_eq!(labels[1].text, "■");
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzi-share-one-camera-and-measurement-unit-08.rs" skip=8077
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 21.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 21.1
npm --prefix ../session_tests run course -- save 21.1
```

Continue with [21.2 · Precise CPU queries and origin-stamped picks](../../consolidation/96zzzzzj-project-cpu-queries-around-the-precise-target.md).
