            LaneId::Verts => 44,               // vertex plus its object row; register:meshes
            LaneId::Faces => 5,                // index plus a third of a face id; register:meshes
            LaneId::Print | LaneId::Text => 4, // register:meshes
            LaneId::Sources => 16,             // register:meshes
