    let count=count_at(at,sample);
    let inside=select(0u,1u<<sample,count>0.5);
    let selected=count>SELECTED_CROSSING-0.5;
