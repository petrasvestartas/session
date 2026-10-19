ol)->String{source_with_samples(stride,pick,grid,varying,1)}
pub fn source_with_samples(stride:u64,pick:bool,grid:bool,varying:bool,samples:u32)->String{
 assert!(matches!(samples,1|4)&&(!pick||samples==1));
 let(base,i