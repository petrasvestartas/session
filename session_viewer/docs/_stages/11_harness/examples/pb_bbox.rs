// Print each point cloud's bounding box from .pb files

fn main() {
    for path in std::env::args().skip(1){
        let bytes = std::fs::read(&path).expect("read");
        let s = session_rust::Session::pb_loads(&bytes).expect("parse");
        for g in s.order(){
            if let Some(session_rust::Geometry::PointCloud(pc)) = s.lookup.get(&g){
                let c = pc.coords();
                let mut mn = [f64::INFINITY; 3];
                let mut mx = [f64::NEG_INFINITY; 3];
                for i in (0..c.len()).step_by(3){
                    for k in 0..3{
                        mn[k] = mn[k].min(c[i+k]);
                        mx[k] = mx[k].max(c[i+k]);
                    }
                    // percentile bounds too: a scane's min/max box is mostly empty air
                    let n = c.len() / 3;
                    let mut pl = [0.0f64; 3];
                    let mut ph = [0.0f64; 3];
                    for k in 0..3 {
                        let mut v: Vec<f64> = (0..n).step_by((n / 20000).max(1)).map(|i| c[i*3 + k]).collect();
                        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        pl[k] = v[v.len() * 2 / 100];
                        ph[k] = v[v.len() * 98 / 100];
                    }
                    println!("{path} {mn:?} {mx:?} p2 {pl:?} p98 {ph:?}");
                }
            }
        }
    }
}