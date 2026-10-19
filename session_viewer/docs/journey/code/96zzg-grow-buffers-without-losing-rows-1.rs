pub struct GrowBuf {
    pub buf:wgpu::Buffer,
    stride:u64,len:u32,label:&'static str,usage:wgpu::BufferUsages,
}
impl GrowBuf {
    pub fn new(ctx:&GpuCtx,label:&'static str,stride:u64,usage:wgpu::BufferUsages)->Self {
        assert!(stride>0&&stride%4==0);
        let usage=usage|wgpu::BufferUsages::COPY_SRC|wgpu::BufferUsages::COPY_DST;
        Self {buf:ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label:Some(label),size:stride,usage,mapped_at_creation:false,
        }),stride,len:0,label,usage}
    }

    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let limits=ctx.device.limits();
        let limit=if self.usage.contains(wgpu::BufferUsages::STORAGE){
            limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))
        }else{limits.max_buffer_size};
        let most=limit/self.stride;assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
            let cap=(cap+cap/2).max(u64::from(end)).min(most);
            let next=ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label:Some(self.label),size:cap*self.stride,usage:self.usage,mapped_at_creation:false,
            });
            if self.len>0 {
                let mut encoder=ctx.device.create_command_encoder(&Default::default());
                encoder.copy_buffer_to_buffer(&self.buf,0,&next,0,u64::from(self.len)*self.stride);
                ctx.queue.submit([encoder.finish()]);
            }
            self.buf=next;
        }
        ctx.queue.write_buffer(&self.buf,u64::from(self.len)*self.stride,bytes);
        self.len=end;grew
    }

    pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {
        assert_eq!(self.stride,N as u64*4);
        let bytes:Vec<u8>=vertices.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect();
        self.append_bytes(ctx,&bytes);
    }
    pub fn reset(&mut self){self.len=0;}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx,self.label,self.stride,self.usage);}
    pub fn len(&self)->u32{self.len}
    pub fn is_empty(&self)->bool{self.len==0}
}
