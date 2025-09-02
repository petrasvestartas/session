use session_rust::{Line, Mesh, PointCloud, Arrow, Point, Vector, Color, Xform, AllGeometryData};
use session_rust::json_serialization::{FromJsonData, JsonHandler};

// Minimal star polygon mesh (concave, non-self-intersecting)
fn make_star_mesh() -> Mesh {
    let polygon = vec![
        Point::new(0.12821, 0.514321, 3.0),
        Point::new(-0.103219, 0.282757, 3.0),
        Point::new(-0.430101, 0.264609, 3.0),
        Point::new(-0.281387, -0.02705, 3.0),
        Point::new(-0.365139, -0.343542, 3.0),
        Point::new(-0.041799, -0.292234, 3.0),
        Point::new(0.233322, -0.469688, 3.0),
        Point::new(0.284442, -0.146318, 3.0),
        Point::new(0.538228, 0.0605, 3.0),
        Point::new(0.246482, 0.209046, 3.0),
    ];
    let mut mesh = Mesh::from_polygons(vec![polygon], None);
    for vd in mesh.vertex.values_mut() {
        vd.set_color(1.0, 0.84, 0.0);
    }
    // Set edge color and thickness using existing data field
    mesh.facecolors = [0.0, 1.0, 0.0, 1.0]; // Bright green (RGBA 0.0-1.0)
    mesh.width = 0.3; // Thicker edges for better visibility
    mesh
}

fn make_cube_mesh() -> Mesh{
    let cube_faces = vec![
        // Bottom face (z=0) - CCW when viewed from below (outward normal -Z)
        vec![
            Point::new(2.0, 0.0, 0.0),
            Point::new(2.0, 1.0, 0.0),
            Point::new(3.0, 1.0, 0.0),
            Point::new(3.0, 0.0, 0.0),
        ],
        // Top face (z=1) - CCW when viewed from above (outward normal +Z)
        vec![
            Point::new(2.0, 0.0, 1.0),
            Point::new(3.0, 0.0, 1.0),
            Point::new(3.0, 1.0, 1.0),
            Point::new(2.0, 1.0, 1.0),
        ],
        // Front face (y=0) - CCW when viewed from front (outward normal -Y)
        vec![
            Point::new(2.0, 0.0, 0.0),
            Point::new(3.0, 0.0, 0.0),
            Point::new(3.0, 0.0, 1.0),
            Point::new(2.0, 0.0, 1.0),
        ],
        // Back face (y=1) - CCW when viewed from back (outward normal +Y)
        vec![
            Point::new(3.0, 1.0, 0.0),
            Point::new(2.0, 1.0, 0.0),
            Point::new(2.0, 1.0, 1.0),
            Point::new(3.0, 1.0, 1.0),
        ],
        // Left face (x=2) - CCW when viewed from left (outward normal -X)
        vec![
            Point::new(2.0, 1.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
            Point::new(2.0, 0.0, 1.0),
            Point::new(2.0, 1.0, 1.0),
        ],
        // Right face (x=3) - CCW when viewed from right (outward normal +X)
        vec![
            Point::new(3.0, 0.0, 0.0),
            Point::new(3.0, 1.0, 0.0),
            Point::new(3.0, 1.0, 1.0),
            Point::new(3.0, 0.0, 1.0),
        ],
    ];
    let mut cube = Mesh::from_polygons(cube_faces, None);
    
    // Check edges before serialization
    println!("🔍 BEFORE serialization - Cube edges:");
    println!("  Number of faces: {}", cube.face.len());
    println!("  Number of vertices: {}", cube.vertex.len());
    for (face_key, face_vertices) in &cube.face {
        println!("  Face {}: {:?}", face_key, face_vertices);
    }
    
    let edges_before = cube.extract_edges_as_lines();
    println!("  Total unique edges: {}", edges_before.len());
    
    // Set edge color and thickness using existing data field
    cube.facecolors = [0.0, 0.0, 1.0, 1.0]; // Blue (RGBA 0.0-1.0)
    cube.width = 0.3; // Thicker edges for better visibility
    cube
}

fn make_dodecahedron_mesh() -> Mesh {
    // Golden ratio
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let edge_length = 1.0f32; // L = 2.0
    let a = edge_length / 2.0;
    let b = a * phi;
    let c = a + b;
    
    // 20 vertices of regular dodecahedron (moved +3 units on Y axis)
    let vertices = vec![
        Point::new(-b, -b + 3.0, -b), // 0
        Point::new( b, -b + 3.0, -b), // 1
        Point::new(-b,  b + 3.0, -b), // 2
        Point::new( b,  b + 3.0, -b), // 3
        Point::new(-b, -b + 3.0,  b), // 4
        Point::new( b, -b + 3.0,  b), // 5
        Point::new(-b,  b + 3.0,  b), // 6
        Point::new( b,  b + 3.0,  b), // 7
        Point::new( c, -a + 3.0,  0.0), // 8
        Point::new( c,  a + 3.0,  0.0), // 9
        Point::new(-c, -a + 3.0,  0.0), // 10
        Point::new(-c,  a + 3.0,  0.0), // 11
        Point::new( a,  0.0 + 3.0, -c), // 12
        Point::new(-a,  0.0 + 3.0, -c), // 13
        Point::new( a,  0.0 + 3.0,  c), // 14
        Point::new(-a,  0.0 + 3.0,  c), // 15
        Point::new( 0.0, -c + 3.0, -a), // 16
        Point::new( 0.0, -c + 3.0,  a), // 17
        Point::new( 0.0,  c + 3.0, -a), // 18
        Point::new( 0.0,  c + 3.0,  a), // 19
    ];
    
    // 12 pentagonal faces (counterclockwise when viewed from outside)
    let faces = vec![
        vec![vertices[1].clone(), vertices[12].clone(), vertices[3].clone(), vertices[9].clone(), vertices[8].clone()],   // Face 0
        vec![vertices[5].clone(), vertices[8].clone(), vertices[9].clone(), vertices[7].clone(), vertices[14].clone()],   // Face 1
        vec![vertices[0].clone(), vertices[10].clone(), vertices[11].clone(), vertices[2].clone(), vertices[13].clone()], // Face 2
        vec![vertices[4].clone(), vertices[15].clone(), vertices[6].clone(), vertices[11].clone(), vertices[10].clone()], // Face 3
        vec![vertices[1].clone(), vertices[16].clone(), vertices[0].clone(), vertices[13].clone(), vertices[12].clone()], // Face 4
        vec![vertices[3].clone(), vertices[12].clone(), vertices[13].clone(), vertices[2].clone(), vertices[18].clone()], // Face 5
        vec![vertices[5].clone(), vertices[14].clone(), vertices[15].clone(), vertices[4].clone(), vertices[17].clone()], // Face 6
        vec![vertices[7].clone(), vertices[19].clone(), vertices[6].clone(), vertices[15].clone(), vertices[14].clone()], // Face 7
        vec![vertices[1].clone(), vertices[8].clone(), vertices[5].clone(), vertices[17].clone(), vertices[16].clone()],  // Face 8
        vec![vertices[0].clone(), vertices[16].clone(), vertices[17].clone(), vertices[4].clone(), vertices[10].clone()], // Face 9
        vec![vertices[3].clone(), vertices[18].clone(), vertices[19].clone(), vertices[7].clone(), vertices[9].clone()],  // Face 10
        vec![vertices[2].clone(), vertices[11].clone(), vertices[6].clone(), vertices[19].clone(), vertices[18].clone()], // Face 11
    ];
    
    let mut dodecahedron = Mesh::from_polygons(faces, None);
    // Set edge color and thickness using existing data field
    dodecahedron.facecolors = [0.39, 0.39, 0.39, 1.0]; // Gray (RGBA 0.0-1.0)
    dodecahedron.width = 0.3; // Thicker edges for better visibility
    
    dodecahedron
}

fn make_point_cloud() -> PointCloud {
    
    let mut points = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    
    // Generate 10x10x10 = 1,000 points within 10x10x10 bounds
    let grid_size = 50;
    let bound = 1.0f32; // -5 to +5 = 10 units
    let step = (2.0 * bound) / (grid_size as f32 - 1.0);
    
    for i in 0..grid_size {
        for j in 0..grid_size {
            for k in 0..grid_size {
                let x = -bound + (i as f32) * step;
                let y = -bound + (j as f32) * step + 5.0;
                let z = -bound + (k as f32) * step;
                
                let mut point = Point::new(x, y, z);
                
                // Apply colors based on position (rainbow gradient) in 0-1 range
                let r = i as f32 / grid_size as f32;
                let g = j as f32 / grid_size as f32;
                let b = k as f32 / grid_size as f32;
                point.pointcolor = Color::from_float(r, g, b, 1.0);
                
                points.push(point);
                
                // Generate upward-pointing normals
                normals.push(Vector::new(0.0, 0.0, 1.0));
                
                // Generate colors based on position (rainbow gradient)
                let r_255 = ((i as f32 / grid_size as f32) * 255.0) as u8;
                let g_255 = ((j as f32 / grid_size as f32) * 255.0) as u8;
                let b_255 = ((k as f32 / grid_size as f32) * 255.0) as u8;
                colors.push(Color::new(r_255, g_255, b_255, 255));
            }
        }
    }
    
    
    // Extract pointcolors before moving points
    let pointcolors: Vec<[f32; 4]> = points.iter().map(|p| p.pointcolor.to_float_array()).collect();
    
    // Create a point cloud with transformation matrix and colors
    let mut point_cloud = PointCloud::new(points, normals);
    
    // Set the pointcolors vector
    point_cloud.pointcolors = pointcolors;
    
    // Apply a transformation: translate by (2, 0, 1) and rotate 45 degrees around Z-axis
    let cos_45 = 0.7071067811865476f32; // cos(45°)
    let sin_45 = 0.707_106_77_f32; // sin(45°)
    
    point_cloud.xform = Xform::from_matrix([
        cos_45*0.0+1.0, -sin_45*0.0, 0.0, 0.0,  // Rotate + translate X
        sin_45*0.0,  cos_45*0.0+1.0, 0.0, 0.0,  // Rotate + translate Y  
        0.0,     0.0,    1.0, 0.0,  // No rotation in Z + translate Z
        0.0,     0.0,    0.0, 1.0   // Homogeneous coordinate
    ]);
    
    point_cloud
}

fn make_lines() -> Vec<Line> {
    // Create lines with varying thickness and color
    let mut lines: Vec<Line> = Vec::new();

    // Grid lines with default thickness
    let size: i32 = 40; // -5..=5 => 11 lines => 10x10 cells
    let thickness = 0.1;

    // Horizontal lines (vary X) - red for x-axis (y=0), black for others
    for i in -size..=size {
        let y = i as f32;
        let mut line = Line::from_points(&Point::new(-(size as f32), y, 0.0), &Point::new(size as f32, y, 0.0));
        line.width = thickness;
        line.linecolor = [0.5, 0.5, 0.5, 1.0]; // Gray
        lines.push(line);
    }
    // Vertical lines (vary Y) - green for y-axis (x=0), black for others
    for i in -size..=size {
        let x = i as f32;
        let mut line = Line::from_points(&Point::new(x, -(size as f32), 0.0), &Point::new(x, size as f32, 0.0));
        line.width = thickness;
        line.linecolor = [0.5, 0.5, 0.5, 1.0]; // Gray
        lines.push(line);
    }
    let axes_scale = 2.0;
    let mut line_x = Line::from_points(&Point::new(0.0, 0.0, 0.0), &Point::new(size as f32, 0.0, 0.0));
    line_x.width = thickness*axes_scale;
    line_x.linecolor = [1.0, 0.0, 0.0, 1.0]; // Red for x-axis
    lines.push(line_x);
    let mut line_y = Line::from_points(&Point::new(0.0, 0.0, 0.0), &Point::new(0.0, size as f32, 0.0));
    line_y.width = thickness*axes_scale;
    line_y.linecolor = [0.0, 1.0, 0.0, 1.0]; // Green for y-axis
    lines.push(line_y);
    let mut line_z = Line::from_points(&Point::new(0.0, 0.0, 0.0), &Point::new(0.0, 0.0, size as f32));
    line_z.width = thickness*axes_scale;
    line_z.linecolor = [0.0, 0.0, 1.0, 1.0]; // Blue for z-axis
    lines.push(line_z);

    lines
}

fn make_arrows() -> Vec<Arrow> {
    let mut arrows: Vec<Arrow> = Vec::new();
    let thickness = 0.3;
    
    // Create arrows at origin with normal size and bright colors for visibility
    let mut arrow_x = Arrow::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
    arrow_x.facecolor = [1.0, 0.0, 0.0, 1.0]; // Red
    arrow_x.width = thickness;
    arrows.push(arrow_x);
    
    let mut arrow_y = Arrow::new(0.0, 0.0, 0.0, 0.0, 10.0, 0.0);
    arrow_y.facecolor = [0.0, 1.0, 0.0, 1.0]; // Green
    arrow_y.width = thickness;
    arrows.push(arrow_y);
    
    let mut arrow_z = Arrow::new(0.0, 0.0, 0.0, 0.0, 0.0, 10.0);
    arrow_z.facecolor = [0.0, 0.0, 1.0, 1.0]; // Blue
    arrow_z.width = thickness;
    arrows.push(arrow_z);
    
    arrows
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting geometry generation...");
    
    let star = make_star_mesh();
    let _sphere = Mesh::create_unit_sphere_high_res();
    let cube = make_cube_mesh();
    let dodecahedron = make_dodecahedron_mesh();
    let point_cloud = make_point_cloud();
    let lines = make_lines();
    let arrows = make_arrows();

    let all_geometry = AllGeometryData {
        points: vec![],
        vectors: vec![],
        lines, // Restore lines but without RGB axis lines
        arrows,
        planes: vec![],
        colors: vec![],
        point_clouds: vec![point_cloud],
        line_clouds: vec![],
        plines: vec![],
        xforms: vec![],
        meshes: vec![star, cube, dodecahedron], //sphere
        mesh_instances: vec![],
        pipe_mesh_index: None,
        sphere_mesh_index: None,
    };

    // Write directly to wink data folder using COMPAS-style JSON serialization
    let parent_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let data_dir = format!("{}/wink/data", parent_dir.display());
    // Write all geometry to JSON file
    let json_handler = JsonHandler::new();
    let output_path = "/Users/petras/rust/wink/data/session.json";
    println!("Writing to: {}", output_path);
    
    match json_handler.save_object(&all_geometry, output_path) {
        Ok(_) => {
            println!("File written successfully!");
            
            // Now test deserialization to check edges after loading
            println!("\n🔍 AFTER serialization - Testing deserialization:");
            match json_handler.load_geometry(output_path) {
                Ok(loaded_json) => {
                    if let Some(loaded_geometry) = AllGeometryData::from_json_data(&loaded_json) {
                        // Find the cube mesh in loaded geometry
                        for mesh in &loaded_geometry.meshes {
                            // Check if this is the cube (has blue color)
                            if mesh.facecolors[2] == 1.0 { // Blue cube
                                println!("  Loaded cube mesh:");
                                println!("  Number of faces: {}", mesh.face.len());
                                println!("  Number of vertices: {}", mesh.vertex.len());
                                for (face_key, face_vertices) in &mesh.face {
                                    println!("  Face {}: {:?}", face_key, face_vertices);
                                }
                                
                                let edges_after = mesh.extract_edges_as_lines();
                                println!("  Total unique edges: {}", edges_after.len());
                                break;
                            }
                        }
                    }
                },
                Err(e) => println!("Error reading file: {}", e),
            }
        },
        Err(e) => println!("Error writing file: {}", e),
    }

    // Create minimal metadata file for efficient change detection
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let metadata_path = format!("{data_dir}/session_metadata.json");
    std::fs::write(&metadata_path, timestamp.to_string())?;
    
    println!("File written successfully!");
    
    Ok(())
}