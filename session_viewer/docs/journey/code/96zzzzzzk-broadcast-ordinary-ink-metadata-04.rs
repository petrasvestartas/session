let count=self.data.len()/self.stride as usize;
        if flags.len()!=if self.varying{count}else{count.min(1)}