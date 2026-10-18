    fn max_rows(&self,ctx:&GpuCtx)->u64 {
        let limits=ctx.device.limits();let bytes=if self.usage.contains(wgpu::BufferUsages::STORAGE){limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))}else{limits.max_buffer_size};bytes/self.stride
    }
    pub fn write_or_append(&mut self,ctx:&GpuCtx,at:u32,bytes:&[u8])->bool {
        assert_eq!(bytes.len()as u64%self.stride,0);assert!(at<=self.len);
        let rows=u32::try_from(bytes.len()as u64/self.stride).expect("Bounded GPU records");let end=at.checked_add(rows).expect("GPU row count");assert!(u64::from(end)<=self.max_rows(ctx));
        let inside=usize::try_from(u64::from((self.len-at).min(rows))*self.stride).unwrap();
        self.write_bytes_at(ctx,at,&bytes[..inside]);self.append_bytes(ctx,&bytes[inside..])
    }
    pub fn append_bytes(