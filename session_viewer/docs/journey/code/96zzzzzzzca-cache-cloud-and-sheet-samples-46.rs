use super::*;
use std::rc::Rc;
#[test]
fn actual_cloud_composite_matches_current_opaque_and_eye_dome_pixels() {
    let mut records=Vec::new();let mut alpha_differences=0;let mut shaded_differences=0;
    for edges in [false,true] {
    for alpha in [0u8,255] {
        let mut coords=Vec::new();let mut colors=Vec::new();
        for y in 0..30 {for x in 0..40 {coords.extend([if edges {-1.05+x as f64*0.055}else{-0.8+x as f64*0.04},if edges {-1.02+y as f64*0.07}else{-0.6+y as f64*0.04},if (x/4+y/4)%2==0 {0.15}else{0.35}]);colors.extend([80,140,220,i32::from(alpha)]);}}
        let mut cloud=session_rust::PointCloud::from_coords(coords,colors,vec![]);cloud.point_size=10.;let cloud=Rc::new(cloud);let _=cloud.guid();let original=cloud.to_proto();
        let mut scene=Scene::empty();let owner=scene.insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap()).unwrap();let mut r=crate::cap_gpu_tests::renderer(&scene);let ctx=r.text_context();let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));
        for samples in [1,4] {
            super::cloud_address_tests::retarget(&mut r,samples);let default_pixels=crate::cap_gpu_tests::pixels_at(&r,matrix);let usage=r.cloud_usage();
            for edl in [0.,0.25] {
                r.clouds.borrow_mut().as_mut().unwrap().set_edl(&ctx,edl);
                let full=crate::cap_gpu_tests::pixels_at(&r,matrix);
                let lane=r.clouds.borrow();let lane=lane.as_ref().unwrap();
                let actual=crate::cloud_gpu::composite_reference::pixels(lane,&ctx,samples,false,edl);
                let reference=crate::cloud_gpu::composite_reference::pixels(lane,&ctx,samples,true,edl);
                assert_eq!(full,reference,"Whole rendered cloud equals independent maintained compositor at both counts and EDL modes");
                assert_eq!(actual,reference,"Every RGBA channel matches exact maintained WGSL");
                let rgba=actual.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a!=b).count();
                let rgb=actual.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a[..3]!=b[..3]).count();
                let a=actual.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a[3]!=b[3]).count();
                if alpha==0 && edl==0. {assert_eq!(a,0);alpha_differences+=a;}
                if alpha==255 && edl>0. {assert_eq!(rgb,0);shaded_differences+=rgb;}
                if alpha==255 && edl==0. {assert_eq!(rgba,0);}
                if edl==0.25 {let default_diff=default_pixels.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a!=b).count();assert_eq!(default_diff,0,"Default whole viewer uses maintained eye-dome shading and opaque cloud output");}
                if !edges&&((alpha==0&&edl==0.&&samples==1)||(alpha==255&&edl>0.&&(samples==1||samples==4))) {for(name,bytes)in[("actual",&actual),("reference",&reference)] {crate::sample_test_proof::write(format!("cloud-parity-{alpha}-{samples}-{name}.rgba"),bytes).unwrap();}}
                records.push(serde_json::json!({"viewportEdges":edges,"sourceAlpha":alpha,"samples":samples,"edl":edl,"rgbaDifferences":rgba,"rgbDifferences":rgb,"alphaDifferences":a}));
            }
            assert_eq!(&r.cloud_usage()[..4],&usage[..4]);r.request_pick([0.,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(cloud.to_proto(),original);
        }
    }
    }
    assert_eq!((alpha_differences,shaded_differences),(0,0));crate::sample_test_proof::write("cloud-parity-observed.json",serde_json::to_vec_pretty(&records).unwrap()).unwrap();
}
