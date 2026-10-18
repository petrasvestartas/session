pixel);
 if(ink_primitive(source_at,sample)==0u&&ink_depth(source_at,sample)>axis.depth+tolerance){return false;}
 if(clip