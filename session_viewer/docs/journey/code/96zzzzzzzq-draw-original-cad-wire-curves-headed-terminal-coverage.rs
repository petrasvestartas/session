   for(i,(cut,round))in original_pixels.chunks_exact(4).zip(explicit.chunks_exact(4)).enumerate(){let x=i%size[0]as usize;let y=i/size[0]as usize;
    if x<size[0]as usize/2{assert_eq!(cut,[255,255,255,255],"A camera cut has no invented source cap or clipped arrowhead");}
    if cut!=round{changed+=1;let radius=2.*density as f64+0.70711;let cy=y as f64+0.5-size[1]as f64/2.;let start=(x as f64+0.5-size[0]as f64/2.).hypot(cy)<radius;let end=(x as f64+0.5-size[0]as f64*0.9).hypot(cy)<radius;
     assert!(start||(kind==1&&end),"Every body pixel matches; differences stay inside production cap/filter support: density={density}, kind={kind}, x={x}, y={y}, cut={cut:?}, round={round:?}");
     if kind==0{assert!(x<size[0]as usize/2,"A plain line retains the same genuine far-end coverage");}
    }
   }
