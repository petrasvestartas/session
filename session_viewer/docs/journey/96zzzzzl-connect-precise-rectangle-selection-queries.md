# 21.2.3 · Connect precise rectangle selection queries

**Chapter 21 · Precise camera and browser coordinates · Section 21.2 · CPU queries and asynchronous picks** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Select original geometry and text through one precise CPU frame.

## Type

Continue from [Prepare precise text selection polygons](96zzzzzk-prepare-precise-text-selection-polygons.md). [Save or recover your work](recovery.md).

### 1. `src/rect_select.rs`

Reuse one rectangle predicate while placing each original family in the explicit query frame.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::scene::{Scene,ObjectId};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rectangle {pub bounds:[f64;4],pub crossing:bool}

impl Rectangle {
    pub fn new(from:[f64;2],to:[f64;2])->Option<Self> {
        if from.into_iter().chain(to).any(|value|!value.is_finite()) {return None;}
        let bounds=[from[0].min(to[0]),from[1].min(to[1]),from[0].max(to[0]),from[1].max(to[1])];
        (bounds[0]<bounds[2]&&bounds[1]<bounds[3]).then_some(Self {bounds,crossing:to[0]<from[0]})
    }

    pub fn contains(&self,point:[f64;2])->bool {
        point.iter().all(|value|value.is_finite())&&point[0]>=self.bounds[0]&&point[0]<=self.bounds[2]&&point[1]>=self.bounds[1]&&point[1]<=self.bounds[3]
    }

    pub fn crosses(&self,a:[f64;2],b:[f64;2])->bool {
        if a.into_iter().chain(b).any(|value|!value.is_finite()) {return false;}
        let [left,top,right,bottom]=self.bounds;let [dx,dy]=[b[0]-a[0],b[1]-a[1]];let (mut low,mut high)=(0_f64,1_f64);
        for (p,q) in [(-dx,a[0]-left),(dx,right-a[0]),(-dy,a[1]-top),(dy,bottom-a[1])] {
            if p==0. {if q<0. {return false;}continue;}
            let t=q/p;if p<0. {low=low.max(t);} else {high=high.min(t);}
        }
        low<=high
    }
}

pub fn screen(matrix:&[f64;16],point:[f64;3],size:[f64;2])->Option<[f64;2]> {
    crate::snap_math::Screen {matrix:*matrix,size,origin:[0.;3]}.unclipped(point)
}

fn world(matrix:&session_rust::Xform,point:[f64;3])->[f64;3] {let point=matrix.transform_point(&session_rust::Point::new(point[0],point[1],point[2]));[point[0],point[1],point[2]]}

fn matches(matrix:&[f64;16],size:[f64;2],rectangle:Rectangle,strokes:Vec<Vec<[f64;3]>>)->bool {
    if strokes.is_empty() {return false;}
    if !rectangle.crossing {return strokes.iter().flatten().all(|point|screen(matrix,*point,size).is_some_and(|point|rectangle.contains(point)));}
    strokes.iter().any(|stroke| {
        let points=stroke.iter().map(|point|screen(matrix,*point,size)).collect::<Vec<_>>();
        points.iter().flatten().any(|point|rectangle.contains(*point))||points.windows(2).any(|pair|matches!(pair,[Some(a),Some(b)] if rectangle.crosses(*a,*b)))
    })
}

pub fn found(scene:&Scene,matrix:&[f64;16],size:[f64;2],rectangle:Rectangle)->Vec<ObjectId> {
    let eligible=|id:ObjectId|scene.pick_id(id.value()).is_some();let mut found=Vec::new();
    for row in scene.objects().iter().filter(|row|eligible(row.id)) {
        let mut strokes=row.mesh.parts.endpoints.iter().map(|ends|ends.iter().map(|point|world(&row.model,*point)).collect()).collect::<Vec<_>>();
        if let Some(source)=row.geometry() {strokes.extend(source.vertices().into_iter().filter_map(|key|source.vertex_point(key)).map(|point|vec![world(&row.model,[point[0],point[1],point[2]])]));}
        if matches(matrix,size,rectangle,strokes) {found.push(row.id);}
    }
    for row in scene.lines().iter().filter(|row|eligible(row.id)) {if matches(matrix,size,rectangle,vec![row.endpoints().map(|point|[point[0],point[1],point[2]]).to_vec()]) {found.push(row.id);}}
    for row in scene.paths().iter().filter(|row|eligible(row.id)) {if matches(matrix,size,rectangle,vec![row.points().into_iter().map(|point|[point[0],point[1],point[2]]).collect()]) {found.push(row.id);}}
    for row in scene.points().iter().filter(|row|eligible(row.id)) {let point=row.point();if matches(matrix,size,rectangle,vec![vec![[point[0],point[1],point[2]]]]) {found.push(row.id);}}
    for row in scene.texts().iter().filter(|row|eligible(row.id)) {if crate::scene_label_screen::matches(row,matrix,size,rectangle) {found.push(row.id);}}
    for row in scene.clouds().iter().filter(|row|eligible(row.id)) {let mut points=row.prepared.source.coords().chunks_exact(3).map(|p|screen(matrix,world(&row.model,[p[0],p[1],p[2]]),size));let hit=if rectangle.crossing {points.any(|p|p.is_some_and(|p|rectangle.contains(p)))} else {points.all(|p|p.is_some_and(|p|rectangle.contains(p)))};if hit {found.push(row.id);}}
    for row in scene.sheets().iter().filter(|row|eligible(row.id)) {let strokes=row.prepared.pages().iter().flat_map(|page|page.coordinates.chunks_exact(6).map(|p|vec![world(&row.model,[p[0],p[1],p[2]]),world(&row.model,[p[3],p[4],p[5]])]).collect::<Vec<_>>()).collect();if matches(matrix,size,rectangle,strokes) {found.push(row.id);}}
    found.sort_unstable_by_key(|id|id.value());found.dedup();found
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzl-connect-precise-rectangle-selection-queries-01.rs"
```

### 2. `src/editor.rs`

Connect the existing window/crossing gesture to the precise camera query owner.

<details>
<summary>Locate the existing block</summary>

```rust
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found_in(&self.scene,&crate::snap_math::Screen::camera(&self.camera,drag.size),rectangle);self.select_rows(ids,add,remove);}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzl-connect-precise-rectangle-selection-queries-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/camera_space_tests.rs`

Supply exact local/distant original window/crossing sets, text/point source identity, hidden eligibility and empty queries with unchanged earlier predicates.

<details>
<summary>Locate the existing block</summary>

```text
use crate::camera::{Camera,Projection,Unit};
use session_rust::Point;
#[test]
fn equivalent_millimetre_and_metre_scenes_share_screen_and_ray_geometry(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.unit=unit;camera.projection=projection;assert!(camera.view_projection().inverse().is_some());assert!(camera.ray([0.2,-0.1]).is_some());}}
 for projection in [Projection::Perspective,Projection::Orthographic]{
  let mut mm=Camera::default();mm.projection=projection;mm.target=[2e6,-3e6,4e6];mm.distance=5000.;mm.radius=1000.;
  let mut m=Camera::default();m.projection=projection;m.unit=Unit::Meters;m.target=mm.target.map(|x|x*0.001);m.distance=5.;m.radius=1.;
  let a=mm.view_projection_anchored(mm.target);let b=m.view_projection_anchored(m.target);
  let pa=a.transform_point(&Point::new(500.,250.,100.));let pb=b.transform_point(&Point::new(0.5,0.25,0.1));for i in 0..3{assert!((pa[i]-pb[i]).abs()<1e-12);}
  let ra=mm.ray([0.2,-0.1]).unwrap();let rb=m.ray([0.2,-0.1]).unwrap();for i in 0..3{assert!((ra.origin[i]*0.001-rb.origin[i]).abs()<1e-6);assert!((ra.direction[i]-rb.direction[i]).abs()<1e-6);}assert!((ra.max_distance*0.001-rb.max_distance).abs()<1e-5);
 }
}
#[test]
fn anchored_camera_keeps_submillimetre_detail_at_a_distant_origin(){
 let mut camera=Camera::default();camera.target=[1e12,-1e12,1e12];camera.distance=1.;camera.radius=1.;
 for projection in [Projection::Perspective,Projection::Orthographic]{camera.projection=projection;let local=camera.view_projection_anchored(camera.target).to_f32();let matrix=session_rust::Xform::from_matrix(local.map(f64::from));let a=matrix.transform_point(&Point::new(0.,0.,0.));let b=matrix.transform_point(&Point::new(0.000125,0.,0.));assert!(b[0]-a[0]>0.0001);assert!((0.0..1.0).contains(&a[2]));}
 for projection in [Projection::Perspective,Projection::Orthographic]{camera.projection=projection;let mut local=Camera::default();local.projection=projection;local.distance=1.;local.radius=1.;for screen in [[0.,0.],[0.2,-0.1]]{let a=camera.ray(screen).unwrap();let b=local.ray(screen).unwrap();for i in 0..3{assert!((a.origin[i]-camera.target[i]-b.origin[i]).abs()<0.00025);assert!((a.direction[i]-b.direction[i]).abs()<1e-10);}assert!((a.max_distance-b.max_distance).abs()<1e-8);}}
 assert_eq!(camera.target,[1e12,-1e12,1e12]);
}

#[test]
fn cpu_screen_queries_keep_distant_pixels_and_original_clip_guards(){
 use crate::snap_math::Screen;let anchor=[1e12,-1e12,1e12];for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.projection=projection;camera.distance=2.;let baseline=Screen::camera(&camera,[640.,480.]);camera.target=anchor;let far=Screen::camera(&camera,[640.,480.]);assert_eq!(baseline.matrix,far.matrix);assert!(baseline!=far);for point in [[0.25,0.125,-0.5],[-0.125,-0.25,0.125],[0.,0.,0.]]{let world=std::array::from_fn(|i|anchor[i]+point[i]);assert_eq!(far.point(world),baseline.point(point));assert_eq!(far.origin,anchor);}}
 let mut view=Screen{matrix:session_rust::Xform::identity().m,size:[200.,100.],origin:anchor};assert_eq!(view.point([anchor[0]+0.25,anchor[1]+0.125,anchor[2]+0.5]),Some([125.,43.75]));for point in [[anchor[0],anchor[1],anchor[2]-0.5],[anchor[0],anchor[1],anchor[2]+1.5],[f64::NAN,0.,0.]]{assert_eq!(view.point(point),None);}view.size[0]=0.;assert_eq!(view.point(anchor),None);view.size[0]=200.;view.origin[0]=f64::NAN;assert_eq!(view.point(anchor),None);view.origin=anchor;view.matrix[15]=0.;assert_eq!(view.point(anchor),None);
}

#[test]
fn text_query_polygons_keep_precise_placements_sources_and_unclipped_rules(){
 use crate::{snap_math::Screen,text::TextPlacement};let anchor=[1e12,-1e12,1e12];for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.projection=projection;camera.distance=3.;let view=Screen::camera(&camera,[800.,600.]);camera.target=anchor;let far_view=Screen::camera(&camera,[800.,600.]);for placement in [TextPlacement::WorldPlane{world:[0.125,0.25,-0.5],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.25},TextPlacement::WorldBillboard{world:[0.125,0.25,-0.5],world_height:0.25},TextPlacement::Anchor{world:[0.125,0.25,-0.5],offset:[3.,5.]},TextPlacement::Nameplate{world:[0.125,0.25,-0.5],padding:[4.,6.],rounded:true}]{let mut scene=crate::scene::Scene::empty();let source=std::rc::Rc::new(crate::scene_label::example(placement));let id=scene.insert_text(crate::scene_label::Prepared::new(source.clone()).unwrap()).unwrap();let baseline=crate::scene_label_screen::polygon_in(&scene.texts()[0],&view).unwrap();let original=serde_json::to_vec(source.as_ref()).unwrap();scene.translate(id,anchor).unwrap();assert_eq!(crate::scene_label_screen::polygon_in(&scene.texts()[0],&far_view).unwrap(),baseline);assert!(std::rc::Rc::ptr_eq(&scene.texts()[0].prepared.source,&source));assert_eq!(serde_json::to_vec(source.as_ref()).unwrap(),original);let bounds=[baseline.iter().map(|p|p[0]).fold(f64::INFINITY,f64::min)-1.,baseline.iter().map(|p|p[1]).fold(f64::INFINITY,f64::min)-1.,baseline.iter().map(|p|p[0]).fold(f64::NEG_INFINITY,f64::max)+1.,baseline.iter().map(|p|p[1]).fold(f64::NEG_INFINITY,f64::max)+1.];let rectangle=crate::rect_select::Rectangle{bounds,crossing:false};assert!(crate::scene_label_screen::matches_in(&scene.texts()[0],&far_view,rectangle));}}
 let mut view=Screen{matrix:session_rust::Xform::identity().m,size:[200.,100.],origin:[0.;3]};assert!(view.unclipped([0.,0.,-1.]).is_some());assert!(view.point([0.,0.,-1.]).is_none());view.matrix[2]=f64::NAN;assert_eq!(view.unclipped([0.25,0.125,0.5]),Some([125.,43.75]));assert!(view.point([0.25,0.125,0.5]).is_none());view.matrix=session_rust::Xform::identity().m;view.matrix[15]=1e-10;assert!(view.unclipped([0.25,0.125,0.5]).is_some());assert!(view.point([0.25,0.125,0.5]).is_none());view.matrix[15]=0.;assert!(view.unclipped([0.;3]).is_none());view.matrix[15]=1.;view.size[0]=0.;assert!(view.unclipped([0.;3]).is_none());
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzl-connect-precise-rectangle-selection-queries-03.rs" skip=6028
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 21.2.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Window and crossing selection use the precise query frame for each supported original family.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Connect precise rectangle selection queries; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Pass the shared query frame to rectangle selection. Transform original mesh, cloud and sheet coordinates through a local placement; reuse the already written local line, path and point accessors. Project those local positions with a zero-origin view of the same matrix. Text bounds retain the original frame so their own placement helper subtracts the origin once.

Original source + query-relative placement → unclipped pixels → window/crossing predicate.



Why does text receive the original query frame while prepared geometry uses a zero-origin view?

Geometry positions are already local when projected. The text polygon owner still needs the world placement and origin to construct its own precise local corners. Subtracting twice would move either result incorrectly.

Study estimate, including typing and experiments: 0.6–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Draw a line, polyline, point and scene text. Select them with left-to-right windows and right-to-left crossing rectangles, then lock or hide one and repeat. Confirm original selections and Undo behavior. Shared GPU rebasing follows after remaining queries and asynchronous picks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 21.2.3
npm --prefix ../session_tests run course -- save 21.2.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Precise rectangle selection is connected. Lasso and widget queries, origin-aware asynchronous picks and shared browser GPU rebasing remain before reversed depth.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 21.2.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
