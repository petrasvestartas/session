pub fn budget(gpu:wgpu::DeviceType)->Option<u32> {
    match gpu {
        wgpu::DeviceType::DiscreteGpu=>Some(9_000_000),
        wgpu::DeviceType::IntegratedGpu|wgpu::DeviceType::VirtualGpu=>Some(2_500_000),
        wgpu::DeviceType::Other=>Some(4_200_000),
        wgpu::DeviceType::Cpu=>None,
    }
}

pub fn samples(solid:bool,size:[u32;2],forced:Option<u32>,budget:Option<u32>,pixel_scale:f32,reduced:bool)->u32 {
    if reduced {return 1;}
    if let Some(count)=forced {return if count==4 {4}else{1};}
    if pixel_scale>=2. {return 1;}
    let pixels=u64::from(size[0])*u64::from(size[1]);
    if solid&&budget.is_some_and(|max|pixels<=u64::from(max)) {4}else{1}
}

#[cfg(test)]
mod tests {
 #[test]
 fn policy_uses_device_budgets_actual_density_and_explicit_recovery_precedence() {
  use super::*;
  for (device,max)in [(wgpu::DeviceType::DiscreteGpu,9_000_000),(wgpu::DeviceType::IntegratedGpu,2_500_000),(wgpu::DeviceType::VirtualGpu,2_500_000),(wgpu::DeviceType::Other,4_200_000)] {
   assert_eq!(budget(device),Some(max));assert_eq!(samples(true,[max,1],None,budget(device),1.,false),4);assert_eq!(samples(true,[max+1,1],None,budget(device),1.,false),1);
  }
  assert_eq!(budget(wgpu::DeviceType::Cpu),None);assert_eq!(samples(true,[900,760],None,None,1.,false),1);assert_eq!(samples(false,[900,760],None,Some(9_000_000),1.,false),1);assert_eq!(samples(true,[u32::MAX,u32::MAX],None,Some(u32::MAX),1.,false),1);
  assert_eq!(samples(true,[900,760],None,Some(9_000_000),1.999,false),4);assert_eq!(samples(true,[900,760],None,Some(9_000_000),2.,false),1);
  assert_eq!(samples(false,[u32::MAX,u32::MAX],Some(4),None,3.,false),4);assert_eq!(samples(true,[900,760],Some(4),Some(9_000_000),1.,true),1);
  for forced in [0,1,2,3,5,8,u32::MAX] {assert_eq!(samples(true,[900,760],Some(forced),Some(9_000_000),1.,false),1);}
 }
}
