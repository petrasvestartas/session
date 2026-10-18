                    "view normals off" => Action::NormalView(false),
                    "example sharp" => Action::AddNormals(crate::normal_example::Mode::Sharp),
                    "example smooth" => Action::AddNormals(crate::normal_example::Mode::Smooth),
                    "example winding" => Action::AddNormals(crate::normal_example::Mode::Winding),
