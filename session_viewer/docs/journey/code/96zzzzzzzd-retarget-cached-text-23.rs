rget) {
        if target.format!=self.target.format {self.pipeline=crate::sample_pipelines::Variants::new(pipeline(ctx,target,&self.layout,false),target.samples);}
        else {self.pipeline.select(target.samples,||pipeline(ctx,target,&self.layout,false));}
        