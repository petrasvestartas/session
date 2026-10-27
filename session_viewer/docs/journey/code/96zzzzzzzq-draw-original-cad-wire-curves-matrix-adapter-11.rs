            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass,&self.view_group,self.ink_visibility);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass);self.matrices.draw(&mut pass);pass.set_bind_group(2,&clip_group,&[]);if self.ink_visibility.lines{self.sheets.draw(&mut pass);}
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group,self.ink_visibility);
