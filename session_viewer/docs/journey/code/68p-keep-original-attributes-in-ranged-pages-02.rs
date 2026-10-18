            }

            if field == 5 {
                if length!=fields.coords_len||normals.1!=0 {return None;}
                normals = (body, length);
            }
