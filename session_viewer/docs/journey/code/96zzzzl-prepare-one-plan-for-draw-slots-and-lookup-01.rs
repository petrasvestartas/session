 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change,table:&mut crate::primitive_slot_table::Table)->bool {
  let rows=(1..space.end()).map(|slot|space.local_slot(slot)).collect::<Vec<_>>();
  assert!(change.slots.iter().all(|range|range.start<=range.end&&range.end<=space.end()));
  let mut ranges=change.slots.clone();ranges.extend(changed_runs(&self.previous,&rows).into_iter().map(|run|(run.start+1)as u32..(run.end+1)as u32));
  if rows.len()!=self.previous.len(){ranges.push(0..1);}
  let change=crate::slot_space::Change {all:change.all,slots:crate::slot_space::coalesce(ranges,space.end())};
  if rows.is_empty()&&self.instances()>0{self.clear(ctx,true);}else{self.set(ctx,&rows);}
  table.publish(ctx,space,&change)
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool)