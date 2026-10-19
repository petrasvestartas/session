        }
    }
    if let Some(root) = message.tree.as_ref().and_then(|tree| tree.root.as_ref()) {
        crate::hierarchy::validate(root,&guids)?;
    }
    Ok(())
}
