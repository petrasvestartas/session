        let part=(self.shape.read)(&frame,&answers,self.choice())?;Ok((answers,part))
    }
    pub fn accept(&mut self,answer:Answer)->Result<Option<Geometry>,String> {
        let (answers,part)=self.with(answer)?;
        let complete=(self.shape.ask)(&answers,&part,self.choice()).is_none();
        let geometry=if complete {Some((self.shape.build)(&part,self.choice())?)}else{None};
        self.answers=answers;self.part=part;Ok(geometry)
    }
}
