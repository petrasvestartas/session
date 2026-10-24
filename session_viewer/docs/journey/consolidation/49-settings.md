# 7.4 · Precise snapping with derived search caches

**Goal:** Snap actual drawing input through the command dock. · **Steps:** 8 · **Source:** [app/command/mod.rs](../../../src/app/command/mod.rs) · [app/command/tool.rs](../../../src/app/command/tool.rs) · [app/cplane.rs](../../../src/app/cplane.rs) · [app/snap.rs](../../../src/app/snap.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Ray ∩ C-Plane: t = -dot(ro - O, n) / dot(rd, n)</text>
  <polygon points="48,144 128,84 316,84 236,144" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="88" y1="114" x2="276" y2="114" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="182" y1="84" x2="102" y2="144" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="74" y1="48" x2="182" y2="114" stroke="#dc2626" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#arr)"/>
  <circle cx="182" cy="114" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="200" y="110" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">P_snap(u,v)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D Gumball Axes &amp; Arcs (Move / Rotate / Scale)</text>
  <circle cx="560" cy="116" r="46" fill="none" stroke="#2563eb" stroke-width="2" stroke-dasharray="5 3"/>
  <line x1="560" y1="116" x2="646" y2="116" stroke="#dc2626" stroke-width="3" marker-end="url(#arrR)"/>
  <line x1="560" y1="116" x2="560" y2="44" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <line x1="560" y1="116" x2="504" y2="154" stroke="#2563eb" stroke-width="3" marker-end="url(#arrB)"/>
  <rect x="618" y="84" width="10" height="10" fill="#dc2626" stroke="#111111" stroke-width="1.2"/>
  <circle cx="560" cy="116" r="5" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Transformed Geometry: M_delta * P_local</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="842,136 882,88 932,136" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
  <polygon points="918,126 958,78 1008,126" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <line x1="882" y1="112" x2="952" y2="102" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
</svg>
</div>

## <a id="checkpoint-49-settings"></a>1. Separate object and grid snapping

### 1. `src/snap.rs`

Define snap kinds, independent switches and validated current1000mm grid settings.

Create the file and type:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    End,
    Vertex,
    Mid,
    Center,
    Perp,
    Near,
    Grid,
}

pub const END: u8 = 1;
pub const NEAR: u8 = 2;
pub const MID: u8 = 4;
pub const CENTER: u8 = 8;
pub const PERP: u8 = 16;
pub const MODES: [(&str, u8); 5] = [
    ("End", END),
    ("Near", NEAR),
    ("Mid", MID),
    ("Center", CENTER),
    ("Perp", PERP),
];

impl Kind {
    pub fn mode(self) -> u8 {
        match self {
            Self::End | Self::Vertex => END,
            Self::Near => NEAR,
            Self::Mid => MID,
            Self::Center => CENTER,
            Self::Perp => PERP,
            Self::Grid => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
    pub point: [f64; 3],
    pub kind: Kind,
    pub owner: Option<crate::scene::ObjectId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub objects: bool,
    pub modes: u8,
    pub grid: bool,
    pub step: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            objects: true,
            modes: END | NEAR | MID | CENTER,
            grid: true,
            step: 1000.,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Setting {
    Objects(Option<bool>),
    Mode(u8),
    Grid(Option<bool>),
    Spacing(f64),
}

impl Settings {
    pub fn apply(&mut self, setting: Setting) -> Result<String, &'static str> {
        Ok(match setting {
            Setting::Objects(on) => {
                self.objects = on.unwrap_or(!self.objects);
                format!(
                    "Snap {}",
                    if self.objects {
                        "On"
                    } else {
                        "Off"
                    }
                )
            }
            Setting::Grid(on) => {
                self.grid = on.unwrap_or(!self.grid);
                self.grid_status()
            }
            Setting::Spacing(step) => {
                if !step.is_finite() || step <= 0. {
                    return Err(
                        "The grid spacing must be above zero · Example: Grid Snap 50",
                    );
                }
                self.step = step;
                self.grid = true;
                self.grid_status()
            }
            Setting::Mode(bit) => {
                let (name, _) = MODES
                    .iter()
                    .find(|(_, value)| *value == bit)
                    .ok_or("Unknown snap mode")?;
                self.modes ^= bit;
                format!(
                    "Snap {name} {}",
                    if self.modes & bit != 0 {
                        "On"
                    } else {
                        "Off"
                    }
                )
            }
        })
    }
    fn grid_status(&self) -> String {
        if self.grid {
            format!("Grid Snap On · {} mm", self.step)
        } else {
            "Grid Snap Off".into()
        }
    }
}
```

### 2. `src/lib.rs`

Register the snap component and supplied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod selection_cycle_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod selection_cycle_tests;

pub mod snap;
```

## <a id="checkpoint-49a-math"></a>2. Keep snap math in world and plane coordinates

### 1. `src/snap_math.rs`

Resolve screen projection, local-plane rounding and nearest/perpendicular segment points.

Create the file and type:

```rust
#[derive(Clone, PartialEq)]
pub struct Screen {
    pub matrix: [f64; 16],
    pub size: [f64; 2],
}

impl Screen {
    pub fn point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        if self
            .size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let clip: [f64; 4] = std::array::from_fn(|row| {
            point
                .iter()
                .enumerate()
                .fold(self.matrix[12 + row], |sum, (i, v)| {
                    sum + self.matrix[i * 4 + row] * v
                })
        });
        if clip.iter().any(|v| !v.is_finite())
            || clip[3] <= 1e-9
            || clip[2] < 0.
            || clip[2] > clip[3]
        {
            return None;
        }
        let pixel = [
            (clip[0] / clip[3] + 1.) * self.size[0] / 2.,
            (1. - clip[1] / clip[3]) * self.size[1] / 2.,
        ];
        pixel
            .iter()
            .all(|v| v.is_finite())
            .then_some(pixel)
    }
}

// Round the plane's two coordinates around its origin; retain the signed height along its normal.
pub fn grid(
    point: [f64; 3],
    origin: [f64; 3],
    axes: [[f64; 3]; 2],
    step: f64,
) -> Option<[f64; 3]> {
    crate::tool_coords::checked(point).ok()?;
    crate::tool_coords::checked(origin).ok()?;
    let [x, y] = axes;
    if !step.is_finite()
        || step <= 0.
        || x.iter()
            .chain(y.iter())
            .any(|v| !v.is_finite())
        || (dot(x, x) - 1.).abs() > 1e-9
        || (dot(y, y) - 1.).abs() > 1e-9
        || dot(x, y).abs() > 1e-9
    {
        return None;
    }
    let z = [
        x[1] * y[2] - x[2] * y[1],
        x[2] * y[0] - x[0] * y[2],
        x[0] * y[1] - x[1] * y[0],
    ];
    let d = sub(point, origin);
    let mut local = [dot(d, x), dot(d, y), dot(d, z)];
    for value in &mut local[..2] {
        let units = *value / step;
        if units.is_finite() {
            *value = units.round() * step;
        }
    }
    crate::tool_coords::checked(std::array::from_fn(|i| {
        origin[i] + local[0] * x[i] + local[1] * y[i] + local[2] * z[i]
    }))
    .ok()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.iter().zip(b).map(|(a, b)| a * b).sum()
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}
fn along(a: [f64; 3], u: [f64; 3], t: f64) -> [f64; 3] {
    std::array::from_fn(|i| a[i] + u[i] * t)
}

// Minimize the distance between a source segment and the current ray, then clamp the result to the segment.
pub fn near(a: [f64; 3], b: [f64; 3], ray: &crate::camera::Ray) -> [f64; 3] {
    let u = sub(b, a);
    let w = sub(a, std::array::from_fn(|i| ray.origin[i]));
    let d = std::array::from_fn(|i| ray.direction[i]);
    let (uu, ud, dd) = (dot(u, u), dot(u, d), dot(d, d));
    let denominator = uu * dd - ud * ud;
    let t = if denominator > 1e-12 * uu * dd {
        ((ud * dot(d, w) - dd * dot(u, w)) / denominator).clamp(0., 1.)
    } else {
        0.
    };
    along(a, u, t)
}

pub fn perpendicular(a: [f64; 3], b: [f64; 3], from: [f64; 3]) -> Option<[f64; 3]> {
    let u = sub(b, a);
    let length = dot(u, u);
    if length <= 1e-24 {
        return None;
    }
    let t = dot(sub(from, a), u) / length;
    (t > 1e-9 && t < 1. - 1e-9).then(|| along(a, u, t))
}
```

### 2. `src/lib.rs`

Register the snap component and supplied checks.

**Locate this existing block:**

```rust
mod selection_cycle_tests;

pub mod snap;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod selection_cycle_tests;

pub mod snap;

pub mod snap_math;
```

## <a id="checkpoint-49b-sources"></a>3. Collect targets from placed geometry

Collect targets from original placed geometry.

### 1. `src/snap_sources.rs`

Collect precise placed source candidates and original boundaries without retaining source owners.

Create the file and type:

```rust
use crate::{
    scene::{ObjectId, Scene},
    snap::{Hit, Kind},
};

pub enum Target {
    Point(Hit),
    Wire {
        ends: [[f64; 3]; 2],
        owner: ObjectId,
    },
}

pub fn collect(scene: &Scene) -> Vec<Target> {
    let mut out = Vec::new();
    for row in scene.points().iter().filter(|r| r.visible()) {
        let p = row.point();
        point(
            &mut out,
            row.id,
            [p[0], p[1], p[2]],
            Kind::End,
        );
    }
    for row in scene.lines().iter().filter(|r| r.visible()) {
        let ends = row.endpoints().map(|p| [p[0], p[1], p[2]]);
        polyline(&mut out, row.id, &ends);
    }
    for row in scene.paths().iter().filter(|r| r.visible()) {
        let points: Vec<_> = row
            .points()
            .iter()
            .map(|p| [p[0], p[1], p[2]])
            .collect();
        if let crate::chain::Source::Curve(curve, _) = &row.prepared.source {
            let (a, b) = curve.domain();
            for (t, kind) in [
                (a, Kind::End),
                (b, Kind::End),
                ((a + b) * 0.5, Kind::Mid),
            ] {
                let p = row.model.transform_point(&curve.point_at(t));
                point(&mut out, row.id, [p[0], p[1], p[2]], kind);
            }
            wires(&mut out, row.id, &points);
        } else {
            polyline(&mut out, row.id, &points);
        }
        if points.len() >= 4
            && (0..3)
                .map(|i| (points[0][i] - points[points.len() - 1][i]).powi(2))
                .sum::<f64>()
                .sqrt()
                <= 1e-9
        {
            let corners = &points[..points.len() - 1];
            let center = std::array::from_fn(|i| {
                corners.iter().map(|p| p[i]).sum::<f64>() / corners.len() as f64
            });
            point(&mut out, row.id, center, Kind::Center);
        }
    }
    for row in scene
        .objects()
        .iter()
        .filter(|r| r.visible())
    {
        let mut seen = std::collections::HashSet::new();
        if let Some(source) = row.geometry() {
            for key in source.vertices() {
                if let Some(p) = source.vertex_point(key) {
                    let p = row.model.transform_point(&p);
                    point(
                        &mut out,
                        row.id,
                        [p[0], p[1], p[2]],
                        Kind::Vertex,
                    );
                    seen.insert(key);
                }
            }
        }
        for (&(a, b), ends) in row
            .mesh
            .parts
            .edges
            .iter()
            .zip(&row.mesh.parts.endpoints)
        {
            let ends = ends.map(|p| {
                let p = row
                    .model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                [p[0], p[1], p[2]]
            });
            for (key, p) in [(a, ends[0]), (b, ends[1])] {
                if seen.insert(key) {
                    point(&mut out, row.id, p, Kind::Vertex);
                }
            }
            out.push(Target::Wire {
                ends,
                owner: row.id,
            });
        }
    }
    out
}

fn point(out: &mut Vec<Target>, owner: ObjectId, point: [f64; 3], kind: Kind) {
    out.push(Target::Point(Hit {
        point,
        kind,
        owner: Some(owner),
    }));
}
fn wires(out: &mut Vec<Target>, owner: ObjectId, points: &[[f64; 3]]) {
    for ends in points.windows(2) {
        out.push(Target::Wire {
            ends: [ends[0], ends[1]],
            owner,
        });
    }
}
fn polyline(out: &mut Vec<Target>, owner: ObjectId, points: &[[f64; 3]]) {
    for (i, &p) in points.iter().enumerate() {
        point(
            out,
            owner,
            p,
            if i == 0 || i + 1 == points.len() {
                Kind::End
            } else {
                Kind::Vertex
            },
        );
    }
    for p in points.windows(2) {
        point(
            out,
            owner,
            std::array::from_fn(|i| (p[0][i] + p[1][i]) * 0.5),
            Kind::Mid,
        );
    }
    wires(out, owner, points);
}
```

### 2. `src/lib.rs`

Register the snap component and supplied checks.

**Locate this existing block:**

```rust
pub mod snap;

pub mod snap_math;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod snap;

pub mod snap_math;

pub mod snap_sources;
```

## <a id="checkpoint-49c-bins"></a>4. Search nearby screen cells

### 1. `src/snap_bins.rs`

Bound screen-cell insertion and deduplicate nearby candidate IDs.

Create the file and type:

```rust
use std::collections::HashMap;

// Small screen cells bound nearby queries; segments spanning too many cells stay in one conservative fallback list.
#[derive(Default)]
pub struct Bins {
    cells: HashMap<(i32, i32), Vec<usize>>,
    large: Vec<usize>,
}

impl Bins {
    const CELL: f64 = 64.;
    pub fn insert(&mut self, id: usize, ends: Option<[[f64; 2]; 2]>) {
        let Some([a, b]) = ends else {
            self.large.push(id);
            return;
        };
        let low = std::array::from_fn::<_, 2, _>(|i| {
            (a[i].min(b[i]) / Self::CELL).floor() as i32
        });
        let high = std::array::from_fn::<_, 2, _>(|i| {
            (a[i].max(b[i]) / Self::CELL).floor() as i32
        });
        let width = high[0] as i64 - low[0] as i64 + 1;
        let height = high[1] as i64 - low[1] as i64 + 1;
        if width > 64 || height > 64 || width * height > 64 {
            self.large.push(id);
            return;
        }
        for y in low[1]..=high[1] {
            for x in low[0]..=high[0] {
                self.cells
                    .entry((x, y))
                    .or_default()
                    .push(id);
            }
        }
    }
    pub fn near(&self, cursor: [f64; 2], radius: f64) -> Vec<usize> {
        let mut out = self.large.clone();
        let low = cursor.map(|v| ((v - radius) / Self::CELL).floor() as i32);
        let high = cursor.map(|v| ((v + radius) / Self::CELL).floor() as i32);
        for y in low[1]..=high[1] {
            for x in low[0]..=high[0] {
                if let Some(ids) = self.cells.get(&(x, y)) {
                    out.extend(ids);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}
```

### 2. `src/lib.rs`

Register the snap component and supplied checks.

**Locate this existing block:**

```rust
pub mod snap_math;

pub mod snap_sources;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod snap_math;

pub mod snap_sources;

pub mod snap_bins;
```

## <a id="checkpoint-49d-index"></a>5. Choose snap kinds before grid fallback

### 1. `src/snap_index.rs`

Rank nearby valid candidates and apply independent construction-plane grid fallback.

Create the file and type:

```rust
use crate::{
    snap::{Hit, Kind, Settings},
    snap_math::Screen,
    snap_sources::Target,
};

pub struct Index {
    targets: Vec<Target>,
    screen: Option<Screen>,
    bins: crate::snap_bins::Bins,
    pub builds: u64,
    pub visits: usize,
}

impl Index {
    pub fn new(scene: &crate::scene::Scene) -> Self {
        Self {
            targets: crate::snap_sources::collect(scene),
            screen: None,
            bins: Default::default(),
            builds: 0,
            visits: 0,
        }
    }
    pub fn query(
        &mut self,
        screen: Screen,
        ray: &crate::camera::Ray,
        cursor: [f64; 2],
        modes: u8,
        from: Option<[f64; 3]>,
    ) -> Option<Hit> {
        if cursor
            .iter()
            .chain(screen.size.iter())
            .any(|v| !v.is_finite())
            || screen.size.iter().any(|v| *v <= 0.)
        {
            return None;
        }
        if self.screen.as_ref() != Some(&screen) {
            self.bins = Default::default();
            self.builds += 1;
            for (id, target) in self.targets.iter().enumerate() {
                match target {
                    Target::Point(hit) => {
                        if let Some(p) = screen.point(hit.point) {
                            self.bins.insert(id, Some([p, p]));
                        }
                    }
                    Target::Wire { ends, .. } => self.bins.insert(
                        id,
                        screen
                            .point(ends[0])
                            .zip(screen.point(ends[1]))
                            .map(|(a, b)| [a, b]),
                    ),
                }
            }
            self.screen = Some(screen.clone());
        }
        let ids = self.bins.near(cursor, 12.);
        self.visits = ids.len();
        let mut winner = None;
        for id in ids {
            match &self.targets[id] {
                Target::Point(hit) => choose(&mut winner, *hit, &screen, cursor, modes),
                Target::Wire { ends, owner } => {
                    choose(
                        &mut winner,
                        Hit {
                            point: crate::snap_math::near(ends[0], ends[1], ray),
                            kind: Kind::Near,
                            owner: Some(*owner),
                        },
                        &screen,
                        cursor,
                        modes,
                    );
                    if let Some(p) = from.and_then(|from| {
                        crate::snap_math::perpendicular(ends[0], ends[1], from)
                    }) {
                        choose(
                            &mut winner,
                            Hit {
                                point: p,
                                kind: Kind::Perp,
                                owner: Some(*owner),
                            },
                            &screen,
                            cursor,
                            modes,
                        );
                    }
                }
            }
        }
        winner.map(|(hit, _)| hit)
    }
}

// A preferred kind wins inside the aperture before distance breaks ties between candidates of that same kind.
fn choose(
    winner: &mut Option<(Hit, f64)>,
    hit: Hit,
    screen: &Screen,
    cursor: [f64; 2],
    modes: u8,
) {
    if modes & hit.kind.mode() == 0 {
        return;
    }
    let Some(p) = screen.point(hit.point) else {
        return;
    };
    let distance = (p[0] - cursor[0]).hypot(p[1] - cursor[1]);
    if distance > 12. {
        return;
    }
    if winner.is_none_or(|(best, d)| {
        hit.kind < best.kind || (hit.kind == best.kind && distance < d)
    }) {
        *winner = Some((hit, distance));
    }
}

pub fn fallback(
    settings: Settings,
    camera: &crate::camera::Camera,
    screen: [f32; 2],
) -> Option<Hit> {
    let point = crate::tool_coords::on_xy(camera, screen)?;
    Some(if settings.grid {
        Hit {
            point: crate::snap_math::grid(
                point,
                [0.; 3],
                [[1., 0., 0.], [0., 1., 0.]],
                settings.step,
            )?,
            kind: Kind::Grid,
            owner: None,
        }
    } else {
        Hit {
            point,
            kind: Kind::Near,
            owner: None,
        }
    })
}
```

### 2. `src/lib.rs`

Register the snap component and supplied checks.

**Locate this existing block:**

```rust
pub mod snap_sources;

pub mod snap_bins;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod snap_sources;

pub mod snap_bins;

pub mod snap_index;
```

## <a id="checkpoint-49e-cache"></a>6. Invalidate snap caches after source edits

### 1. `src/editor.rs`

Own derived target caches and invalidate them after source and placement transactions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        match action {
            Action::BeginTool(shape) => {
                self.draft = Some(crate::tool::Draft::new(shape));
```

Replace it with:

```rust
        }
        match action {
            Action::BeginTool(shape) => {
                self.snap_index = None;
                self.draft = Some(crate::tool::Draft::new(shape));
```

**Locate this block:**

```rust
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        if matches!(
            &action,
            Action::Close
```

Replace it with:

```rust
        Ok(Change::Scene)
    }

    pub fn snap_point(
        &mut self,
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::snap::Hit> {
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let index = self
                .snap_index
                .get_or_insert_with(|| crate::snap_index::Index::new(&self.scene));
            let viewport = crate::snap_math::Screen {
                matrix: self.camera.view_projection().m,
                size,
            };
            index.query(
                viewport,
                &ray,
                [
                    (screen[0] as f64 + 1.) * size[0] / 2.,
                    (1. - screen[1] as f64) * size[1] / 2.,
                ],
                self.snapping.modes,
                from,
            )
        } else {
            None
        };
        hit.or_else(|| crate::snap_index::fallback(self.snapping, &self.camera, screen))
    }

    pub fn snap_usage(&self) -> [u64; 2] {
        self.snap_index
            .as_ref()
            .map_or([0, 0], |index| {
                [index.builds, index.visits as u64]
            })
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let change = self.apply_action(action)?;
        if change == Change::Scene {
            self.snap_index = None;
        }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {
        if matches!(
            &action,
            Action::Close
```

**Locate this block:**

```rust
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        Ok(Change::Scene)
    }
```

Replace it with:

```rust
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

**Locate this block:**

```rust
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        Ok(Change::Scene)
    }
```

Replace it with:

```rust
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

**Locate this block:**

```rust
                scene.hydrate(plan);
            }
        }
        Ok(true)
    }
```

Replace it with:

```rust
                scene.hydrate(plan);
            }
        }
        self.snap_index = None;
        Ok(true)
    }
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            draft: None,
            tool_status: String::new(),
            scene: Scene::demo(),
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            snapping: Default::default(),
            snap_index: None,
            draft: None,
            tool_status: String::new(),
            scene: Scene::demo(),
```

**Locate this block:**

```rust
pub struct Editor {
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
```

Replace it with:

```rust
pub struct Editor {
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
```

## <a id="checkpoint-49ea-state"></a>7. Preserve drafts while snapping

### 1. `src/tool.rs`

Connect snap actions and feedback while preserving accepted draft points and history.

**Locate this existing block:**

```rust
    pub cursor: Option<[f64; 3]>,
}

impl Draft {
    pub fn new(shape: Shape) -> Self {
        Self {
            tool: Box::new(shape),
            points: Vec::new(),
            cursor: None,
        }
    }
    pub fn prompt(&self) -> String {
        format!(
            "{} point {} of {} · Esc cancels",
            self.tool.name(),
            self.points.len() + 1,
            self.tool.count()
        )
    }
    pub fn submit(&mut self, point: [f64; 3]) -> Result<Option<Built>, &'static str> {
        let point = crate::tool_coords::checked(point)?;
        let mut points = self.points.clone();
        points.push(point);
        if points.len() == self.tool.count() {
            return self.tool.build(&points).map(Some);
        }
        if points.len() > self.tool.count() {
            return Err("The tool has enough points");
        }
        self.points = points;
        self.cursor = None;
        Ok(None)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub cursor: Option<[f64; 3]>,
    pub snapped: Option<crate::snap::Kind>,
}

impl Draft {
    pub fn new(shape: Shape) -> Self {
        Self {
            tool: Box::new(shape),
            points: Vec::new(),
            cursor: None,
            snapped: None,
        }
    }
    pub fn prompt(&self) -> String {
        let mut prompt = format!(
            "{} point {} of {} · Esc cancels",
            self.tool.name(),
            self.points.len() + 1,
            self.tool.count()
        );
        if let Some(kind) = self.snapped {
            prompt.push_str(&format!(" · Snap {kind:?}"));
        }
        prompt
    }
    pub fn submit(&mut self, point: [f64; 3]) -> Result<Option<Built>, &'static str> {
        let point = crate::tool_coords::checked(point)?;
        let mut points = self.points.clone();
        points.push(point);
        if points.len() == self.tool.count() {
            return self.tool.build(&points).map(Some);
        }
        if points.len() > self.tool.count() {
            return Err("The tool has enough points");
        }
        self.points = points;
        self.cursor = None;
        self.snapped = None;
        Ok(None)
```

### 2. `src/editor.rs`

Connect snap actions and feedback while preserving accepted draft points and history.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => {
                if let Some(draft) = self.draft.as_mut() {
                    draft.hover(point)?;
```

Replace it with:

```rust
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => {
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = None;
                    draft.hover(point)?;
```

**Locate this block:**

```rust
            self.draft = None;
        }
        match action {
            Action::BeginTool(shape) => {
                self.snap_index = None;
                self.draft = Some(crate::tool::Draft::new(shape));
```

Replace it with:

```rust
            self.draft = None;
        }
        match action {
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() {
                    draft.cursor = None;
                    draft.snapped = None;
                }
                return Ok(Change::View);
            }
            Action::ToolScreen {
                screen,
                size,
                commit,
            } => {
                let hit = self.snap_point(screen, size);
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = hit
                        .filter(|h| {
                            h.owner.is_some() || h.kind == crate::snap::Kind::Grid
                        })
                        .map(|h| h.kind);
                }
                if commit {
                    return hit.map_or(Ok(Change::View), |hit| {
                        self.place_tool_point(hit.point)
                    });
                }
                if let Some(draft) = self.draft.as_mut() {
                    draft.hover(hit.map(|h| h.point))?;
                }
                return Ok(Change::View);
            }
            Action::BeginTool(shape) => {
                self.snap_index = None;
                self.draft = Some(crate::tool::Draft::new(shape));
```

**Locate this block:**

```rust
pub enum Action {
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
```

Replace it with:

```rust
pub enum Action {
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
        size: [f64; 2],
        commit: bool,
    },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
```

## <a id="checkpoint-49f-input"></a>8. Snap actual drawing input through the command dock

### 1. `src/commands.rs`

Route shared snap commands and current-camera CSS queries through tool-owned input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .map(|offset| Command::Editor(Action::Translate(offset)))
            .map_err(str::to_owned);
    }
```

Replace it with:

```rust
            .map(|offset| Command::Editor(Action::Translate(offset)))
            .map_err(str::to_owned);
    }
    if name == "Grid Snap" && words.len() > count {
        if words.len() != count + 1 {
            return Err("Grid Snap (On Off) or a spacing".into());
        }
        let step = words[count]
            .parse::<f64>()
            .map_err(|_| "Grid Snap (On Off) or a spacing")?;
        if !step.is_finite() || step <= 0. {
            return Err(
                "The grid spacing must be above zero · Example: Grid Snap 50".into(),
            );
        }
        return Ok(Command::Editor(Action::Snap(
            crate::snap::Setting::Spacing(step),
        )));
    }
```

**Locate this block:**

```rust
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Point", || {
        Action::BeginTool(crate::tool::Shape::Point)
    }),
```

Replace it with:

```rust
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Snap", || {
        Action::Snap(crate::snap::Setting::Objects(None))
    }),
    ("Snap On", || {
        Action::Snap(crate::snap::Setting::Objects(Some(true)))
    }),
    ("Snap Off", || {
        Action::Snap(crate::snap::Setting::Objects(Some(false)))
    }),
    ("Grid Snap", || {
        Action::Snap(crate::snap::Setting::Grid(None))
    }),
    ("Grid Snap On", || {
        Action::Snap(crate::snap::Setting::Grid(Some(true)))
    }),
    ("Grid Snap Off", || {
        Action::Snap(crate::snap::Setting::Grid(Some(false)))
    }),
    ("Snap End", || {
        Action::Snap(crate::snap::Setting::Mode(crate::snap::END))
    }),
    ("Snap Near", || {
        Action::Snap(crate::snap::Setting::Mode(crate::snap::NEAR))
    }),
    ("Snap Mid", || {
        Action::Snap(crate::snap::Setting::Mode(crate::snap::MID))
    }),
    ("Snap Center", || {
        Action::Snap(crate::snap::Setting::Mode(
            crate::snap::CENTER,
        ))
    }),
    ("Snap Perp", || {
        Action::Snap(crate::snap::Setting::Mode(crate::snap::PERP))
    }),
    ("Point", || {
        Action::BeginTool(crate::tool::Shape::Point)
    }),
```

### 2. `src/browser.rs`

Route shared snap commands and current-camera CSS queries through tool-owned input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor}).to_string()))?;
    canvas.set_attribute(
```

Replace it with:

```rust
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
    canvas.set_attribute("data-snap-settings", &serde_json::json!({"objects":editor.snapping.objects,"modes":editor.snapping.modes,"grid":editor.snapping.grid,"step":editor.snapping.step}).to_string())?;
    canvas.set_attribute(
        "data-snap-usage",
        &serde_json::to_string(&editor.snap_usage()).unwrap(),
    )?;
    canvas.set_attribute(
```

**Locate this block:**

```rust
                highlight_changed |= matches!(&action, Action::SelectAll);
                let tool_action = matches!(
                    &action,
                    Action::BeginTool(_) | Action::ToolPoint(_) | Action::CancelTool
                );
                if matches!(&action, Action::CancelTool) {
                    panel.cancel_input();
```

Replace it with:

```rust
                highlight_changed |= matches!(&action, Action::SelectAll);
                let tool_action = matches!(
                    &action,
                    Action::BeginTool(_)
                        | Action::ToolPoint(_)
                        | Action::CancelTool
                        | Action::Snap(_)
                        | Action::ToolScreen { commit: true, .. }
                );
                if matches!(&action, Action::CancelTool) {
                    panel.cancel_input();
```

**Locate this block:**

```rust
                                        / r.height())
                                        as f32,
                                ];
                                Action::ToolHover(crate::tool_coords::on_xy(
                                    &editor.camera,
                                    screen,
                                ))
                            })
                    }
                    Some(Action::Pick(screen)) => {
```

Replace it with:

```rust
                                        / r.height())
                                        as f32,
                                ];
                                Action::ToolScreen {
                                    screen,
                                    size: [r.width(), r.height()],
                                    commit: false,
                                }
                            })
                    }
                    Some(Action::Pick(screen)) => {
```

**Locate this block:**

```rust
                    Some(Action::Pick(screen)) if editor.draft.is_some() => {
                        crate::tool_coords::on_xy(&editor.camera, screen)
                            .map(Action::ToolPoint)
                    }
                    None if editor.draft.is_some()
                        && matches!(
```

Replace it with:

```rust
                    Some(Action::Pick(screen)) if editor.draft.is_some() => {
                        Some(Action::ToolScreen {
                            screen,
                            size: [
                                pointer_canvas
                                    .get_bounding_client_rect()
                                    .width(),
                                pointer_canvas
                                    .get_bounding_client_rect()
                                    .height(),
                            ],
                            commit: true,
                        })
                    }
                    None if editor.draft.is_some()
                        && matches!(
```

### 3. `src/snap_tests.rs`

Copy source precision, ranking, tilted-grid math, cache, draft and history regression checks.

Create the file and type:

```rust
use crate::{
    scene::Scene,
    snap::{Kind, Setting, Settings},
    snap_index::Index,
    snap_math::Screen,
};
use std::rc::Rc;

fn screen() -> Screen {
    Screen {
        matrix: session_rust::Xform::identity().m,
        size: [200., 100.],
    }
}
fn ray(p: [f64; 2]) -> crate::camera::Ray {
    crate::camera::Ray {
        origin: session_rust::Point::new(p[0], p[1], -1.),
        direction: session_rust::Vector::z_axis(),
        max_distance: 2.,
    }
}
fn empty() -> Scene {
    let mut scene = Scene::demo();
    scene.clear();
    scene
}
fn line(scene: &mut Scene, a: [f64; 3], b: [f64; 3]) -> crate::scene::ObjectId {
    scene
        .insert_line(
            crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                a[0], a[1], a[2], b[0], b[1], b[2],
            )))
            .unwrap(),
        )
        .unwrap()
}

#[test]
fn current_grid_default_and_independent_switches_validate_before_mutation() {
    let mut settings = Settings::default();
    assert!(settings.objects && settings.grid);
    assert_eq!(settings.step, 1000.);
    assert_eq!(
        settings.modes,
        crate::snap::END | crate::snap::NEAR | crate::snap::MID | crate::snap::CENTER
    );
    settings
        .apply(Setting::Objects(Some(false)))
        .unwrap();
    assert!(settings.grid && !settings.objects);
    settings
        .apply(Setting::Grid(Some(false)))
        .unwrap();
    settings
        .apply(Setting::Spacing(50.))
        .unwrap();
    assert!(settings.grid && !settings.objects);
    assert_eq!(settings.step, 50.);
    let before = settings;
    for value in [0., -1., f64::NAN, f64::INFINITY] {
        assert!(
            settings
                .apply(Setting::Spacing(value))
                .is_err()
        );
        assert_eq!(settings, before);
    }
    assert!(settings.apply(Setting::Mode(32)).is_err());
    assert_eq!(settings, before);
    settings
        .apply(Setting::Mode(crate::snap::PERP))
        .unwrap();
    assert_ne!(settings.modes & crate::snap::PERP, 0);
}

#[test]
fn grid_rounding_preserves_height_and_never_creates_nonfinite_points() {
    assert_eq!(
        crate::snap_math::grid(
            [149., 51., 33.3],
            [0.; 3],
            [[1., 0., 0.], [0., 1., 0.]],
            100.
        )
        .unwrap(),
        [100., 100., 33.3]
    );
    assert_eq!(
        crate::snap_math::grid(
            [149., 51., 33.3],
            [0.; 3],
            [[0., 1., 0.], [0., 0., 1.]],
            10.
        )
        .unwrap(),
        [149., 50., 30.]
    );
    assert_eq!(
        crate::snap_math::grid(
            [-150., 150., 0.],
            [0.; 3],
            [[1., 0., 0.], [0., 1., 0.]],
            100.
        )
        .unwrap(),
        [-200., 200., 0.]
    );
    assert!(
        crate::snap_math::grid(
            [1., 2., 3.],
            [0.; 3],
            [[1., 0., 0.], [0., 1., 0.]],
            0.
        )
        .is_none()
    );
    assert!(
        crate::snap_math::grid(
            [f64::NAN, 2., 3.],
            [0.; 3],
            [[1., 0., 0.], [0., 1., 0.]],
            50.
        )
        .is_none()
    );
    assert_eq!(
        crate::snap_math::grid(
            [1e12, 2., 3.],
            [0.; 3],
            [[1., 0., 0.], [0., 1., 0.]],
            1e-320
        )
        .unwrap(),
        [1e12, 2., 3.]
    );
}

#[test]
fn priority_wins_inside_twelve_css_pixels_then_distance_breaks_same_kind_ties() {
    let mut scene = empty();
    let owner = line(&mut scene, [-0.5, 0., 0.2], [0.5, 0., 0.2]);
    let mut index = Index::new(&scene);
    let hit = index
        .query(
            screen(),
            &ray([0.49, 0.]),
            [149., 50.],
            crate::snap::END | crate::snap::NEAR,
            None,
        )
        .unwrap();
    assert_eq!(hit.kind, Kind::End);
    assert_eq!(hit.point, [0.5, 0., 0.2]);
    assert_eq!(hit.owner, Some(owner));
    let near = index
        .query(
            screen(),
            &ray([0.49, 0.]),
            [149., 50.],
            crate::snap::NEAR,
            None,
        )
        .unwrap();
    assert_eq!(near.kind, Kind::Near);
    assert!((near.point[0] - 0.49).abs() < 1e-12);
    let mid = index
        .query(
            screen(),
            &ray([0., 0.]),
            [100., 50.],
            crate::snap::MID,
            None,
        )
        .unwrap();
    assert_eq!(mid.point, [0., 0., 0.2]);
    assert!(
        index
            .query(
                screen(),
                &ray([0.7, 0.]),
                [170., 50.],
                crate::snap::END | crate::snap::NEAR,
                None
            )
            .is_none()
    );
}

#[test]
fn placed_original_endpoints_curve_domain_midpoints_and_hidden_sources_are_distinct() {
    let mut scene = empty();
    let id = line(&mut scene, [-0.5, 0., 0.2], [0.5, 0., 0.2]);
    let shift = 0.123456789012345;
    scene
        .place_line(
            id,
            session_rust::Xform::translation(shift, 0., 0.),
        )
        .unwrap();
    let p = [0.5 + shift, 0., 0.2];
    let pixel = screen().point(p).unwrap();
    let mut index = Index::new(&scene);
    assert_eq!(
        index
            .query(
                screen(),
                &ray([p[0], p[1]]),
                pixel,
                crate::snap::END,
                None
            )
            .unwrap()
            .point,
        p
    );
    let mut hidden = session_rust::Point::new(0., 0.8, 0.2);
    hidden.is_visible = false;
    scene
        .insert_point(crate::marker::PreparedPoint::new(Rc::new(hidden)).unwrap())
        .unwrap();
    let mut index = Index::new(&scene);
    assert!(
        index
            .query(
                screen(),
                &ray([0., 0.8]),
                [100., 10.],
                crate::snap::END,
                None
            )
            .is_none()
    );
    let mut curve = session_rust::NurbsCurve::create(
        false,
        2,
        &[
            session_rust::Point::new(-0.5, 0., 0.2),
            session_rust::Point::new(0., 0.6, 0.2),
            session_rust::Point::new(0.5, 0., 0.2),
        ],
    );
    curve.is_locked = true;
    scene
        .insert_path(crate::chain::PreparedChain::curve(Rc::new(curve)).unwrap())
        .unwrap();
    let mut index = Index::new(&scene);
    let mid = index
        .query(
            screen(),
            &ray([0., 0.3]),
            [100., 35.],
            crate::snap::MID,
            None,
        )
        .unwrap();
    assert_eq!(mid.kind, Kind::Mid);
    assert!((mid.point[1] - 0.3).abs() < 1e-12);
    assert!((mid.point[1] - 0.6).abs() > 0.1);
    assert!(
        scene
            .pick_id(mid.owner.unwrap().value())
            .is_none(),
        "Read-only geometry can supply a snap without becoming an editable selection"
    );
}

#[test]
fn perpendicular_uses_the_previous_point_and_excludes_feet_outside_the_segment() {
    let mut scene = empty();
    line(&mut scene, [-0.5, 0., 0.2], [0.5, 0., 0.2]);
    let mut index = Index::new(&scene);
    let hit = index
        .query(
            screen(),
            &ray([0., 0.01]),
            [100., 49.5],
            crate::snap::PERP | crate::snap::NEAR,
            Some([0., 0.5, 0.2]),
        )
        .unwrap();
    assert_eq!(hit.kind, Kind::Perp);
    assert_eq!(hit.point, [0., 0., 0.2]);
    assert!(
        crate::snap_math::perpendicular(
            [-0.5, 0., 0.2],
            [0.5, 0., 0.2],
            [1., 0.5, 0.2]
        )
        .is_none()
    );
    assert!(crate::snap_math::perpendicular([0.; 3], [0.; 3], [1.; 3]).is_none());
}

#[test]
fn screen_cells_keep_queries_local_and_rebuild_only_for_a_changed_view() {
    let mut scene = empty();
    for i in 0..2000 {
        scene
            .insert_point(
                crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                    i as f64 * 2.,
                    0.,
                    0.2,
                )))
                .unwrap(),
            )
            .unwrap();
    }
    let weak = Rc::downgrade(&scene.points()[0].prepared.source);
    let mut index = Index::new(&scene);
    index
        .query(
            screen(),
            &ray([0., 0.]),
            [100., 50.],
            crate::snap::END,
            None,
        )
        .unwrap();
    assert_eq!(index.builds, 1);
    assert!(index.visits < 5);
    index
        .query(
            screen(),
            &ray([0.01, 0.]),
            [101., 50.],
            crate::snap::END,
            None,
        )
        .unwrap();
    assert_eq!(index.builds, 1);
    let mut changed = screen();
    changed.size = [300., 100.];
    index
        .query(
            changed,
            &ray([0., 0.]),
            [150., 50.],
            crate::snap::END,
            None,
        )
        .unwrap();
    assert_eq!(index.builds, 2);
    scene.clear();
    assert!(
        weak.upgrade().is_none(),
        "Derived snap coordinates do not retain original source owners"
    );
    let mut bins = crate::snap_bins::Bins::default();
    bins.insert(1, Some([[0., 0.], [128., 64.]]));
    bins.insert(2, Some([[-1e50, -1e50], [1e50, 1e50]]));
    assert_eq!(bins.near([127., 63.], 12.), vec![1, 2]);
    assert_eq!(bins.near([1e6, 1e6], 12.), vec![2]);
}

#[test]
fn snapping_settings_preserve_a_draft_and_typed_coordinates_bypass_grid_rounding() {
    use crate::editor::{Action, Editor};
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor
        .apply(Action::BeginTool(crate::tool::Shape::Line))
        .unwrap();
    editor
        .apply(Action::ToolPoint([
            0.123456789012345,
            0.25,
            0.,
        ]))
        .unwrap();
    let first = editor.draft.as_ref().unwrap().points.clone();
    editor
        .apply(Action::Snap(Setting::Objects(Some(false))))
        .unwrap();
    editor
        .apply(Action::ToolScreen {
            screen: [0.25, 0.25],
            size: [400., 300.],
            commit: false,
        })
        .unwrap();
    assert_eq!(
        editor.draft.as_ref().unwrap().cursor,
        Some([0., 0., 0.])
    );
    assert_eq!(
        editor.draft.as_ref().unwrap().snapped,
        Some(Kind::Grid)
    );
    editor
        .apply(Action::Snap(Setting::Spacing(50.)))
        .unwrap();
    assert_eq!(editor.draft.as_ref().unwrap().points, first);
    editor
        .apply(Action::Snap(Setting::Grid(Some(false))))
        .unwrap();
    editor
        .apply(Action::ToolScreen {
            screen: [0.25, 0.25],
            size: [400., 300.],
            commit: false,
        })
        .unwrap();
    assert_ne!(
        editor.draft.as_ref().unwrap().cursor,
        Some([0., 0., 0.])
    );
    assert_eq!(editor.draft.as_ref().unwrap().snapped, None);
    editor
        .apply(Action::ToolPoint([
            0.987654321012345,
            0.5,
            0.,
        ]))
        .unwrap();
    assert_eq!(
        editor.scene.lines()[0]
            .prepared
            .source
            .start()[0],
        first[0][0]
    );
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.lines().is_empty());
}

#[test]
fn original_transactions_invalidate_snap_positions_without_creating_snap_history() {
    use crate::editor::{Action, Editor};
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.camera.distance = 4.;
    editor.camera.aspect = 2.;
    editor.apply(Action::AddLine).unwrap();
    editor.apply(Action::SelectAll).unwrap();
    editor
        .apply(Action::BeginTool(crate::tool::Shape::Point))
        .unwrap();
    editor
        .apply(Action::Snap(Setting::Grid(Some(false))))
        .unwrap();
    editor.snapping.modes = crate::snap::END;
    let target = |editor: &Editor, p: [f64; 3]| {
        let p = editor
            .camera
            .view_projection()
            .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
        [p[0] as f32, p[1] as f32]
    };
    let old = target(&editor, [0.6, 0., 0.]);
    assert_eq!(
        editor
            .snap_point(old, [400., 200.])
            .unwrap()
            .point,
        [0.6, 0., 0.]
    );
    editor
        .apply(Action::Translate([0.5, 0., 0.]))
        .unwrap();
    let current = target(&editor, [1.1, 0., 0.]);
    assert_eq!(
        editor
            .snap_point(current, [400., 200.])
            .unwrap()
            .point,
        [1.1, 0., 0.]
    );
    assert!(
        editor
            .snap_point(old, [400., 200.])
            .unwrap()
            .owner
            .is_none()
    );
    editor.apply(Action::CancelTool).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.scene.lines()[0].endpoints()[1][0],
        0.6
    );
    for text in [
        "Grid Snap 0",
        "Grid Snap NaN",
        "Grid Snap -1",
        "Grid Snap 1 2",
        "Snap Maybe",
    ] {
        assert!(
            crate::commands::parse(text).is_err(),
            "{text}"
        );
    }
    assert!(matches!(
        crate::commands::parse("GridSnap 50").unwrap(),
        crate::commands::Command::Editor(Action::Snap(Setting::Spacing(50.)))
    ));
}

#[test]
fn a_closed_source_loop_offers_its_center_with_a_small_closure_tolerance() {
    let mut scene = empty();
    let points = vec![
        session_rust::Point::new(-0.5, -0.5, 0.2),
        session_rust::Point::new(0.5, -0.5, 0.2),
        session_rust::Point::new(0.5, 0.5, 0.2),
        session_rust::Point::new(-0.5, 0.5, 0.2),
        session_rust::Point::new(-0.5 + 1e-10, -0.5, 0.2),
    ];
    scene
        .insert_path(
            crate::chain::PreparedChain::polyline(Rc::new(
                session_rust::Polyline::new(points),
            ))
            .unwrap(),
        )
        .unwrap();
    let hit = Index::new(&scene)
        .query(
            screen(),
            &ray([0., 0.]),
            [100., 50.],
            crate::snap::CENTER,
            None,
        )
        .unwrap();
    assert_eq!(hit.kind, Kind::Center);
    assert_eq!(hit.point, [0., 0., 0.2]);
}

#[test]
fn a_tilted_grid_counts_from_its_origin_and_preserves_local_height() {
    let q = 2.0_f64.sqrt().recip();
    let origin = [10., 0., 0.];
    let axes = [[0., q, q], [-1., 0., 0.]];
    let world = |u: f64, v: f64, w: f64| [10. - v, q * (u - w), q * (u + w)];
    let point =
        crate::snap_math::grid(world(1490., 520., 7.), origin, axes, 1000.).unwrap();
    let expected = world(1000., 1000., 7.);
    assert!((0..3).all(|i| (point[i] - expected[i]).abs() < 1e-9));
    assert!(
        crate::snap_math::grid(
            world(1., 2., 3.),
            origin,
            [[1., 0., 0.], [1., 0., 0.]],
            10.
        )
        .is_none()
    );
}
```

### 4. `src/lib.rs`

Register the snap component and supplied checks.

**Locate this existing block:**

```rust
pub mod snap_bins;

pub mod snap_index;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod snap_bins;

pub mod snap_index;

#[cfg(test)]
mod snap_tests;
```

## Build & Run

![Recorded visual output for Lesson 7.4 (49f-input)](../../screenshots/journey/49f-input.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 7.4
npm --prefix ../session_tests run course -- save 7.4
```

Continue with [7.5 · Temporary transforms with a fixed baseline](50-placements.md).
