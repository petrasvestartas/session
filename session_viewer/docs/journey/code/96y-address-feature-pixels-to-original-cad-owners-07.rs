            let node=&doc.nodes[index];
            let members=doc.nodes[index..node.end].iter().filter_map(|n|n.object).filter(|id|scene.contains(*id)).collect::<Vec<_>>();
            let ids=doc.nodes[index..node.end].iter().filter(|n|node.readonly || !n.derived).filter_map(|n|n.object).filter(|id|scene.contains(*id)).collect::<Vec<_>>();