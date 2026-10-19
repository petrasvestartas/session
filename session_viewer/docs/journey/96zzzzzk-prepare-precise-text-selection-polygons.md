# 21.2.2 · Prepare precise text selection polygons

**Chapter 21 · Precise camera and browser coordinates · Section 21.2 · CPU queries and asynchronous picks** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Share the existing clip transform and pixel conversion between depth-clipped point queries and unclipped region queries. Keep their distinct visibility rules. For text bounds, derive the original row’s local placement before constructing plane corners, anchors, nameplates or billboard scale. The source label remains unchanged.

## Type

Continue from [Project CPU queries around the precise target](96zzzzzj-project-cpu-queries-around-the-precise-target.md). [Save or recover your work](recovery.md).

### 1. `src/snap_math.rs`

Share coordinate conversion while preserving depth, positive-w and logical-size rules.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        if self.size.iter().any(|v| !v.is_finite() || *v <= 0.) { return None; }
        let point=std::array::from_fn::<_,3,_>(|i|point[i]-self.origin[i]);
        let clip: [f64; 4] = std::array::from_fn(|row| point.iter().enumerate().fold(self.matrix[12+row], |sum,(i,v)| sum+self.matrix[i*4+row]*v));
        if clip.iter().any(|v| !v.is_finite()) || clip[3] <= 1e-9 || clip[2] < 0. || clip[2] > clip[3] { return None; }
        let pixel=[(clip[0]/clip[3]+1.)*self.size[0]/2., (1.-clip[1]/clip[3])*self.size[1]/2.];
        pixel.iter().all(|v| v.is_finite()).then_some(pixel)
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn clip(&self,point:[f64;3])->[f64;4] {
        let point=std::array::from_fn::<_,3,_>(|i|point[i]-self.origin[i]);
        let clip: [f64; 4] = std::array::from_fn(|row| point.iter().enumerate().fold(self.matrix[12+row], |sum,(i,v)| sum+self.matrix[i*4+row]*v));
        clip
    }
    pub fn point(&self,point:[f64;3])->Option<[f64;2]> {
        let clip=self.clip(point);
        if clip.iter().any(|v|!v.is_finite())||clip[3]<=1e-9||clip[2]<0.||clip[2]>clip[3] {return None;}
        self.pixel(clip)
    }
    pub fn unclipped(&self,point:[f64;3])->Option<[f64;2]> {
        let clip=self.clip(point);if !clip[3].is_finite()||clip[3]<=0. {return None;}
        self.pixel(clip)
    }
    fn pixel(&self,clip:[f64;4])->Option<[f64;2]> {
        if self.size.iter().any(|v|!v.is_finite()||*v<=0.) {return None;}
        let pixel=[(clip[0]/clip[3]+1.)*self.size[0]/2., (1.-clip[1]/clip[3])*self.size[1]/2.];
        pixel.iter().all(|v| v.is_finite()).then_some(pixel)
    }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-01.rs"
```

</details>

### 2. `src/rect_select.rs`

Reuse the shared projection without adding depth clipping to region selection.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn screen(matrix:&[f64;16],point:[f64;3],size:[f64;2])->Option<[f64;2]> {
    let clip=|row:usize|point.iter().enumerate().map(|(i,value)|matrix[i*4+row]*value).sum::<f64>()+matrix[12+row];let w=clip(3);
    if !w.is_finite()||w<=0.||size.iter().any(|value|!value.is_finite()||*value<=0.) {return None;}
    let point=[(clip(0)/w+1.)*size[0]/2.,(1.-clip(1)/w)*size[1]/2.];point.iter().all(|value|value.is_finite()).then_some(point)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn screen(matrix:&[f64;16],point:[f64;3],size:[f64;2])->Option<[f64;2]> {
    crate::snap_math::Screen {matrix:*matrix,size,origin:[0.;3]}.unclipped(point)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-02.rs"
```

</details>

### 3. `src/scene_label_screen.rs`

Use the explicit query frame instead of a second projection recipe. Derive text coordinates after precise placement subtraction; keep the original source owner. Construct small text-plane corners in the local frame. Compute billboard scale from the same local clip transform. Project the local text anchor through the shared query. Keep the existing window/crossing polygon predicate with an explicit frame.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{scene_label::Row,text::TextPlacement,rect_select::{screen,Rectangle}};

pub fn polygon(row:&Row,matrix:&[f64;16],size:[f64;2])->Option<Vec<[f64;2]>> {
 let label=row.label(false,true).ok()?;let [width,height]=row.prepared.extent;
 if let TextPlacement::WorldPlane {world,right,up,world_height}=label.placement {
  let unit=world_height/label.font_size as f64;let vertical=label.font_size as f64*2./9.;let horizontal=height*0.5+vertical;
  return [[-horizontal,-vertical],[-horizontal,height+vertical],[width+horizontal,height+vertical],[width+horizontal,-vertical]].into_iter().map(|[x,y]|screen(matrix,std::array::from_fn(|i|world[i]+(right[i]*x-up[i]*y)*unit),size)).collect();
 }
 let (world,offset,scale,center,padding)=match label.placement {
  TextPlacement::Anchor {world,offset}=>(world,offset,1.,false,[height*0.5+label.font_size as f64*2./9.,label.font_size as f64*2./9.]),
  TextPlacement::Nameplate {world,padding,rounded}=>(world,[0.;2],1.,true,[if rounded {(padding[0] as f64).max(height*0.5+padding[1] as f64)} else {padding[0] as f64},padding[1] as f64]),
  TextPlacement::WorldBillboard {world,world_height}=>{
   let w=(0..3).map(|i|matrix[i*4+3]*world[i]).sum::<f64>()+matrix[15];let projection=[matrix[1],matrix[5],matrix[9]].iter().map(|n|n*n).sum::<f64>().sqrt();let scale=world_height*projection*size[1]/(2.*w*label.font_size as f64);if !scale.is_finite()||scale<=0. {return None;}(world,[0.;2],scale,false,[height*0.5+label.font_size as f64*2./9.,label.font_size as f64*2./9.])
  },_=>return None,
 };
 let [mut left,mut top]=screen(matrix,world,size)?;left+=offset[0] as f64;top+=offset[1] as f64;if center {left-=width*scale*0.5;top-=height*scale*0.5;}
 let bounds=[left-padding[0]*scale,top-padding[1]*scale,left+(width+padding[0])*scale,top+(height+padding[1])*scale];Some(vec![[bounds[0],bounds[1]],[bounds[0],bounds[3]],[bounds[2],bounds[3]],[bounds[2],bounds[1]]])
}
fn inside(p:[f64;2],points:&[[f64;2]])->bool {let mut sign=0.;for (a,b) in points.iter().zip(points.iter().cycle().skip(1)).take(points.len()) {let cross=(b[0]-a[0])*(p[1]-a[1])-(b[1]-a[1])*(p[0]-a[0]);if cross==0. {continue;}if sign==0. {sign=cross.signum();} else if sign!=cross.signum() {return false;}}sign!=0.}
pub fn matches(row:&Row,matrix:&[f64;16],size:[f64;2],rectangle:Rectangle)->bool {
 let Some(points)=polygon(row,matrix,size) else {return false;};if !rectangle.crossing {return points.iter().all(|p|rectangle.contains(*p));}
 points.iter().any(|p|rectangle.contains(*p))||points.iter().zip(points.iter().cycle().skip(1)).take(points.len()).any(|(a,b)|rectangle.crosses(*a,*b))||inside([rectangle.bounds[0],rectangle.bounds[1]],&points)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{scene_label::Row,text::TextPlacement,rect_select::Rectangle,snap_math::Screen};

pub fn polygon(row:&Row,matrix:&[f64;16],size:[f64;2])->Option<Vec<[f64;2]>> {polygon_in(row,&Screen{matrix:*matrix,size,origin:[0.;3]})}
pub fn polygon_in(row:&Row,view:&Screen)->Option<Vec<[f64;2]>> {
 let row=Row{model:crate::render_space::placed(&row.model,view.origin),..row.clone()};
 let local=Screen{matrix:view.matrix,size:view.size,origin:[0.;3]};let matrix=&local.matrix;let size=local.size;
 let label=row.label(false,true).ok()?;let [width,height]=row.prepared.extent;
 if let TextPlacement::WorldPlane {world,right,up,world_height}=label.placement {
  let unit=world_height/label.font_size as f64;let vertical=label.font_size as f64*2./9.;let horizontal=height*0.5+vertical;
  return [[-horizontal,-vertical],[-horizontal,height+vertical],[width+horizontal,height+vertical],[width+horizontal,-vertical]].into_iter().map(|[x,y]|local.unclipped(std::array::from_fn(|i|world[i]+(right[i]*x-up[i]*y)*unit))).collect();
 }
 let (world,offset,scale,center,padding)=match label.placement {
  TextPlacement::Anchor {world,offset}=>(world,offset,1.,false,[height*0.5+label.font_size as f64*2./9.,label.font_size as f64*2./9.]),
  TextPlacement::Nameplate {world,padding,rounded}=>(world,[0.;2],1.,true,[if rounded {(padding[0] as f64).max(height*0.5+padding[1] as f64)} else {padding[0] as f64},padding[1] as f64]),
  TextPlacement::WorldBillboard {world,world_height}=>{
   let w=local.clip(world)[3];let projection=[matrix[1],matrix[5],matrix[9]].iter().map(|n|n*n).sum::<f64>().sqrt();let scale=world_height*projection*size[1]/(2.*w*label.font_size as f64);if !scale.is_finite()||scale<=0. {return None;}(world,[0.;2],scale,false,[height*0.5+label.font_size as f64*2./9.,label.font_size as f64*2./9.])
  },_=>return None,
 };
 let [mut left,mut top]=local.unclipped(world)?;left+=offset[0] as f64;top+=offset[1] as f64;if center {left-=width*scale*0.5;top-=height*scale*0.5;}
 let bounds=[left-padding[0]*scale,top-padding[1]*scale,left+(width+padding[0])*scale,top+(height+padding[1])*scale];Some(vec![[bounds[0],bounds[1]],[bounds[0],bounds[3]],[bounds[2],bounds[3]],[bounds[2],bounds[1]]])
}
fn inside(p:[f64;2],points:&[[f64;2]])->bool {let mut sign=0.;for (a,b) in points.iter().zip(points.iter().cycle().skip(1)).take(points.len()) {let cross=(b[0]-a[0])*(p[1]-a[1])-(b[1]-a[1])*(p[0]-a[0]);if cross==0. {continue;}if sign==0. {sign=cross.signum();} else if sign!=cross.signum() {return false;}}sign!=0.}
pub fn matches(row:&Row,matrix:&[f64;16],size:[f64;2],rectangle:Rectangle)->bool {matches_in(row,&Screen{matrix:*matrix,size,origin:[0.;3]},rectangle)}
pub fn matches_in(row:&Row,view:&Screen,rectangle:Rectangle)->bool {
 let Some(points)=polygon_in(row,view) else {return false;};if !rectangle.crossing {return points.iter().all(|p|rectangle.contains(*p));}
 points.iter().any(|p|rectangle.contains(*p))||points.iter().zip(points.iter().cycle().skip(1)).take(points.len()).any(|(a,b)|rectangle.crosses(*a,*b))||inside([rectangle.bounds[0],rectangle.bounds[1]],&points)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-03.rs"
```

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-04.rs"
```

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-05.rs"
```

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-06.rs"
```

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-07.rs"
```

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-08.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/camera_space_tests.rs`

Supply exact local/distant plane/billboard/anchor/nameplate polygons and unchanged source owners, retaining distinct unused-depth, tiny-w, finite and size predicates.

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
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzk-prepare-precise-text-selection-polygons-09.rs" skip=3593
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 21.2.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Text selection polygons prepare precise local coordinates using one shared projection recipe.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Prepare precise text selection polygons; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Original text placement − query origin → local polygon → shared pixel projection.



Why subtract the origin before constructing text corners?

Adding small corner offsets to a large world anchor can round them before projection. A local placement keeps those offsets precise, and the shared query owner supplies the matching camera matrix.

Study estimate, including typing and experiments: 0.5–0.8 hours.

</details>

<details>
<summary>Optional experiment</summary>

Create scene text, change its placement and select it with crossing and window rectangles. Confirm the existing zero-origin selection behavior. The new polygon query API prepares the precise region-query connection in the next lesson.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 21.2.2
npm --prefix ../session_tests run course -- save 21.2.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Precise text query bounds and distinct projection predicates are prepared. Live rectangle/lasso/widget queries, asynchronous pick snapshots and shared GPU rebasing remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 21.2.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
