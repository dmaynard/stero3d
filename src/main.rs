use macroquad::prelude::*;

// 4D vector type for hypersolids
#[derive(Clone, Copy, Debug)]
struct Vec4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4D {
    const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlatonicSolid {
    Tetrahedron = 0,
    Cube = 1,
    Octahedron = 2,
    Dodecahedron = 3,
    Icosahedron = 4,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hypersolid {
    Tesseract = 0,
    FourSimplex = 1,
    FourOrthoplex = 2,
}

impl PlatonicSolid {
    fn next(self) -> Self {
        match self {
            PlatonicSolid::Tetrahedron => PlatonicSolid::Cube,
            PlatonicSolid::Cube => PlatonicSolid::Octahedron,
            PlatonicSolid::Octahedron => PlatonicSolid::Dodecahedron,
            PlatonicSolid::Dodecahedron => PlatonicSolid::Icosahedron,
            PlatonicSolid::Icosahedron => PlatonicSolid::Tetrahedron,
        }
    }
    
    fn name(self) -> &'static str {
        match self {
            PlatonicSolid::Tetrahedron => "Tetrahedron",
            PlatonicSolid::Cube => "Cube",
            PlatonicSolid::Octahedron => "Octahedron",
            PlatonicSolid::Dodecahedron => "Dodecahedron",
            PlatonicSolid::Icosahedron => "Icosahedron",
        }
    }
}

impl Hypersolid {
    fn next(self) -> Self {
        match self {
            Hypersolid::Tesseract => Hypersolid::FourSimplex,
            Hypersolid::FourSimplex => Hypersolid::FourOrthoplex,
            Hypersolid::FourOrthoplex => Hypersolid::Tesseract,
        }
    }
    
    fn name(self) -> &'static str {
        match self {
            Hypersolid::Tesseract => "Tesseract",
            Hypersolid::FourSimplex => "4-Simplex",
            Hypersolid::FourOrthoplex => "4-Orthoplex",
        }
    }
}

// Tetrahedron vertices (4 vertices, 6 edges)
const TETRAHEDRON_VERTICES: [Vec3; 4] = [
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(-1.0, -1.0, 1.0),
    Vec3::new(-1.0, 1.0, -1.0),
    Vec3::new(1.0, -1.0, -1.0),
];

const TETRAHEDRON_EDGES: [(usize, usize); 6] = [
    (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3),
];

// Cube vertex data (8 vertices, 12 edges)
const CUBE_VERTICES: [Vec3; 8] = [
    Vec3::new(-1.0, -1.0, -1.0), // 0: bottom-left-back
    Vec3::new(1.0, -1.0, -1.0),  // 1: bottom-right-back
    Vec3::new(1.0, 1.0, -1.0),   // 2: top-right-back
    Vec3::new(-1.0, 1.0, -1.0),  // 3: top-left-back
    Vec3::new(-1.0, -1.0, 1.0),  // 4: bottom-left-front
    Vec3::new(1.0, -1.0, 1.0),   // 5: bottom-right-front
    Vec3::new(1.0, 1.0, 1.0),    // 6: top-right-front
    Vec3::new(-1.0, 1.0, 1.0),   // 7: top-left-front
];

const CUBE_EDGES: [(usize, usize); 12] = [
    // Back face edges
    (0, 1), (1, 2), (2, 3), (3, 0),
    // Front face edges
    (4, 5), (5, 6), (6, 7), (7, 4),
    // Connecting edges
    (0, 4), (1, 5), (2, 6), (3, 7),
];

// Octahedron vertices (6 vertices, 12 edges) - scaled to match cube size
const OCTAHEDRON_VERTICES: [Vec3; 6] = [
    Vec3::new(1.4, 0.0, 0.0),   // +X (scaled by 1.4)
    Vec3::new(-1.4, 0.0, 0.0),  // -X
    Vec3::new(0.0, 1.4, 0.0),   // +Y
    Vec3::new(0.0, -1.4, 0.0),  // -Y
    Vec3::new(0.0, 0.0, 1.4),   // +Z
    Vec3::new(0.0, 0.0, -1.4),  // -Z
];

const OCTAHEDRON_EDGES: [(usize, usize); 12] = [
    (0, 2), (0, 3), (0, 4), (0, 5), // +X to all others
    (1, 2), (1, 3), (1, 4), (1, 5), // -X to all others
    (2, 4), (2, 5), (3, 4), (3, 5), // Y to Z connections
];

// Dodecahedron vertices (20 vertices, 30 edges) - using golden ratio
const PHI: f32 = 1.618034; // Golden ratio
const INV_PHI: f32 = 0.618034; // 1/phi

const DODECAHEDRON_VERTICES: [Vec3; 20] = [
    // Cube vertices
    Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, -1.0), Vec3::new(1.0, -1.0, 1.0), Vec3::new(1.0, -1.0, -1.0),
    Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-1.0, 1.0, -1.0), Vec3::new(-1.0, -1.0, 1.0), Vec3::new(-1.0, -1.0, -1.0),
    // Golden ratio rectangles in YZ plane
    Vec3::new(0.0, PHI, INV_PHI), Vec3::new(0.0, PHI, -INV_PHI), Vec3::new(0.0, -PHI, INV_PHI), Vec3::new(0.0, -PHI, -INV_PHI),
    // Golden ratio rectangles in XZ plane
    Vec3::new(INV_PHI, 0.0, PHI), Vec3::new(-INV_PHI, 0.0, PHI), Vec3::new(INV_PHI, 0.0, -PHI), Vec3::new(-INV_PHI, 0.0, -PHI),
    // Golden ratio rectangles in XY plane
    Vec3::new(PHI, INV_PHI, 0.0), Vec3::new(PHI, -INV_PHI, 0.0), Vec3::new(-PHI, INV_PHI, 0.0), Vec3::new(-PHI, -INV_PHI, 0.0),
];

const DODECAHEDRON_EDGES: [(usize, usize); 30] = [
    // This is a simplified edge set for wireframe display
    (0, 8), (0, 12), (0, 16), (1, 9), (1, 14), (1, 16), (2, 10), (2, 12), (2, 17),
    (3, 11), (3, 14), (3, 17), (4, 8), (4, 13), (4, 18), (5, 9), (5, 15), (5, 18),
    (6, 10), (6, 13), (6, 19), (7, 11), (7, 15), (7, 19), (8, 9), (10, 11), (12, 13),
    (14, 15), (16, 17), (18, 19),
];

// Icosahedron vertices (12 vertices, 30 edges) - using golden ratio
const ICOSAHEDRON_VERTICES: [Vec3; 12] = [
    // Golden ratio rectangles in different planes
    Vec3::new(0.0, INV_PHI, PHI), Vec3::new(0.0, INV_PHI, -PHI), Vec3::new(0.0, -INV_PHI, PHI), Vec3::new(0.0, -INV_PHI, -PHI),
    Vec3::new(INV_PHI, PHI, 0.0), Vec3::new(INV_PHI, -PHI, 0.0), Vec3::new(-INV_PHI, PHI, 0.0), Vec3::new(-INV_PHI, -PHI, 0.0),
    Vec3::new(PHI, 0.0, INV_PHI), Vec3::new(-PHI, 0.0, INV_PHI), Vec3::new(PHI, 0.0, -INV_PHI), Vec3::new(-PHI, 0.0, -INV_PHI),
];

const ICOSAHEDRON_EDGES: [(usize, usize); 30] = [
    (0, 2), (0, 4), (0, 6), (0, 8), (0, 9), (1, 3), (1, 4), (1, 6), (1, 10), (1, 11),
    (2, 5), (2, 7), (2, 8), (2, 9), (3, 5), (3, 7), (3, 10), (3, 11), (4, 6), (4, 8),
    (4, 10), (5, 7), (5, 8), (5, 10), (6, 9), (6, 11), (7, 9), (7, 11), (8, 10), (9, 11),
];

// Tesseract (4D hypercube) - 16 vertices, 32 edges
const TESSERACT_VERTICES: [Vec4D; 16] = [
    // Bottom cube (w = -1.0) - unit hypercube
    Vec4D::new(-1.0, -1.0, -1.0, -1.0), Vec4D::new(1.0, -1.0, -1.0, -1.0),
    Vec4D::new(-1.0, 1.0, -1.0, -1.0), Vec4D::new(1.0, 1.0, -1.0, -1.0),
    Vec4D::new(-1.0, -1.0, 1.0, -1.0), Vec4D::new(1.0, -1.0, 1.0, -1.0),
    Vec4D::new(-1.0, 1.0, 1.0, -1.0), Vec4D::new(1.0, 1.0, 1.0, -1.0),
    // Top cube (w = 1.0) - unit hypercube
    Vec4D::new(-1.0, -1.0, -1.0, 1.0), Vec4D::new(1.0, -1.0, -1.0, 1.0),
    Vec4D::new(-1.0, 1.0, -1.0, 1.0), Vec4D::new(1.0, 1.0, -1.0, 1.0),
    Vec4D::new(-1.0, -1.0, 1.0, 1.0), Vec4D::new(1.0, -1.0, 1.0, 1.0),
    Vec4D::new(-1.0, 1.0, 1.0, 1.0), Vec4D::new(1.0, 1.0, 1.0, 1.0),
];

const TESSERACT_EDGES: [(usize, usize); 32] = [
    // Bottom cube edges
    (0, 1), (0, 2), (0, 4), (1, 3), (1, 5), (2, 3), (2, 6), (3, 7),
    (4, 5), (4, 6), (5, 7), (6, 7),
    // Top cube edges  
    (8, 9), (8, 10), (8, 12), (9, 11), (9, 13), (10, 11), (10, 14), (11, 15),
    (12, 13), (12, 14), (13, 15), (14, 15),
    // Connecting edges (between bottom and top cubes)
    (0, 8), (1, 9), (2, 10), (3, 11), (4, 12), (5, 13), (6, 14), (7, 15),
];

// 4-Simplex (4D tetrahedron) - 5 vertices, 10 edges
const FOUR_SIMPLEX_VERTICES: [Vec4D; 5] = [
    Vec4D::new(1.0, 1.0, 1.0, 1.0),
    Vec4D::new(-1.0, -1.0, 1.0, 1.0),
    Vec4D::new(-1.0, 1.0, -1.0, 1.0),
    Vec4D::new(1.0, -1.0, -1.0, 1.0),
    Vec4D::new(0.0, 0.0, 0.0, -1.0),
];

const FOUR_SIMPLEX_EDGES: [(usize, usize); 10] = [
    (0, 1), (0, 2), (0, 3), (0, 4),
    (1, 2), (1, 3), (1, 4),
    (2, 3), (2, 4),
    (3, 4),
];

// 4-Orthoplex (4D octahedron) - 8 vertices, 24 edges
const FOUR_ORTHOPLEX_VERTICES: [Vec4D; 8] = [
    Vec4D::new(1.0, 0.0, 0.0, 0.0), Vec4D::new(-1.0, 0.0, 0.0, 0.0),
    Vec4D::new(0.0, 1.0, 0.0, 0.0), Vec4D::new(0.0, -1.0, 0.0, 0.0),
    Vec4D::new(0.0, 0.0, 1.0, 0.0), Vec4D::new(0.0, 0.0, -1.0, 0.0),
    Vec4D::new(0.0, 0.0, 0.0, 1.0), Vec4D::new(0.0, 0.0, 0.0, -1.0),
];

const FOUR_ORTHOPLEX_EDGES: [(usize, usize); 24] = [
    (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
    (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
    (2, 4), (2, 5), (2, 6), (2, 7),
    (3, 4), (3, 5), (3, 6), (3, 7),
    (4, 6), (4, 7),
    (5, 6), (5, 7),
];

struct StereogramViewer {
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    rotation_velocity_x: f32,
    rotation_velocity_y: f32,
    rotation_velocity_z: f32,
    // 4D rotations (6 rotation planes: XY, XZ, YZ, XW, YW, ZW)
    rotation_xy: f32,
    rotation_xz: f32,
    rotation_yz: f32,
    rotation_xw: f32,
    rotation_yw: f32,
    rotation_zw: f32,
    rotation_velocity_xy: f32,
    rotation_velocity_xz: f32,
    rotation_velocity_yz: f32,
    rotation_velocity_xw: f32,
    rotation_velocity_yw: f32,
    rotation_velocity_zw: f32,
    eye_separation: f32,
    perspective_distance: f32,
    is_paused: bool,
    show_guides: bool,
    depth_coloring: bool,
    w_depth_coloring: bool,
    show_ui: bool,
    dark_background: bool,
    orthographic: bool,
    current_solid: PlatonicSolid,
    current_hypersolid: Hypersolid,
    is_4d_mode: bool, // Toggle between 3D and 4D modes
    show_3d_controls: bool,
    show_4d_controls: bool,
    dragging_slider: Option<usize>, // None, Some(0) for X, Some(1) for Y, Some(2) for Z
    dragging_angle_slider: Option<usize>, // None, Some(0) for X angle, Some(1) for Y angle, Some(2) for Z angle
    dragging_4d_slider: Option<usize>, // None, Some(0-5) for 4D rotation planes
    debug_printed: bool, // Flag to only print debug info once per pause
}

impl StereogramViewer {
    fn new() -> Self {
        Self {
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            rotation_velocity_x: 0.0, // Turn off automatic rotations for debugging
            rotation_velocity_y: 0.0,  // Turn off automatic rotations for debugging
            rotation_velocity_z: 0.0, // Turn off automatic rotations for debugging
            // 4D rotations initialized to 0
            rotation_xy: 0.0,
            rotation_xz: 0.0,
            rotation_yz: 0.0,
            rotation_xw: 0.0,
            rotation_yw: 0.0,
            rotation_zw: 0.0,
            rotation_velocity_xy: 0.0, // Turn off automatic 4D rotations for debugging
            rotation_velocity_xz: 0.0,
            rotation_velocity_yz: 0.0,
            rotation_velocity_xw: 0.0, // Turn off automatic 4D rotations for debugging
            rotation_velocity_yw: 0.0,
            rotation_velocity_zw: 0.0,
            eye_separation: 0.06, // Reduced for iPhone dimensions
            perspective_distance: 5.0, // Much smaller perspective distance for unit objects
            is_paused: false,
            show_guides: true,
            depth_coloring: true,
            w_depth_coloring: false,
            #[cfg(target_arch = "wasm32")]
            show_ui: false, // Web users see HTML instructions, so hide UI by default
            #[cfg(not(target_arch = "wasm32"))]
            show_ui: true, // Native app users need UI visible by default
            dark_background: false, // Default to white background
            orthographic: false, // Perspective projection is default
            current_solid: PlatonicSolid::Cube, // Default to cube
            current_hypersolid: Hypersolid::Tesseract, // Default to tesseract
            is_4d_mode: false, // Start in 3D mode
            show_3d_controls: false, // Default to off
            show_4d_controls: false, // Default to off
            dragging_slider: None, // No slider being dragged initially
            dragging_angle_slider: None, // No angle slider being dragged initially
            dragging_4d_slider: None, // No 4D slider being dragged initially
            debug_printed: false, // No debug output printed yet
        }
    }

    fn update(&mut self) {
        if !self.is_paused {
            if self.is_4d_mode {
                // Update 4D rotations
                self.rotation_xy += self.rotation_velocity_xy;
                self.rotation_xz += self.rotation_velocity_xz;
                self.rotation_yz += self.rotation_velocity_yz;
                self.rotation_xw += self.rotation_velocity_xw;
                self.rotation_yw += self.rotation_velocity_yw;
                self.rotation_zw += self.rotation_velocity_zw;
            } else {
                // Update 3D rotations
                self.rotation_x += self.rotation_velocity_x;
                self.rotation_y += self.rotation_velocity_y;
                self.rotation_z += self.rotation_velocity_z;
            }
        }
    }
    
    fn get_vertices(&self) -> &[Vec3] {
        match self.current_solid {
            PlatonicSolid::Tetrahedron => &TETRAHEDRON_VERTICES,
            PlatonicSolid::Cube => &CUBE_VERTICES,
            PlatonicSolid::Octahedron => &OCTAHEDRON_VERTICES,
            PlatonicSolid::Dodecahedron => &DODECAHEDRON_VERTICES,
            PlatonicSolid::Icosahedron => &ICOSAHEDRON_VERTICES,
        }
    }
    
    fn get_edges(&self) -> &[(usize, usize)] {
        match self.current_solid {
            PlatonicSolid::Tetrahedron => &TETRAHEDRON_EDGES,
            PlatonicSolid::Cube => &CUBE_EDGES,
            PlatonicSolid::Octahedron => &OCTAHEDRON_EDGES,
            PlatonicSolid::Dodecahedron => &DODECAHEDRON_EDGES,
            PlatonicSolid::Icosahedron => &ICOSAHEDRON_EDGES,
        }
    }
    
    fn get_4d_vertices(&self) -> &[Vec4D] {
        match self.current_hypersolid {
            Hypersolid::Tesseract => &TESSERACT_VERTICES,
            Hypersolid::FourSimplex => &FOUR_SIMPLEX_VERTICES,
            Hypersolid::FourOrthoplex => &FOUR_ORTHOPLEX_VERTICES,
        }
    }
    
    fn get_4d_edges(&self) -> &[(usize, usize)] {
        match self.current_hypersolid {
            Hypersolid::Tesseract => &TESSERACT_EDGES,
            Hypersolid::FourSimplex => &FOUR_SIMPLEX_EDGES,
            Hypersolid::FourOrthoplex => &FOUR_ORTHOPLEX_EDGES,
        }
    }
    
    // Apply 4D rotations in all 6 rotation planes
    fn apply_4d_rotations(&self, vertex: Vec4D) -> Vec4D {
        let mut result = vertex;
        
        // XY plane rotation
        let xy_cos = self.rotation_xy.cos();
        let xy_sin = self.rotation_xy.sin();
        let x = result.x;
        let y = result.y;
        result.x = x * xy_cos - y * xy_sin;
        result.y = x * xy_sin + y * xy_cos;
        
        // XZ plane rotation
        let xz_cos = self.rotation_xz.cos();
        let xz_sin = self.rotation_xz.sin();
        let x = result.x;
        let z = result.z;
        result.x = x * xz_cos - z * xz_sin;
        result.z = x * xz_sin + z * xz_cos;
        
        // YZ plane rotation
        let yz_cos = self.rotation_yz.cos();
        let yz_sin = self.rotation_yz.sin();
        let y = result.y;
        let z = result.z;
        result.y = y * yz_cos - z * yz_sin;
        result.z = y * yz_sin + z * yz_cos;
        
        // XW plane rotation
        let xw_cos = self.rotation_xw.cos();
        let xw_sin = self.rotation_xw.sin();
        let x = result.x;
        let w = result.w;
        result.x = x * xw_cos - w * xw_sin;
        result.w = x * xw_sin + w * xw_cos;
        
        // YW plane rotation
        let yw_cos = self.rotation_yw.cos();
        let yw_sin = self.rotation_yw.sin();
        let y = result.y;
        let w = result.w;
        result.y = y * yw_cos - w * yw_sin;
        result.w = y * yw_sin + w * yw_cos;
        
        // ZW plane rotation
        let zw_cos = self.rotation_zw.cos();
        let zw_sin = self.rotation_zw.sin();
        let z = result.z;
        let w = result.w;
        result.z = z * zw_cos - w * zw_sin;
        result.w = z * zw_sin + w * zw_cos;
        
        result
    }

    // Project 4D vertex to 3D space
    fn project_4d_to_3d(&self, vertex_4d: Vec4D, _w_distance: f32) -> Vec3 {
        // Use W coordinate to create proper 3D separation
        // Spread vertices in 3D space based on their W coordinate
        Vec3::new(
            vertex_4d.x + vertex_4d.w * 0.2, // W affects X position
            vertex_4d.y + vertex_4d.w * 0.2, // W affects Y position
            vertex_4d.z + vertex_4d.w * 0.5  // W affects Z position
        )
    }

    fn draw_solid_wireframe(&self, camera_offset: f32, screen_offset_x: f32) {
        // Apply rotation to vertices first
        // Create rotation matrices for each axis
        let rot_x_matrix = Mat4::from_rotation_x(self.rotation_x);
        let rot_y_matrix = Mat4::from_rotation_y(self.rotation_y);
        let rot_z_matrix = Mat4::from_rotation_z(self.rotation_z);
        
        // Combine rotations: Z * Y * X (order matters for 3D rotation)
        let combined_rotation = rot_z_matrix * rot_y_matrix * rot_x_matrix;

        // Transform and project vertices to 2D screen space manually
        let screen_center_x = screen_width() / 4.0 + screen_offset_x; // Quarter width + offset for each view
        let screen_center_y = screen_height() / 2.0;
        // Adjust scale to maintain cube size as perspective changes
        let base_scale = 180.0;
        let scale = if self.orthographic {
            // For orthographic projection, match the effective scale of the front face in perspective mode
            // In perspective, front face is at z = perspective_distance + 1.0, so effective scale is base_scale / z
            let perspective_scale = base_scale * (self.perspective_distance / 4.0);
            (perspective_scale / (self.perspective_distance + 1.0)) * 1.2 // Match front face size and increase by 20%
        } else {
            base_scale * (self.perspective_distance / 4.0) // Scale proportionally with distance for perspective
        };
        let perspective_distance = self.perspective_distance; // Use adjustable perspective distance

        // Project each vertex to 2D screen coordinates
        let mut projected_vertices = Vec::new();
        let mut transformed_vertices = Vec::new();
        
        let vertices = self.get_vertices();
        for &vertex in vertices {
            // Apply rotation
            let vec4 = combined_rotation * Vec4::new(vertex.x, vertex.y, vertex.z, 1.0);
            let rotated = Vec3::new(vec4.x, vec4.y, vec4.z);
            transformed_vertices.push(rotated);
            
            // Apply camera offset (for stereogram effect)
            let camera_adjusted = Vec3::new(
                rotated.x - camera_offset,
                rotated.y,
                rotated.z + perspective_distance
            );
            
            // Choose projection type: perspective or orthographic
            let (projected_x, projected_y) = if self.orthographic {
                // Orthographic projection - no perspective distortion
                let projected_x = screen_center_x + camera_adjusted.x * scale;
                let projected_y = screen_center_y - camera_adjusted.y * scale;
                (projected_x, projected_y)
            } else {
                // Perspective projection - traditional 3D perspective
                let projected_x = screen_center_x + (camera_adjusted.x * scale) / camera_adjusted.z;
                let projected_y = screen_center_y - (camera_adjusted.y * scale) / camera_adjusted.z;
                (projected_x, projected_y)
            };
            
            projected_vertices.push(Vec2::new(projected_x, projected_y));
        }

        // Draw wireframe edges using 2D lines with depth sorting
        let edges = self.get_edges();
        
        // Collect all edges with their depth information for sorting
        let mut edge_data: Vec<(f32, Vec2, Vec2, Color)> = Vec::new();
        
        for &(start_idx, end_idx) in edges {
            let start_2d = projected_vertices[start_idx];
            let end_2d = projected_vertices[end_idx];
            
            // Calculate depth for sorting (use raw Z values after transformation)
            let start_3d = transformed_vertices[start_idx];
            let end_3d = transformed_vertices[end_idx];
            let avg_z = (start_3d.z + end_3d.z) / 2.0;
            
            let wire_color = if self.depth_coloring {
                // Map Z distance to color intensity (closer = darker, farther = lighter)
                // Use raw Z values for consistent depth calculation
                let min_z = -2.0; // Approximate range for transformed vertices
                let max_z = 2.0;
                let intensity = ((max_z - avg_z) / (max_z - min_z)).clamp(0.2, 1.0);
                
                // Create color based on depth and background
                if self.dark_background {
                    // Dark background: use white to light gray
                    Color::new(intensity, intensity, intensity, 1.0)
                } else {
                    // Light background: use black to dark gray
                    Color::new(1.0 - intensity, 1.0 - intensity, 1.0 - intensity, 1.0)
                }
            } else {
                // Use plain color based on background
                if self.dark_background {
                    WHITE
                } else {
                    BLACK
                }
            };
            
            // Store edge data with depth for sorting
            edge_data.push((avg_z, start_2d, end_2d, wire_color));
        }
        
        // Sort edges by depth (nearest first, farthest last) for proper depth sorting
        edge_data.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        // Draw edges from front to back (nearest first)
        for (_, start_2d, end_2d, wire_color) in edge_data {
            draw_line(start_2d.x, start_2d.y, end_2d.x, end_2d.y, 2.0, wire_color);
        }
    }

    fn render_stereogram(&mut self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let half_width = screen_width / 2.0;
        
        // Clear the screen with current background setting
        if self.dark_background {
            clear_background(BLACK);
        } else {
            clear_background(WHITE);
        }
        
        // Use default 2D camera for all rendering
        set_default_camera();
        
        // Render left eye view (left half of screen)
        if self.is_4d_mode {
            self.draw_4d_hypersolid_wireframe(-self.eye_separation, 0.0);
        } else {
            self.draw_solid_wireframe(-self.eye_separation, 0.0);
        }
        
        // Render right eye view (right half of screen)  
        if self.is_4d_mode {
            self.draw_4d_hypersolid_wireframe(self.eye_separation, half_width);
        } else {
            self.draw_solid_wireframe(self.eye_separation, half_width);
        }
        
        // Draw guides only if show_guides is true
        if self.show_guides {
            // Add fusion aids - outline circles just above the cubes
            let fusion_aid_y = screen_height / 2.0 - 150.0; // Higher above cube center
            let fusion_aid_size = 6.0; // Smaller for less distraction
            let outline_thickness = 2.0; // Thin outline
            
            // Left fusion aid (center of left half)
            let left_aid_x = half_width / 2.0;
            draw_circle_lines(left_aid_x, fusion_aid_y, fusion_aid_size, outline_thickness, RED);
            
            // Right fusion aid (center of right half)
            let right_aid_x = half_width + half_width / 2.0;
            draw_circle_lines(right_aid_x, fusion_aid_y, fusion_aid_size, outline_thickness, RED);
        }
    }
    
    fn draw_4d_hypersolid_wireframe(&mut self, camera_offset: f32, screen_offset_x: f32) {
        // Debug: Print vertex coordinates when paused (only once per pause)
        let should_print_debug = self.is_paused && !self.debug_printed;
        if should_print_debug {
            println!("\n=== 4D VERTEX DEBUG (PAUSED) ===");
            println!("Hypersolid: {:?}", self.current_hypersolid);
            println!("Orthographic: {}", self.orthographic);
            println!("Perspective Distance: {:.1}", self.perspective_distance);
            println!("Camera Offset: {:.1}", camera_offset);
            println!("Screen Offset X: {:.1}", screen_offset_x);
            println!("Screen Size: {}x{}", screen_width(), screen_height());
            self.debug_printed = true; // Mark as printed
        }
        
        let vertices_4d = self.get_4d_vertices();
        let edges = self.get_4d_edges();
        
        // Apply 4D rotations to vertices
        let mut transformed_vertices_4d = Vec::new();
        for (i, &vertex_4d) in vertices_4d.iter().enumerate() {
            // Apply 4D rotations in all 6 planes
            let transformed = self.apply_4d_rotations(vertex_4d);
            
            if should_print_debug { // Print all vertices to see what's happening
                println!("Vertex {}: 4D original ({:.1}, {:.1}, {:.1}, {:.1}) -> 4D transformed ({:.1}, {:.1}, {:.1}, {:.1})", 
                    i, vertex_4d.x, vertex_4d.y, vertex_4d.z, vertex_4d.w,
                    transformed.x, transformed.y, transformed.z, transformed.w);
            }
            
            transformed_vertices_4d.push(transformed);
        }
        
        // Project 4D vertices to 3D space
        // Use a smaller distance for 4D objects to prevent clustering
        let w_distance = if self.orthographic { 1.0 } else { 1.5 }; // Even smaller for perspective mode
        let mut vertices_3d = Vec::new();
        for (i, vertex_4d) in transformed_vertices_4d.iter().enumerate() {
            let vertex_3d = self.project_4d_to_3d(*vertex_4d, w_distance);
            if should_print_debug { // Print all vertices to see what's happening
                println!("Vertex {}: 4D->3D projected ({:.1}, {:.1}, {:.1})", 
                    i, vertex_3d.x, vertex_3d.y, vertex_3d.z);
            }
            vertices_3d.push(vertex_3d);
            
        }
        
        // Apply 3D rotation to the projected vertices (no scaling yet)
        let rot_x_matrix = Mat4::from_rotation_x(self.rotation_x);
        let rot_y_matrix = Mat4::from_rotation_y(self.rotation_y);
        let rot_z_matrix = Mat4::from_rotation_z(self.rotation_z);
        let combined_rotation = rot_z_matrix * rot_y_matrix * rot_x_matrix;
        
        let mut transformed_vertices = Vec::new();
        for (i, vertex_3d) in vertices_3d.iter().enumerate() {
            let rotated_vertex = combined_rotation.transform_point3(*vertex_3d);
            if should_print_debug { // Print all vertices to see what's happening
                println!("Vertex {}: 3D rotated ({:.1}, {:.1}, {:.1})", 
                    i, rotated_vertex.x, rotated_vertex.y, rotated_vertex.z);
            }
            transformed_vertices.push(rotated_vertex);
            
        }
        
        // Calculate 2D screen positions with proper scaling
        // Scale to fit 25% of screen width and center the object
        let screen_width = screen_width();
        let screen_height = screen_height();
        let target_width = screen_width * 0.25; // 25% of screen width for proper sizing
        let half_screen_width = screen_width * 0.5;
        let viewport_center_x = screen_offset_x + half_screen_width * 0.5; // Center in the quarter-screen viewport
        let viewport_center_y = screen_height * 0.5;
        
        // Find the bounding box of the rotated vertices to determine scale
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        
        if self.orthographic {
            // For orthographic, use direct 3D coordinates
            for vertex in &transformed_vertices {
                min_x = min_x.min(vertex.x);
                max_x = max_x.max(vertex.x);
                min_y = min_y.min(vertex.y);
                max_y = max_y.max(vertex.y);
            }
        } else {
            // For perspective, calculate projected coordinates to get accurate size
            for vertex in &transformed_vertices {
                let perspective_factor = 1.0 / (self.perspective_distance + vertex.z);
                let proj_x = vertex.x * perspective_factor;
                let proj_y = vertex.y * perspective_factor;
                min_x = min_x.min(proj_x);
                max_x = max_x.max(proj_x);
                min_y = min_y.min(proj_y);
                max_y = max_y.max(proj_y);
            }
        }
        
        let object_width = max_x - min_x;
        let object_height = max_y - min_y;
        let object_size = object_width.max(object_height);
        let scale = if object_size > 0.0 { target_width / object_size } else { 1.0 };
        
        if should_print_debug {
            println!("Scale: {:.3}, Object size: {:.3}, Target width: {:.1}", scale, object_size, target_width);
            println!("Viewport center: ({:.1}, {:.1})", viewport_center_x, viewport_center_y);
        }
        
        let mut edge_data: Vec<(f32, f32, Vec2, Vec2, Color)> = Vec::new(); // (avg_z, avg_w, start_2d, end_2d, color)
        for &(start_idx, end_idx) in edges {
            let start_3d = transformed_vertices[start_idx];
            let end_3d = transformed_vertices[end_idx];
            
            
            let start_2d = if self.orthographic {
                Vec2::new(
                    viewport_center_x + start_3d.x * scale + camera_offset,
                    viewport_center_y - start_3d.y * scale - start_3d.z * scale * 0.5, // Use Z for depth separation
                )
            } else {
                // Use perspective distance for 4D objects
                let perspective_distance = self.perspective_distance;
                let perspective_factor = 1.0 / (perspective_distance + start_3d.z);
                Vec2::new(
                    viewport_center_x + start_3d.x * scale * perspective_factor + camera_offset,
                    viewport_center_y - start_3d.y * scale * perspective_factor, // Invert Y to center properly
                )
            };
            
            let end_2d = if self.orthographic {
                Vec2::new(
                    viewport_center_x + end_3d.x * scale + camera_offset,
                    viewport_center_y - end_3d.y * scale - end_3d.z * scale * 0.5, // Use Z for depth separation
                )
            } else {
                // Use perspective distance for 4D objects
                let perspective_distance = self.perspective_distance;
                let perspective_factor = 1.0 / (perspective_distance + end_3d.z);
                Vec2::new(
                    viewport_center_x + end_3d.x * scale * perspective_factor + camera_offset,
                    viewport_center_y - end_3d.y * scale * perspective_factor, // Invert Y to center properly
                )
            };
            
            // Debug: Print 2D screen coordinates for first few edges when paused
            if should_print_debug && edge_data.len() < 3 {
                println!("Edge {}: 3D start ({:.1}, {:.1}, {:.1}) -> 2D start ({:.1}, {:.1})", 
                    edge_data.len(), start_3d.x, start_3d.y, start_3d.z, start_2d.x, start_2d.y);
                println!("Edge {}: 3D end ({:.1}, {:.1}, {:.1}) -> 2D end ({:.1}, {:.1})", 
                    edge_data.len(), end_3d.x, end_3d.y, end_3d.z, end_2d.x, end_2d.y);
                if !self.orthographic {
                    let perspective_factor_start = 1.0 / (self.perspective_distance + start_3d.z);
                    let perspective_factor_end = 1.0 / (self.perspective_distance + end_3d.z);
                    println!("  Perspective factors: start={:.3}, end={:.3}", perspective_factor_start, perspective_factor_end);
                }
            }
            
            
            let avg_z = (start_3d.z + end_3d.z) / 2.0;
            
            // Calculate average W coordinate for W-depth coloring
            let start_w = transformed_vertices_4d[start_idx].w;
            let end_w = transformed_vertices_4d[end_idx].w;
            let avg_w = (start_w + end_w) / 2.0;
            
            // Store edge data without color - color will be calculated later
            edge_data.push((avg_z, avg_w, start_2d, end_2d, WHITE)); // Placeholder color
        }
        
        // Calculate Z range once per frame for consistent depth coloring
        let mut frame_min_z = f32::MAX;
        let mut frame_max_z = f32::MIN;
        for (avg_z, _, _, _, _) in &edge_data {
            frame_min_z = frame_min_z.min(*avg_z);
            frame_max_z = frame_max_z.max(*avg_z);
        }
        
        // Debug: Print frame Z range
        if should_print_debug {
            println!("Frame Z range: {:.3} to {:.3}", frame_min_z, frame_max_z);
        }
        
        // Sort edges by depth (front to back)
        edge_data.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        // Calculate W range for W-depth coloring
        let mut frame_min_w = f32::MAX;
        let mut frame_max_w = f32::MIN;
        for (_, avg_w, _, _, _) in &edge_data {
            frame_min_w = frame_min_w.min(*avg_w);
            frame_max_w = frame_max_w.max(*avg_w);
        }
        
        // Debug: Print frame W range
        if should_print_debug {
            println!("Frame W range: {:.3} to {:.3}", frame_min_w, frame_max_w);
        }
        
        // Draw edges with improved depth coloring
        for (avg_z, avg_w, start_2d, end_2d, _) in &edge_data {
            let wire_color = if self.w_depth_coloring {
                // Use W coordinate for 4th dimension coloring
                let w_range = frame_max_w - frame_min_w;
                let normalized_w = if w_range > 0.001 {
                    (avg_w - frame_min_w) / w_range  // Higher W = higher normalized_w
                } else {
                    0.5 // Default to middle if no range
                };
                
                // Debug: Print normalized W values for first few edges
                if should_print_debug && edge_data.len() < 5 {
                    println!("Edge {}: avg_w={:.3}, normalized_w={:.3}", edge_data.len(), avg_w, normalized_w);
                }
                
                // Create W-based color bands (different from Z coloring)
                if self.dark_background {
                    // Dark background: shades of white/gray based on W (6 bands) - highest W = brightest
                    if normalized_w > 0.83 {
                        Color::new(1.0, 1.0, 1.0, 1.0) // White (highest W)
                    } else if normalized_w > 0.67 {
                        Color::new(0.9, 0.9, 0.9, 1.0) // Very light gray
                    } else if normalized_w > 0.50 {
                        Color::new(0.8, 0.8, 0.8, 1.0) // Light gray
                    } else if normalized_w > 0.33 {
                        Color::new(0.7, 0.7, 0.7, 1.0) // Medium gray
                    } else if normalized_w > 0.17 {
                        Color::new(0.6, 0.6, 0.6, 1.0) // Dark gray
                    } else {
                        Color::new(0.5, 0.5, 0.5, 1.0) // Very dark gray (lowest W)
                    }
                } else {
                    // Light background: shades of black/gray based on W (6 bands) - highest W = darkest
                    if normalized_w > 0.83 {
                        Color::new(0.0, 0.0, 0.0, 1.0) // Black (highest W)
                    } else if normalized_w > 0.67 {
                        Color::new(0.2, 0.2, 0.2, 1.0) // Very dark gray
                    } else if normalized_w > 0.50 {
                        Color::new(0.4, 0.4, 0.4, 1.0) // Dark gray
                    } else if normalized_w > 0.33 {
                        Color::new(0.6, 0.6, 0.6, 1.0) // Medium gray
                    } else if normalized_w > 0.17 {
                        Color::new(0.8, 0.8, 0.8, 1.0) // Light gray
                    } else {
                        Color::new(0.9, 0.9, 0.9, 1.0) // Very light gray (lowest W)
                    }
                }
            } else if self.depth_coloring {
                // Use frame-wide Z range for consistent coloring
                // Invert the calculation so closer edges (lower Z) get higher normalized values
                let z_range = frame_max_z - frame_min_z;
                let normalized_z = if z_range > 0.001 {
                    1.0 - (avg_z - frame_min_z) / z_range  // Invert: closer = higher normalized_z
                } else {
                    0.5 // Default to middle if no range
                };
                
                // Debug: Print normalized Z values for first few edges
                if should_print_debug && edge_data.len() < 5 {
                    println!("Edge {}: avg_z={:.3}, normalized_z={:.3}", edge_data.len(), avg_z, normalized_z);
                }
                
                // Create more color bands for better distribution
                if self.dark_background {
                    // Dark background: shades of white/gray (6 bands) - closest = brightest
                    if normalized_z > 0.83 {
                        Color::new(1.0, 1.0, 1.0, 1.0) // White (closest)
                    } else if normalized_z > 0.67 {
                        Color::new(0.95, 0.95, 0.95, 1.0) // Very light gray
                    } else if normalized_z > 0.50 {
                        Color::new(0.9, 0.9, 0.9, 1.0) // Light gray
                    } else if normalized_z > 0.33 {
                        Color::new(0.85, 0.85, 0.85, 1.0) // Medium-light gray
                    } else if normalized_z > 0.17 {
                        Color::new(0.8, 0.8, 0.8, 1.0) // Medium gray
                    } else {
                        Color::new(0.75, 0.75, 0.75, 1.0) // Light gray (farthest)
                    }
                } else {
                    // Light background: dark colors (6 bands) - closest = darkest
                    if normalized_z > 0.83 {
                        Color::new(0.1, 0.1, 0.1, 1.0) // Very dark gray (closest)
                    } else if normalized_z > 0.67 {
                        Color::new(0.25, 0.25, 0.25, 1.0) // Dark gray
                    } else if normalized_z > 0.50 {
                        Color::new(0.4, 0.4, 0.4, 1.0) // Medium-dark gray
                    } else if normalized_z > 0.33 {
                        Color::new(0.55, 0.55, 0.55, 1.0) // Medium gray
                    } else if normalized_z > 0.17 {
                        Color::new(0.7, 0.7, 0.7, 1.0) // Light gray
                    } else {
                        Color::new(0.85, 0.85, 0.85, 1.0) // Very light gray (farthest)
                    }
                }
            } else {
                // Uniform color when depth coloring is off
                if self.dark_background {
                    Color::new(0.8, 0.8, 0.8, 1.0)
                } else {
                    Color::new(0.2, 0.2, 0.2, 1.0)
                }
            };
            
            draw_line(start_2d.x, start_2d.y, end_2d.x, end_2d.y, 2.0, wire_color);
        }
        
        
        // Debug info moved to main UI section to avoid duplicate drawing
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "iPhone Stereogram Viewer".to_owned(),
        window_width: 393,
        window_height: 852,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut viewer = StereogramViewer::new();
    
    loop {
        viewer.update();
        
        // Render stereogram
        viewer.render_stereogram();
        
        // Draw UI overlay
        set_default_camera();
        
        // Draw help button (always visible)
        let help_button_x = 10.0;
        let help_button_y = 10.0;
        let button_size = 30.0;
        
        // Help button background
        draw_rectangle(
            help_button_x,
            help_button_y,
            button_size,
            button_size,
            if viewer.dark_background { Color::new(0.2, 0.2, 0.2, 0.8) } else { Color::new(0.9, 0.9, 0.9, 0.8) }
        );
        
        // Help button border
        draw_rectangle_lines(
            help_button_x,
            help_button_y,
            button_size,
            button_size,
            2.0,
            if viewer.dark_background { WHITE } else { BLACK }
        );
        
        // Help button text "?"
        draw_text(
            "?",
            help_button_x + 8.0,
            help_button_y + 22.0,
            20.0,
            if viewer.dark_background { WHITE } else { BLACK }
        );
        
        // Draw 3D controls button (always visible)
        let controls_button_x = 50.0;
        let controls_button_y = 10.0;
        
        // 3D controls button background
        draw_rectangle(
            controls_button_x,
            controls_button_y,
            button_size,
            button_size,
            if viewer.show_3d_controls {
                if viewer.dark_background { Color::new(0.3, 0.5, 0.3, 0.8) } else { Color::new(0.7, 0.9, 0.7, 0.8) }
            } else {
                if viewer.dark_background { Color::new(0.2, 0.2, 0.2, 0.8) } else { Color::new(0.9, 0.9, 0.9, 0.8) }
            }
        );
        
        // 3D controls button border
        draw_rectangle_lines(
            controls_button_x,
            controls_button_y,
            button_size,
            button_size,
            2.0,
            if viewer.dark_background { WHITE } else { BLACK }
        );
        
        // 3D controls button text "3D"
        // Draw "3D"/"4D" button with dynamic label
        let button_label = if viewer.is_4d_mode { "4D" } else { "3D" };
        draw_text(
            button_label,
            controls_button_x + 4.0,
            controls_button_y + 22.0,
            16.0,
            if viewer.dark_background { WHITE } else { BLACK }
        );
        
        // Draw 3D controls panel
        if viewer.show_3d_controls {
            let panel_x = 10.0;
            let panel_y = 50.0;
            let panel_width = 300.0;
            let panel_height = 340.0;
            
            // Panel background
            draw_rectangle(
                panel_x,
                panel_y,
                panel_width,
                panel_height,
                if viewer.dark_background { Color::new(0.1, 0.1, 0.1, 0.9) } else { Color::new(0.95, 0.95, 0.95, 0.9) }
            );
            
            // Panel border
            draw_rectangle_lines(
                panel_x,
                panel_y,
                panel_width,
                panel_height,
                2.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Panel title
            let panel_title = if viewer.is_4d_mode { "4D Rotation Controls" } else { "3D Rotation Controls" };
            draw_text(
                panel_title,
                panel_x + 10.0,
                panel_y + 25.0,
                20.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Rotation angle sliders (0-360 degrees)
            let angle_slider_x = panel_x + 20.0;
            let angle_slider_y = panel_y + 50.0;
            let angle_slider_width = 120.0; // Reduced width to fit in left column
            let angle_slider_height = 20.0;
            let angle_slider_spacing = 35.0;
            
            if viewer.is_4d_mode {
                // Draw 4D rotation sliders in two columns
                let rotation_angles = [
                    (viewer.rotation_xy, "XY"),
                    (viewer.rotation_xz, "XZ"),
                    (viewer.rotation_yz, "YZ"),
                    (viewer.rotation_xw, "XW"),
                    (viewer.rotation_yw, "YW"),
                    (viewer.rotation_zw, "ZW"),
                ];
                
                // Left column: Angle sliders
                for (i, (rotation_angle, label)) in rotation_angles.iter().enumerate() {
                    let y_pos = angle_slider_y + (i as f32) * angle_slider_spacing;
                    
                    // Label
                    draw_text(
                        label,
                        angle_slider_x,
                        y_pos - 5.0,
                        14.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    
                    // Degree markers
                    draw_text(
                        "0°",
                        angle_slider_x - 15.0,
                        y_pos + 15.0,
                        12.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    draw_text(
                        "180°",
                        angle_slider_x + angle_slider_width / 2.0 - 15.0,
                        y_pos + 15.0,
                        12.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    draw_text(
                        "360°",
                        angle_slider_x + angle_slider_width + 5.0,
                        y_pos + 15.0,
                        12.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    
                    // Slider track
                    draw_rectangle(
                        angle_slider_x,
                        y_pos,
                        angle_slider_width,
                        angle_slider_height,
                        if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
                    );
                    
                    // Slider handle (normalize to 0-360 degrees)
                    let degrees = rotation_angle.to_degrees() % 360.0;
                    let normalized = if degrees < 0.0 { degrees + 360.0 } else { degrees };
                    let handle_x = angle_slider_x + (normalized / 360.0) * angle_slider_width;
                    draw_rectangle(
                        handle_x - 5.0,
                        y_pos + 2.0,
                        10.0,
                        angle_slider_height - 4.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                }
            } else {
                // Draw 3D rotation sliders (X, Y, Z)
            
            // X angle slider
            draw_text(
                "X:",
                angle_slider_x,
                angle_slider_y - 5.0,
                14.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Slider labels (0, 180, 360)
            draw_text(
                "0°",
                angle_slider_x - 15.0,
                angle_slider_y + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "180°",
                angle_slider_x + angle_slider_width / 2.0 - 15.0,
                angle_slider_y + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "360°",
                angle_slider_x + angle_slider_width + 5.0,
                angle_slider_y + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // X angle slider track
            draw_rectangle(
                angle_slider_x,
                angle_slider_y,
                angle_slider_width,
                angle_slider_height,
                if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
            );
            
            // X angle slider handle (normalize to 0-360 degrees)
            let x_degrees = viewer.rotation_x.to_degrees() % 360.0;
            let x_normalized = if x_degrees < 0.0 { x_degrees + 360.0 } else { x_degrees };
            let x_angle_handle_x = angle_slider_x + (x_normalized / 360.0) * angle_slider_width;
            draw_rectangle(
                x_angle_handle_x - 5.0,
                angle_slider_y + 2.0,
                10.0,
                angle_slider_height - 4.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Y angle slider
            draw_text(
                "Y:",
                angle_slider_x,
                angle_slider_y + angle_slider_spacing - 5.0,
                14.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_text(
                "0°",
                angle_slider_x - 15.0,
                angle_slider_y + angle_slider_spacing + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "180°",
                angle_slider_x + angle_slider_width / 2.0 - 15.0,
                angle_slider_y + angle_slider_spacing + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "360°",
                angle_slider_x + angle_slider_width + 5.0,
                angle_slider_y + angle_slider_spacing + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_rectangle(
                angle_slider_x,
                angle_slider_y + angle_slider_spacing,
                angle_slider_width,
                angle_slider_height,
                if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
            );
            
            // Y angle slider handle (normalize to 0-360 degrees)
            let y_degrees = viewer.rotation_y.to_degrees() % 360.0;
            let y_normalized = if y_degrees < 0.0 { y_degrees + 360.0 } else { y_degrees };
            let y_angle_handle_x = angle_slider_x + (y_normalized / 360.0) * angle_slider_width;
            draw_rectangle(
                y_angle_handle_x - 5.0,
                angle_slider_y + angle_slider_spacing + 2.0,
                10.0,
                angle_slider_height - 4.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Z angle slider
            draw_text(
                "Z:",
                angle_slider_x,
                angle_slider_y + angle_slider_spacing * 2.0 - 5.0,
                14.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_text(
                "0°",
                angle_slider_x - 15.0,
                angle_slider_y + angle_slider_spacing * 2.0 + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "180°",
                angle_slider_x + angle_slider_width / 2.0 - 15.0,
                angle_slider_y + angle_slider_spacing * 2.0 + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "360°",
                angle_slider_x + angle_slider_width + 5.0,
                angle_slider_y + angle_slider_spacing * 2.0 + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_rectangle(
                angle_slider_x,
                angle_slider_y + angle_slider_spacing * 2.0,
                angle_slider_width,
                angle_slider_height,
                if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
            );
            
            // Z angle slider handle (normalize to 0-360 degrees)
            let z_degrees = viewer.rotation_z.to_degrees() % 360.0;
            let z_normalized = if z_degrees < 0.0 { z_degrees + 360.0 } else { z_degrees };
            let z_angle_handle_x = angle_slider_x + (z_normalized / 360.0) * angle_slider_width;
            draw_rectangle(
                z_angle_handle_x - 5.0,
                angle_slider_y + angle_slider_spacing * 2.0 + 2.0,
                10.0,
                angle_slider_height - 4.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            } // End of 3D rotation sliders
            
            // Velocity sliders
            let slider_x = panel_x + 20.0;
            let slider_y = panel_y + 180.0;
            let slider_width = 120.0; // Reduced width to match angle sliders
            let slider_height = 20.0;
            let slider_spacing = 35.0;
            
            if viewer.is_4d_mode {
                // Right column: Velocity sliders
                let rotation_velocities = [
                    (viewer.rotation_velocity_xy, "dXY"),
                    (viewer.rotation_velocity_xz, "dXZ"),
                    (viewer.rotation_velocity_yz, "dYZ"),
                    (viewer.rotation_velocity_xw, "dXW"),
                    (viewer.rotation_velocity_yw, "dYW"),
                    (viewer.rotation_velocity_zw, "dZW"),
                ];
                
                // Position velocity sliders in right column
                let velocity_slider_x = panel_x + 150.0; // Right column position, closer to center
                
                for (i, (rotation_velocity, label)) in rotation_velocities.iter().enumerate() {
                    let y_pos = angle_slider_y + (i as f32) * angle_slider_spacing; // Same Y as angle sliders
                    
                    // Label
                    draw_text(
                        label,
                        velocity_slider_x,
                        y_pos - 5.0,
                        14.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    
                    // Slider labels (-, 0, +)
                    draw_text(
                        "-",
                        velocity_slider_x - 10.0,
                        y_pos + 15.0,
                        12.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    draw_text(
                        "0",
                        velocity_slider_x + slider_width / 2.0 - 3.0,
                        y_pos + 15.0,
                        12.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    draw_text(
                        "+",
                        velocity_slider_x + slider_width + 5.0,
                        y_pos + 15.0,
                        12.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                    
                    // Slider track
                    draw_rectangle(
                        velocity_slider_x,
                        y_pos,
                        slider_width,
                        slider_height,
                        if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
                    );
                    
                    // Center line on slider
                    draw_line(
                        velocity_slider_x + slider_width / 2.0,
                        y_pos,
                        velocity_slider_x + slider_width / 2.0,
                        y_pos + slider_height,
                        1.0,
                        if viewer.dark_background { Color::new(0.5, 0.5, 0.5, 1.0) } else { Color::new(0.3, 0.3, 0.3, 1.0) }
                    );
                    
                    // Slider handle (normalize velocity to slider range)
                    let handle_x = velocity_slider_x + (rotation_velocity + 0.02) / 0.04 * slider_width;
                    draw_rectangle(
                        handle_x - 5.0,
                        y_pos + 2.0,
                        10.0,
                        slider_height - 4.0,
                        if viewer.dark_background { WHITE } else { BLACK }
                    );
                }
            } else {
                // Draw 3D velocity sliders (X, Y, Z)
            
            // X velocity slider
            draw_text(
                "dX:",
                slider_x,
                slider_y - 5.0,
                14.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Slider labels (-, 0, +)
            draw_text(
                "-",
                slider_x - 10.0,
                slider_y + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "0",
                slider_x + slider_width / 2.0 - 3.0,
                slider_y + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "+",
                slider_x + slider_width + 5.0,
                slider_y + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Slider track
            draw_rectangle(
                slider_x,
                slider_y,
                slider_width,
                slider_height,
                if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
            );
            
            // Center line on slider
            draw_line(
                slider_x + slider_width / 2.0,
                slider_y,
                slider_x + slider_width / 2.0,
                slider_y + slider_height,
                1.0,
                if viewer.dark_background { Color::new(0.5, 0.5, 0.5, 1.0) } else { Color::new(0.3, 0.3, 0.3, 1.0) }
            );
            
            // Slider handle
            let x_handle_x = slider_x + (viewer.rotation_velocity_x + 0.02) / 0.04 * slider_width;
            draw_rectangle(
                x_handle_x - 5.0,
                slider_y + 2.0,
                10.0,
                slider_height - 4.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Y velocity slider
            draw_text(
                "dY:",
                slider_x,
                slider_y + slider_spacing - 5.0,
                14.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Slider labels (-, 0, +)
            draw_text(
                "-",
                slider_x - 10.0,
                slider_y + slider_spacing + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "0",
                slider_x + slider_width / 2.0 - 3.0,
                slider_y + slider_spacing + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "+",
                slider_x + slider_width + 5.0,
                slider_y + slider_spacing + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_rectangle(
                slider_x,
                slider_y + slider_spacing,
                slider_width,
                slider_height,
                if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
            );
            
            // Center line on slider
            draw_line(
                slider_x + slider_width / 2.0,
                slider_y + slider_spacing,
                slider_x + slider_width / 2.0,
                slider_y + slider_spacing + slider_height,
                1.0,
                if viewer.dark_background { Color::new(0.5, 0.5, 0.5, 1.0) } else { Color::new(0.3, 0.3, 0.3, 1.0) }
            );
            
            let y_handle_x = slider_x + (viewer.rotation_velocity_y + 0.02) / 0.04 * slider_width;
            draw_rectangle(
                y_handle_x - 5.0,
                slider_y + slider_spacing + 2.0,
                10.0,
                slider_height - 4.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Z velocity slider
            draw_text(
                "dZ:",
                slider_x,
                slider_y + slider_spacing * 2.0 - 5.0,
                14.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Slider labels (-, 0, +)
            draw_text(
                "-",
                slider_x - 10.0,
                slider_y + slider_spacing * 2.0 + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "0",
                slider_x + slider_width / 2.0 - 3.0,
                slider_y + slider_spacing * 2.0 + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            draw_text(
                "+",
                slider_x + slider_width + 5.0,
                slider_y + slider_spacing * 2.0 + 15.0,
                12.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_rectangle(
                slider_x,
                slider_y + slider_spacing * 2.0,
                slider_width,
                slider_height,
                if viewer.dark_background { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.7, 0.7, 0.7, 1.0) }
            );
            
            // Center line on slider
            draw_line(
                slider_x + slider_width / 2.0,
                slider_y + slider_spacing * 2.0,
                slider_x + slider_width / 2.0,
                slider_y + slider_spacing * 2.0 + slider_height,
                1.0,
                if viewer.dark_background { Color::new(0.5, 0.5, 0.5, 1.0) } else { Color::new(0.3, 0.3, 0.3, 1.0) }
            );
            
            let z_handle_x = slider_x + (viewer.rotation_velocity_z + 0.02) / 0.04 * slider_width;
            draw_rectangle(
                z_handle_x - 5.0,
                slider_y + slider_spacing * 2.0 + 2.0,
                10.0,
                slider_height - 4.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            } // End of 3D velocity sliders
            
            // Instructions
            draw_text(
                "Click and drag sliders to adjust rotation speed",
                panel_x + 10.0,
                panel_y + 310.0,
                12.0,
                if viewer.dark_background { Color::new(0.7, 0.7, 0.7, 1.0) } else { Color::new(0.4, 0.4, 0.4, 1.0) }
            );
        }
        
        // Mode and shape label below buttons - always visible
        let mode_text = if viewer.is_4d_mode {
            format!("4D {}", viewer.current_hypersolid.name())
        } else {
            format!("3D {}", viewer.current_solid.name())
        };
        draw_text(
            &mode_text,
            10.0,
            60.0, // Moved down below the buttons (which are at y=10-50)
            20.0,
            if viewer.dark_background { WHITE } else { BLACK }
        );
        
        if viewer.show_ui {
            draw_text(
                "3D/4D Hypersolids Stereogram Viewer",
                90.0,
                30.0,
                25.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_text(
                "Left Eye | Right Eye",
                10.0,
                60.0,
                20.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Show current mode (3D or 4D)
            let mode_text = if viewer.is_4d_mode {
                format!("4D Mode: {}", viewer.current_hypersolid.name())
            } else {
                format!("3D Mode: {}", viewer.current_solid.name())
            };
            draw_text(
                &mode_text,
                10.0,
                80.0,
                18.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Debug info for 4D mode (only show in 4D mode)
            if viewer.is_4d_mode {
                let vertices_4d = viewer.get_4d_vertices();
                if !vertices_4d.is_empty() {
                    let first_4d = &vertices_4d[0];
                    draw_text(&format!("4D: ({:.1}, {:.1}, {:.1}, {:.1})", first_4d.x, first_4d.y, first_4d.z, first_4d.w), 
                            10.0, 100.0, 16.0, BLUE);
                    draw_text(&format!("Vertices: {}", vertices_4d.len()), 
                            10.0, 120.0, 16.0, GREEN);
                }
            }
            
            // Show rotation angles based on mode
            let rotation_y_start = if viewer.is_4d_mode { 140.0 } else { 100.0 };
            
            if viewer.is_4d_mode {
                // Show 4D rotations
                let color = if viewer.dark_background { WHITE } else { BLACK };
                draw_text("4D Rotations:", 10.0, rotation_y_start, 16.0, color);
                draw_text(&format!("XY: {:.1}°", viewer.rotation_xy.to_degrees()), 10.0, rotation_y_start + 20.0, 16.0, color);
                draw_text(&format!("XZ: {:.1}°", viewer.rotation_xz.to_degrees()), 10.0, rotation_y_start + 40.0, 16.0, color);
                draw_text(&format!("YZ: {:.1}°", viewer.rotation_yz.to_degrees()), 10.0, rotation_y_start + 60.0, 16.0, color);
                draw_text(&format!("XW: {:.1}°", viewer.rotation_xw.to_degrees()), 10.0, rotation_y_start + 80.0, 16.0, color);
                draw_text(&format!("YW: {:.1}°", viewer.rotation_yw.to_degrees()), 10.0, rotation_y_start + 100.0, 16.0, color);
                draw_text(&format!("ZW: {:.1}°", viewer.rotation_zw.to_degrees()), 10.0, rotation_y_start + 120.0, 16.0, color);
            } else {
                // Show 3D rotations
                draw_text(
                    &format!("Rotation X: {:.1}°", viewer.rotation_x.to_degrees()),
                    10.0,
                    rotation_y_start,
                    18.0,
                    if viewer.dark_background { WHITE } else { BLACK }
                );
                
                draw_text(
                    &format!("Rotation Y: {:.1}°", viewer.rotation_y.to_degrees()),
                    10.0,
                    rotation_y_start + 20.0,
                    18.0,
                    if viewer.dark_background { WHITE } else { BLACK }
                );
                
                draw_text(
                    &format!("Rotation Z: {:.1}°", viewer.rotation_z.to_degrees()),
                    10.0,
                    rotation_y_start + 40.0,
                    18.0,
                    if viewer.dark_background { WHITE } else { BLACK }
                );
            }
            
            
            // Show pause status
            if viewer.is_paused {
                draw_text(
                    "PAUSED",
                    10.0,
                    165.0,
                    25.0,
                    RED
                );
            }
            
            draw_text(
                "Look \"through\" the image to see 3D effect!",
                10.0,
                180.0,
                18.0,
                if viewer.dark_background { YELLOW } else { Color::new(0.8, 0.6, 0.0, 1.0) } // Dark yellow for white background
            );
            
            draw_text(
                "Press SPACE to pause/resume animation",
                10.0,
                210.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) } // Dark green for white background
            );
            
            draw_text(
                "Press G to toggle guides",
                10.0,
                235.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press Z to toggle depth coloring",
                10.0,
                260.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press W to toggle W-depth coloring",
                10.0,
                280.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press H to toggle 3D/4D mode",
                10.0,
                300.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press J to cycle through hypersolids (4D only)",
                10.0,
                320.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press B to toggle background (black/white)",
                10.0,
                340.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press T to toggle all text/UI",
                10.0,
                360.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press O to toggle orthographic/perspective projection",
                10.0,
                380.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press S to cycle through Platonic solids",
                10.0,
                400.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press LEFT/RIGHT to adjust eye separation",
                10.0,
                420.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press UP/DOWN to adjust perspective (distance + scale)",
                10.0,
                440.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Fusion tips: Focus THROUGH screen, merge red circles",
                10.0,
                480.0,
                18.0,
                if viewer.dark_background { YELLOW } else { Color::new(0.8, 0.6, 0.0, 1.0) }
            );
            
            // Show current eye separation value
            draw_text(
                &format!("Eye Separation: {:.3}", viewer.eye_separation),
                10.0,
                375.0,
                18.0,
                if viewer.dark_background { ORANGE } else { Color::new(0.8, 0.4, 0.0, 1.0) } // Dark orange for white background
            );
            
            // Show current perspective distance
            draw_text(
                &format!("Perspective Distance: {:.1}", viewer.perspective_distance),
                10.0,
                400.0,
                18.0,
                if viewer.dark_background { ORANGE } else { Color::new(0.8, 0.4, 0.0, 1.0) }
            );
            
            // Show current projection mode
            draw_text(
                &format!("Projection: {}", if viewer.orthographic { "Orthographic" } else { "Perspective" }),
                10.0,
                425.0,
                18.0,
                if viewer.dark_background { ORANGE } else { Color::new(0.8, 0.4, 0.0, 1.0) }
            );
            
            // Show current solid
            draw_text(
                &format!("Solid: {}", viewer.current_solid.name()),
                10.0,
                450.0,
                18.0,
                if viewer.dark_background { ORANGE } else { Color::new(0.8, 0.4, 0.0, 1.0) }
            );
            

            

            
            // Draw distance indicator
            draw_text(
                &format!("Distance: {:.1} units", viewer.eye_separation * 2.0),
                10.0,
                400.0,
                16.0,
                ORANGE
            );
        }
        
        // Handle input
        if is_key_pressed(KeyCode::Space) {
            // Toggle pause state
            viewer.is_paused = !viewer.is_paused;
            // Reset debug flag when unpausing so we can print again on next pause
            if !viewer.is_paused {
                viewer.debug_printed = false;
            }
        }
        
        if is_key_pressed(KeyCode::G) {
            // Toggle guides visibility
            viewer.show_guides = !viewer.show_guides;
        }
        
        if is_key_pressed(KeyCode::Z) {
            // Toggle Z-depth coloring (disable W-depth if enabled)
            viewer.depth_coloring = !viewer.depth_coloring;
            if viewer.depth_coloring {
                viewer.w_depth_coloring = false; // Disable W-depth when Z-depth is enabled
            }
        }
        
        if is_key_pressed(KeyCode::W) {
            // Toggle W-depth coloring (disable Z-depth if enabled)
            viewer.w_depth_coloring = !viewer.w_depth_coloring;
            if viewer.w_depth_coloring {
                viewer.depth_coloring = false; // Disable Z-depth when W-depth is enabled
            }
        }
        
        if is_key_pressed(KeyCode::T) {
            // Toggle UI visibility
            viewer.show_ui = !viewer.show_ui;
        }
        
        if is_key_pressed(KeyCode::B) {
            // Toggle background color
            viewer.dark_background = !viewer.dark_background;
        }
        
        if is_key_pressed(KeyCode::O) {
            // Toggle orthographic/perspective projection
            viewer.orthographic = !viewer.orthographic;
        }
        
        if is_key_pressed(KeyCode::S) {
            // Cycle through Platonic solids
            viewer.current_solid = viewer.current_solid.next();
        }
        
        if is_key_pressed(KeyCode::H) {
            // Toggle between 3D and 4D modes
            viewer.is_4d_mode = !viewer.is_4d_mode;
        }
        
        if is_key_pressed(KeyCode::J) {
            // Cycle through hypersolids (only in 4D mode)
            if viewer.is_4d_mode {
                viewer.current_hypersolid = viewer.current_hypersolid.next();
            }
        }
        
        // Handle button clicks
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            
            // Help button (upper left)
            let help_button_x = 10.0;
            let help_button_y = 10.0;
            let button_size = 30.0;
            
            if mouse_pos.0 >= help_button_x && mouse_pos.0 <= help_button_x + button_size &&
               mouse_pos.1 >= help_button_y && mouse_pos.1 <= help_button_y + button_size {
                viewer.show_ui = !viewer.show_ui;
            }
            
            // 3D controls button (next to help button)
            let controls_button_x = 50.0;
            let controls_button_y = 10.0;
            
            if mouse_pos.0 >= controls_button_x && mouse_pos.0 <= controls_button_x + button_size &&
               mouse_pos.1 >= controls_button_y && mouse_pos.1 <= controls_button_y + button_size {
                viewer.show_3d_controls = !viewer.show_3d_controls;
            }
            
            // Handle slider interactions when 3D controls are visible
            if viewer.show_3d_controls {
                let panel_x = 10.0;
                let panel_y = 50.0;
                let slider_x = panel_x + 20.0;
                let slider_y = panel_y + 180.0;
                let slider_width = 200.0;
                let slider_height = 20.0;
                let slider_spacing = 35.0;
                
                // Check angle sliders first
                let angle_slider_y = panel_y + 50.0;
                let angle_slider_spacing = 35.0;
                
                if viewer.is_4d_mode {
                    // 4D mode: Check left column (angle sliders) and right column (velocity sliders)
                    let velocity_slider_x = panel_x + 150.0; // Right column position
                    
                    // Check left column for angle sliders
                    if mouse_pos.0 >= slider_x && mouse_pos.0 <= slider_x + slider_width {
                        for i in 0..6 {
                            let y_pos = angle_slider_y + (i as f32) * angle_slider_spacing;
                            if mouse_pos.1 >= y_pos && mouse_pos.1 <= y_pos + slider_height {
                                viewer.dragging_4d_slider = Some(i);
                                break;
                            }
                        }
                    }
                    
                    // Check right column for velocity sliders
                    if mouse_pos.0 >= velocity_slider_x && mouse_pos.0 <= velocity_slider_x + slider_width {
                        for i in 0..6 {
                            let y_pos = angle_slider_y + (i as f32) * angle_slider_spacing;
                            if mouse_pos.1 >= y_pos && mouse_pos.1 <= y_pos + slider_height {
                                viewer.dragging_slider = Some(i);
                                break;
                            }
                        }
                    }
                } else if mouse_pos.0 >= slider_x && mouse_pos.0 <= slider_x + slider_width {
                    // 3D mode: Check for 3 angle sliders (X, Y, Z)
                    if mouse_pos.1 >= angle_slider_y && mouse_pos.1 <= angle_slider_y + slider_height {
                        viewer.dragging_angle_slider = Some(0);
                    }
                    else if mouse_pos.1 >= angle_slider_y + angle_slider_spacing && mouse_pos.1 <= angle_slider_y + angle_slider_spacing + slider_height {
                        viewer.dragging_angle_slider = Some(1);
                    }
                    else if mouse_pos.1 >= angle_slider_y + angle_slider_spacing * 2.0 && mouse_pos.1 <= angle_slider_y + angle_slider_spacing * 2.0 + slider_height {
                        viewer.dragging_angle_slider = Some(2);
                    }
                    // Check for 3 velocity sliders (X, Y, Z)
                    else if mouse_pos.1 >= slider_y && mouse_pos.1 <= slider_y + slider_height {
                        viewer.dragging_slider = Some(0);
                    }
                    else if mouse_pos.1 >= slider_y + slider_spacing && mouse_pos.1 <= slider_y + slider_spacing + slider_height {
                        viewer.dragging_slider = Some(1);
                    }
                    else if mouse_pos.1 >= slider_y + slider_spacing * 2.0 && mouse_pos.1 <= slider_y + slider_spacing * 2.0 + slider_height {
                        viewer.dragging_slider = Some(2);
                    }
                }
            }
        }
        
        // Handle mouse release (stop dragging)
        if is_mouse_button_released(MouseButton::Left) {
            viewer.dragging_slider = None;
            viewer.dragging_angle_slider = None;
            viewer.dragging_4d_slider = None;
        }
        
        // Handle slider dragging
        if let Some(slider_index) = viewer.dragging_slider {
            let mouse_pos = mouse_position();
            let panel_x = 10.0;
            let slider_width = 120.0; // Match the reduced slider width
            
            // Use appropriate X position based on mode
            let slider_x = if viewer.is_4d_mode {
                panel_x + 150.0 // Right column for 4D velocity sliders
            } else {
                panel_x + 20.0  // Left column for 3D velocity sliders
            };
            
            let normalized = ((mouse_pos.0 - slider_x) / slider_width).clamp(0.0, 1.0);
            // Reduced range: -0.02 to 0.02 with larger discrete steps for easier centering
            let raw_velocity = (normalized * 0.04) - 0.02; // Range: -0.02 to 0.02
            
            // Create discrete steps: -0.02, -0.015, -0.01, -0.005, 0.00, 0.005, 0.01, 0.015, 0.02
            let step_size = 0.0025; // Halved step size for finer control
            let velocity = (raw_velocity / step_size).round() * step_size;
            let velocity = velocity.clamp(-0.02, 0.02);
            
            if viewer.is_4d_mode {
                // 4D velocity sliders
                match slider_index {
                    0 => viewer.rotation_velocity_xy = velocity,
                    1 => viewer.rotation_velocity_xz = velocity,
                    2 => viewer.rotation_velocity_yz = velocity,
                    3 => viewer.rotation_velocity_xw = velocity,
                    4 => viewer.rotation_velocity_yw = velocity,
                    5 => viewer.rotation_velocity_zw = velocity,
                    _ => {}
                }
            } else {
                // 3D velocity sliders
                match slider_index {
                    0 => viewer.rotation_velocity_x = velocity,
                    1 => viewer.rotation_velocity_y = velocity,
                    2 => viewer.rotation_velocity_z = velocity,
                    _ => {}
                }
            }
        }
        
        // Handle angle slider dragging
        if let Some(angle_slider_index) = viewer.dragging_angle_slider {
            let mouse_pos = mouse_position();
            let panel_x = 10.0;
            let slider_x = panel_x + 20.0;
            let slider_width = 200.0;
            
            let normalized = ((mouse_pos.0 - slider_x) / slider_width).clamp(0.0, 1.0);
            let degrees = normalized * 360.0;
            let radians = degrees.to_radians();
            
            match angle_slider_index {
                0 => viewer.rotation_x = radians,
                1 => viewer.rotation_y = radians,
                2 => viewer.rotation_z = radians,
                _ => {}
            }
        }
        
        // Handle 4D angle slider dragging
        if let Some(angle_slider_index) = viewer.dragging_4d_slider {
            let mouse_pos = mouse_position();
            let panel_x = 10.0;
            let slider_x = panel_x + 20.0;
            let slider_width = 200.0;
            
            let normalized = ((mouse_pos.0 - slider_x) / slider_width).clamp(0.0, 1.0);
            let degrees = normalized * 360.0;
            let radians = degrees.to_radians();
            
            match angle_slider_index {
                0 => viewer.rotation_xy = radians,
                1 => viewer.rotation_xz = radians,
                2 => viewer.rotation_yz = radians,
                3 => viewer.rotation_xw = radians,
                4 => viewer.rotation_yw = radians,
                5 => viewer.rotation_zw = radians,
                _ => {}
            }
        }
        
        // Adjust eye separation for parallel viewing
        if is_key_pressed(KeyCode::Left) {
            viewer.eye_separation = (viewer.eye_separation - 0.01).max(0.05);
        }
        if is_key_pressed(KeyCode::Right) {
            viewer.eye_separation = (viewer.eye_separation + 0.01).min(0.3);
        }
        
        // Adjust perspective distance and scale
        if is_key_pressed(KeyCode::Up) {
            viewer.perspective_distance = (viewer.perspective_distance + 0.5).min(20.0);
        }
        if is_key_pressed(KeyCode::Down) {
            viewer.perspective_distance = (viewer.perspective_distance - 0.5).max(2.0);
        }
        
        next_frame().await;
    }
}
