            self.printed.draw(&mut pass,&self.view_group,&clip_group,&self.meshes,false);
            self.highlight.ink(&mut pass,&self.view_group,self.ink_visibility);