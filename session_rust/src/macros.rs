// Macro implementations for common geometry structs

/// Macro for creating a Point with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use session_rust::primitives::Point;
/// use session_rust::point;
/// 
/// // Default point (origin)
/// let p1 = point![];
/// assert_eq!(p1.x, 0.0);
/// assert_eq!(p1.y, 0.0);
/// assert_eq!(p1.z, 0.0);
/// 
/// // Point with coordinates
/// let p2 = point![1.0, 2.0, 3.0];
/// assert_eq!(p2.x, 1.0);
/// assert_eq!(p2.y, 2.0);
/// assert_eq!(p2.z, 3.0);
/// 
/// // Point with custom name
/// let p3 = point![4.0, 5.0, 6.0, name: "CustomPoint"];
/// assert_eq!(p3.x, 4.0);
/// assert_eq!(p3.y, 5.0);
/// assert_eq!(p3.z, 6.0);
/// assert_eq!(p3.name, "CustomPoint");
/// ```
#[macro_export]
macro_rules! point {
    // Empty pattern - default point at origin
    () => {
        Point::new(0.0, 0.0, 0.0)
    };
    
    // Basic pattern with x, y, z coordinates
    ($x:expr, $y:expr, $z:expr) => {
        Point::new($x, $y, $z)
    };
    
    // Pattern with custom name
    ($x:expr, $y:expr, $z:expr, name: $name:expr) => {
        {
            let mut pt = Point::new($x, $y, $z);
            pt.name = $name.to_string();
            pt
        }
    };
}

/// Macro for creating a Vector with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use session_rust::primitives::Vector;
/// use session_rust::vector;
/// 
/// // Default vector (zero vector)
/// let v1 = vector![];
/// assert_eq!(v1.x, 0.0);
/// assert_eq!(v1.y, 0.0);
/// assert_eq!(v1.z, 0.0);
/// 
/// // Vector with components
/// let v2 = vector![1.0, 2.0, 3.0];
/// assert_eq!(v2.x, 1.0);
/// assert_eq!(v2.y, 2.0);
/// assert_eq!(v2.z, 3.0);
/// 
/// // Predefined axes
/// let x_axis = vector![unit_x];
/// assert_eq!(x_axis.x, 1.0);
/// assert_eq!(x_axis.y, 0.0);
/// assert_eq!(x_axis.z, 0.0);
/// 
/// let y_axis = vector![unit_y];
/// assert_eq!(y_axis.x, 0.0);
/// assert_eq!(y_axis.y, 1.0);
/// assert_eq!(y_axis.z, 0.0);
/// 
/// let z_axis = vector![unit_z];
/// assert_eq!(z_axis.x, 0.0);
/// assert_eq!(z_axis.y, 0.0);
/// assert_eq!(z_axis.z, 1.0);
/// ```
#[macro_export]
macro_rules! vector {
    // Empty pattern - default zero vector
    () => {
        Vector::new(0.0, 0.0, 0.0)
    };
    
    // Basic pattern with x, y, z components
    ($x:expr, $y:expr, $z:expr) => {
        Vector::new($x, $y, $z)
    };
    
    // Predefined axes
    (unit_x) => {
        Vector::unit_x()
    };
    
    (unit_y) => {
        Vector::unit_y()
    };
    
    (unit_z) => {
        Vector::unit_z()
    };
}

/// Macro for creating a Line with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use session_rust::primitives::Line;
/// use session_rust::line;
/// 
/// // Default line
/// let l1 = line![];
/// assert_eq!(l1.x0, 0.0);
/// assert_eq!(l1.y0, 0.0);
/// assert_eq!(l1.z0, 0.0);
/// assert_eq!(l1.x1, 1.0);
/// assert_eq!(l1.y1, 0.0);
/// assert_eq!(l1.z1, 0.0);
/// 
/// // Line with coordinates
/// let l2 = line![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
/// assert_eq!(l2.x0, 0.0);
/// assert_eq!(l2.y0, 0.0);
/// assert_eq!(l2.z0, 0.0);
/// assert_eq!(l2.x1, 1.0);
/// assert_eq!(l2.y1, 1.0);
/// assert_eq!(l2.z1, 1.0);
/// 
/// // Line with custom name
/// let l3 = line![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, name: "CustomLine"];
/// assert_eq!(l3.x0, 1.0);
/// assert_eq!(l3.y0, 2.0);
/// assert_eq!(l3.z0, 3.0);
/// assert_eq!(l3.x1, 4.0);
/// assert_eq!(l3.y1, 5.0);
/// assert_eq!(l3.z1, 6.0);
/// assert_eq!(l3.name, "CustomLine");
/// ```
#[macro_export]
macro_rules! line {
    // Empty pattern - default line along X axis
    () => {
        Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    };
    
    // Line from coordinates
    ($x0:expr, $y0:expr, $z0:expr, $x1:expr, $y1:expr, $z1:expr) => {
        Line::new($x0, $y0, $z0, $x1, $y1, $z1)
    };
    
    // Line with custom name
    ($x0:expr, $y0:expr, $z0:expr, $x1:expr, $y1:expr, $z1:expr, name: $name:expr) => {
        {
            let mut line = Line::new($x0, $y0, $z0, $x1, $y1, $z1);
            line.name = $name.to_string();
            line
        }
    };
}

/// Macro for creating a Plane with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use session_rust::primitives::{Plane, Point, Vector};
/// use session_rust::plane;
/// 
/// // Default plane (XY plane at origin)
/// let p1 = plane![];
/// assert_eq!(p1.origin.x, 0.0);
/// assert_eq!(p1.origin.y, 0.0);
/// assert_eq!(p1.origin.z, 0.0);
/// assert_eq!(p1.xaxis.x, 1.0);
/// assert_eq!(p1.xaxis.y, 0.0);
/// assert_eq!(p1.xaxis.z, 0.0);
/// assert_eq!(p1.yaxis.x, 0.0);
/// assert_eq!(p1.yaxis.y, 1.0);
/// assert_eq!(p1.yaxis.z, 0.0);
/// assert_eq!(p1.zaxis.x, 0.0);
/// assert_eq!(p1.zaxis.y, 0.0);
/// assert_eq!(p1.zaxis.z, 1.0);
/// 
/// // Plane with specified origin, x-axis and y-axis
/// let p2 = plane![origin: (1.0, 2.0, 3.0), xaxis: (1.0, 0.0, 0.0), yaxis: (0.0, 1.0, 0.0)];
/// assert_eq!(p2.origin.x, 1.0);
/// assert_eq!(p2.origin.y, 2.0);
/// assert_eq!(p2.origin.z, 3.0);
/// assert_eq!(p2.xaxis.x, 1.0);
/// assert_eq!(p2.yaxis.y, 1.0);
/// assert_eq!(p2.zaxis.z, 1.0);
/// 
/// // Plane from existing point and vectors
/// let point = Point::new(4.0, 5.0, 6.0);
/// let x_vec = Vector::new(1.0, 0.0, 0.0);
/// let y_vec = Vector::new(0.0, 1.0, 0.0);
/// let p3 = plane![origin: point, xaxis: x_vec, yaxis: y_vec];
/// assert_eq!(p3.origin.x, 4.0);
/// assert_eq!(p3.origin.y, 5.0);
/// assert_eq!(p3.origin.z, 6.0);
/// assert_eq!(p3.xaxis.x, 1.0);
/// assert_eq!(p3.yaxis.y, 1.0);
/// assert_eq!(p3.zaxis.z, 1.0);
/// 
/// // Plane with custom name
/// let p4 = plane![origin: (1.0, 1.0, 1.0), xaxis: (1.0, 0.0, 0.0), yaxis: (0.0, 1.0, 0.0), name: "CustomPlane"];
/// assert_eq!(p4.origin.x, 1.0);
/// assert_eq!(p4.xaxis.x, 1.0);
/// assert_eq!(p4.name, "CustomPlane");
/// ```
#[macro_export]
macro_rules! plane {
    // Empty pattern - default XY plane at origin
    () => {
        Plane::default()
    };
    
    // Basic pattern with origin point, x-axis and y-axis
    (origin: ($x:expr, $y:expr, $z:expr), xaxis: ($xx:expr, $xy:expr, $xz:expr), yaxis: ($yx:expr, $yy:expr, $yz:expr)) => {
        Plane::new(
            Point::new($x, $y, $z), 
            Vector::new($xx, $xy, $xz), 
            Vector::new($yx, $yy, $yz)
        )
    };
    
    // From existing point and vectors
    (origin: $origin:expr, xaxis: $xaxis:expr, yaxis: $yaxis:expr) => {
        Plane::new($origin, $xaxis, $yaxis)
    };
    
    // Pattern with custom name
    (origin: ($x:expr, $y:expr, $z:expr), xaxis: ($xx:expr, $xy:expr, $xz:expr), yaxis: ($yx:expr, $yy:expr, $yz:expr), name: $name:expr) => {
        {
            let mut pln = Plane::new(
                Point::new($x, $y, $z), 
                Vector::new($xx, $xy, $xz), 
                Vector::new($yx, $yy, $yz)
            );
            pln.name = $name.to_string();
            pln
        }
    };
}

/// Macro for creating a Color with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use session_rust::primitives::Color;
/// use session_rust::color;
/// 
/// // Default color (black)
/// let c1 = color![];
/// assert_eq!(c1.r, 0);
/// assert_eq!(c1.g, 0);
/// assert_eq!(c1.b, 0);
/// assert_eq!(c1.a, 0);
/// 
/// // RGB color (with default alpha = 0)
/// let c2 = color![100, 150, 200];
/// assert_eq!(c2.r, 100);
/// assert_eq!(c2.g, 150);
/// assert_eq!(c2.b, 200);
/// assert_eq!(c2.a, 0); 
/// 
/// // RGBA color
/// let c3 = color![50, 100, 150, 255];
/// assert_eq!(c3.r, 50);
/// assert_eq!(c3.g, 100);
/// assert_eq!(c3.b, 150);
/// assert_eq!(c3.a, 255);
/// 
/// // Named colors
/// let red = color![red];
/// assert_eq!(red.r, 255);
/// assert_eq!(red.g, 0);
/// assert_eq!(red.b, 0);
/// assert_eq!(red.a, 255);
/// 
/// let green = color![green];
/// assert_eq!(green.r, 0);
/// assert_eq!(green.g, 255);
/// assert_eq!(green.b, 0);
/// assert_eq!(green.a, 255);
/// 
/// let blue = color![blue];
/// assert_eq!(blue.r, 0);
/// assert_eq!(blue.g, 0);
/// assert_eq!(blue.b, 255);
/// assert_eq!(blue.a, 255);
/// ```
#[macro_export]
macro_rules! color {
    // Empty pattern - default black color
    () => {
        Color::new(0, 0, 0, 0)
    };
    
    // RGB color with default alpha
    ($r:expr, $g:expr, $b:expr) => {
        Color::new($r, $g, $b, 0)
    };
    
    // RGBA color
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::new($r, $g, $b, $a)
    };
    
    // Named colors
    (red) => {
        Color::new(255, 0, 0, 255)
    };
    
    (green) => {
        Color::new(0, 255, 0, 255)
    };
    
    (blue) => {
        Color::new(0, 0, 255, 255)
    };
    
    (black) => {
        Color::new(0, 0, 0, 255)
    };
    
    (white) => {
        Color::new(255, 255, 255, 255)
    };
    
    (yellow) => {
        Color::new(255, 255, 0, 255)
    };
    
    (cyan) => {
        Color::new(0, 255, 255, 255)
    };
    
    (magenta) => {
        Color::new(255, 0, 255, 255)
    };
    

}

/// Macro for creating an Xform with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use session_rust::primitives::Xform;
/// use session_rust::xform;
/// 
/// // Identity matrix
/// let x1 = xform![];
/// assert_eq!(x1[(0, 0)], 1.0); // m00
/// assert_eq!(x1[(1, 1)], 1.0); // m11
/// assert_eq!(x1[(2, 2)], 1.0); // m22
/// assert_eq!(x1[(3, 3)], 1.0); // m33
/// assert_eq!(x1[(0, 1)], 0.0); // m01
/// 
/// // Translation
/// let x2 = xform![translation: 1.0, 2.0, 3.0];
/// assert_eq!(x2[(0, 3)], 1.0); // tx
/// assert_eq!(x2[(1, 3)], 2.0); // ty
/// assert_eq!(x2[(2, 3)], 3.0); // tz
/// assert_eq!(x2[(0, 0)], 1.0); // Identity part
/// 
/// // Scaling
/// let x3 = xform![scaling: 2.0, 3.0, 4.0];
/// assert_eq!(x3[(0, 0)], 2.0); // sx
/// assert_eq!(x3[(1, 1)], 3.0); // sy
/// assert_eq!(x3[(2, 2)], 4.0); // sz
/// 
/// // Rotation around X (testing approximate values due to floating point)
/// let x4 = xform![rotation_x: 90.0]; // 90 degrees
/// assert!((x4[(1, 1)] - 0.0).abs() < 1e-6); // cos(90°)
/// assert!((x4[(1, 2)] - (-1.0)).abs() < 1e-6); // -sin(90°)
/// assert!((x4[(2, 1)] - 1.0).abs() < 1e-6); // sin(90°)
/// assert!((x4[(2, 2)] - 0.0).abs() < 1e-6); // cos(90°)
/// 
/// // Full matrix specification
/// let x5 = xform![
///     1.0, 2.0, 3.0, 4.0,
///     5.0, 6.0, 7.0, 8.0,
///     9.0, 10.0, 11.0, 12.0,
///     13.0, 14.0, 15.0, 16.0
/// ];
/// assert_eq!(x5[(0, 0)], 1.0); // m00
/// assert_eq!(x5[(0, 1)], 5.0); // m01 (column-major: m at index 4)
/// assert_eq!(x5[(1, 3)], 14.0); // m13 (column-major: m at index 13)
/// assert_eq!(x5[(3, 3)], 16.0); // m33 (column-major: m at index 15)
/// ```
#[macro_export]
macro_rules! xform {
    // Pattern for identity matrix
    () => {
        Xform::identity()
    };
    
    // Pattern for translation
    (translation: $tx:expr, $ty:expr, $tz:expr) => {
        Xform::translation($tx, $ty, $tz)
    };
    
    // Pattern for scaling
    (scaling: $sx:expr, $sy:expr, $sz:expr) => {
        Xform::scaling($sx, $sy, $sz)
    };
    
    // Pattern for rotation around X axis
    (rotation_x: $angle:expr) => {
        {
            let angle_rad = $angle * std::f32::consts::PI / 180.0;
            let s = angle_rad.sin();
            let c = angle_rad.cos();
            
            let mut xf = Xform::identity();
            xf[(1, 1)] = c;
            xf[(1, 2)] = -s;
            xf[(2, 1)] = s;
            xf[(2, 2)] = c;
            xf
        }
    };
    
    // Pattern for rotation around Y axis
    (rotation_y: $angle:expr) => {
        {
            let angle_rad = $angle * std::f32::consts::PI / 180.0;
            let s = angle_rad.sin();
            let c = angle_rad.cos();
            
            let mut xf = Xform::identity();
            xf[(0, 0)] = c;
            xf[(0, 2)] = s;
            xf[(2, 0)] = -s;
            xf[(2, 2)] = c;
            xf
        }
    };
    
    // Pattern for rotation around Z axis
    (rotation_z: $angle:expr) => {
        {
            let angle_rad = $angle * std::f32::consts::PI / 180.0;
            let s = angle_rad.sin();
            let c = angle_rad.cos();
            
            let mut xf = Xform::identity();
            xf[(0, 0)] = c;
            xf[(0, 1)] = -s;
            xf[(1, 0)] = s;
            xf[(1, 1)] = c;
            xf
        }
    };
    
    // Pattern for full matrix specification
    ($m00:expr, $m01:expr, $m02:expr, $m03:expr,
     $m10:expr, $m11:expr, $m12:expr, $m13:expr,
     $m20:expr, $m21:expr, $m22:expr, $m23:expr,
     $m30:expr, $m31:expr, $m32:expr, $m33:expr) => {
        Xform { 
            m: [
                $m00, $m01, $m02, $m03,
                $m10, $m11, $m12, $m13,
                $m20, $m21, $m22, $m23,
                $m30, $m31, $m32, $m33
            ] 
        }
    };
}
