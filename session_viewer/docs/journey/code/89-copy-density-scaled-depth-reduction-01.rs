pub fn resolution(full: (u32, u32), dpr: f64) -> (u32, u32) {
    let scale = (1.0 / dpr.max(1.0)).clamp(0.25, 0.5);
    (
        (f64::from(full.0) * scale).ceil().max(1.0) as u32,
        (f64::from(full.1) * scale).ceil().max(1.0) as u32,
    )
}

#[cfg(test)]
mod tests{
 #[test]fn resolution_follows_device_pixels_without_a_navigation_mode(){
 assert_eq!(super::resolution((1920,1080),1.),(960,540));assert_eq!(super::resolution((3840,2160),2.),(1920,1080));assert_eq!(super::resolution((1179,2556),3.),(393,852));assert_eq!(super::resolution((1920,1080),5.),(480,270));
 }
}
