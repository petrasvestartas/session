            }
            Edit::Saved(bytes) => {
                let mut restored=crate::scene::Scene::demo();crate::scene_document::open(&mut restored,&bytes).unwrap();
                assert_eq!(restored.objects().len(), e.scene.objects().len());
            }
        }
