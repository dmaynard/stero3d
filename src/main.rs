use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlatonicSolid {
    Tetrahedron = 0,
    Cube = 1,
    Octahedron = 2,
    Dodecahedron = 3,
    Icosahedron = 4,
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

struct StereogramViewer {
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    rotation_velocity_x: f32,
    rotation_velocity_y: f32,
    rotation_velocity_z: f32,
    eye_separation: f32,
    perspective_distance: f32,
    is_paused: bool,
    show_guides: bool,
    depth_coloring: bool,
    show_ui: bool,
    dark_background: bool,
    orthographic: bool,
    current_solid: PlatonicSolid,
    show_3d_controls: bool,
    dragging_slider: Option<usize>, // None, Some(0) for X, Some(1) for Y, Some(2) for Z
    dragging_angle_slider: Option<usize>, // None, Some(0) for X angle, Some(1) for Y angle, Some(2) for Z angle
}

impl StereogramViewer {
    fn new() -> Self {
        Self {
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            rotation_velocity_x: 0.005, // Slower X rotation
            rotation_velocity_y: 0.01,  // Normal Y rotation
            rotation_velocity_z: 0.003, // Slower Z rotation
            eye_separation: 0.06, // Reduced for iPhone dimensions
            perspective_distance: 10.0, // Initial perspective distance
            is_paused: false,
            show_guides: true,
            depth_coloring: true,
            #[cfg(target_arch = "wasm32")]
            show_ui: false, // Web users see HTML instructions, so hide UI by default
            #[cfg(not(target_arch = "wasm32"))]
            show_ui: true, // Native app users need UI visible by default
            dark_background: false, // Default to white background
            orthographic: false, // Perspective projection is default
            current_solid: PlatonicSolid::Cube, // Default to cube
            show_3d_controls: false, // Default to off
            dragging_slider: None, // No slider being dragged initially
            dragging_angle_slider: None, // No angle slider being dragged initially
        }
    }

    fn update(&mut self) {
        if !self.is_paused {
            self.rotation_x += self.rotation_velocity_x;
            self.rotation_y += self.rotation_velocity_y;
            self.rotation_z += self.rotation_velocity_z;
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

        // Draw wireframe edges using 2D lines with optional depth-based coloring
        let edges = self.get_edges();
        for &(start_idx, end_idx) in edges {
            let start_2d = projected_vertices[start_idx];
            let end_2d = projected_vertices[end_idx];
            
            let wire_color = if self.depth_coloring {
                // Get 3D positions for depth calculation
                let start_3d = transformed_vertices[start_idx];
                let end_3d = transformed_vertices[end_idx];
                
                // Calculate average Z distance (after camera adjustment)
                let start_z = start_3d.z - camera_offset + perspective_distance;
                let end_z = end_3d.z - camera_offset + perspective_distance;
                let avg_z = (start_z + end_z) / 2.0;
                
                // Map Z distance to color intensity (closer = brighter, farther = darker)
                // Dynamic Z range based on current perspective distance
                let min_z = self.perspective_distance - 1.0;
                let max_z = self.perspective_distance + 1.0;
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
            
            draw_line(start_2d.x, start_2d.y, end_2d.x, end_2d.y, 3.0, wire_color);
        }
    }

    fn render_stereogram(&self) {
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
        self.draw_solid_wireframe(-self.eye_separation, 0.0);
        
        // Render right eye view (right half of screen)  
        self.draw_solid_wireframe(self.eye_separation, half_width);
        
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
        draw_text(
            "3D",
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
            draw_text(
                "3D Rotation Controls",
                panel_x + 10.0,
                panel_y + 25.0,
                20.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Rotation angle sliders (0-360 degrees)
            let angle_slider_x = panel_x + 20.0;
            let angle_slider_y = panel_y + 50.0;
            let angle_slider_width = 200.0;
            let angle_slider_height = 20.0;
            let angle_slider_spacing = 35.0;
            
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
            
            // Velocity sliders
            let slider_x = panel_x + 20.0;
            let slider_y = panel_y + 180.0;
            let slider_width = 200.0;
            let slider_height = 20.0;
            let slider_spacing = 35.0;
            
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
            let x_handle_x = slider_x + (viewer.rotation_velocity_x + 0.05) / 0.10 * slider_width;
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
            
            let y_handle_x = slider_x + (viewer.rotation_velocity_y + 0.05) / 0.10 * slider_width;
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
            
            let z_handle_x = slider_x + (viewer.rotation_velocity_z + 0.05) / 0.10 * slider_width;
            draw_rectangle(
                z_handle_x - 5.0,
                slider_y + slider_spacing * 2.0 + 2.0,
                10.0,
                slider_height - 4.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Instructions
            draw_text(
                "Click and drag sliders to adjust rotation speed",
                panel_x + 10.0,
                panel_y + 310.0,
                12.0,
                if viewer.dark_background { Color::new(0.7, 0.7, 0.7, 1.0) } else { Color::new(0.4, 0.4, 0.4, 1.0) }
            );
        }
        
        if viewer.show_ui {
            draw_text(
                "3D Platonic Solids Stereogram Viewer",
                50.0,
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
            
            draw_text(
                &format!("Rotation X: {:.1}°", viewer.rotation_x.to_degrees()),
                10.0,
                90.0,
                18.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_text(
                &format!("Rotation Y: {:.1}°", viewer.rotation_y.to_degrees()),
                10.0,
                110.0,
                18.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            draw_text(
                &format!("Rotation Z: {:.1}°", viewer.rotation_z.to_degrees()),
                10.0,
                130.0,
                18.0,
                if viewer.dark_background { WHITE } else { BLACK }
            );
            
            // Show pause status
            if viewer.is_paused {
                draw_text(
                    "PAUSED",
                    10.0,
                    155.0,
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
                "Press C to toggle depth coloring",
                10.0,
                260.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press B to toggle background (black/white)",
                10.0,
                285.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press T to toggle all text/UI",
                10.0,
                310.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press O to toggle orthographic/perspective projection",
                10.0,
                335.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press S to cycle through Platonic solids",
                10.0,
                360.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press LEFT/RIGHT to adjust eye separation",
                10.0,
                385.0,
                18.0,
                if viewer.dark_background { LIME } else { Color::new(0.0, 0.4, 0.0, 1.0) }
            );
            
            draw_text(
                "Press UP/DOWN to adjust perspective (distance + scale)",
                10.0,
                410.0,
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
        }
        
        if is_key_pressed(KeyCode::G) {
            // Toggle guides visibility
            viewer.show_guides = !viewer.show_guides;
        }
        
        if is_key_pressed(KeyCode::C) {
            // Toggle depth coloring
            viewer.depth_coloring = !viewer.depth_coloring;
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
                
                if mouse_pos.0 >= slider_x && mouse_pos.0 <= slider_x + slider_width {
                    // X angle slider
                    if mouse_pos.1 >= angle_slider_y && mouse_pos.1 <= angle_slider_y + slider_height {
                        viewer.dragging_angle_slider = Some(0);
                    }
                    // Y angle slider
                    else if mouse_pos.1 >= angle_slider_y + angle_slider_spacing && mouse_pos.1 <= angle_slider_y + angle_slider_spacing + slider_height {
                        viewer.dragging_angle_slider = Some(1);
                    }
                    // Z angle slider
                    else if mouse_pos.1 >= angle_slider_y + angle_slider_spacing * 2.0 && mouse_pos.1 <= angle_slider_y + angle_slider_spacing * 2.0 + slider_height {
                        viewer.dragging_angle_slider = Some(2);
                    }
                    // X velocity slider
                    else if mouse_pos.1 >= slider_y && mouse_pos.1 <= slider_y + slider_height {
                        viewer.dragging_slider = Some(0);
                    }
                    // Y velocity slider
                    else if mouse_pos.1 >= slider_y + slider_spacing && mouse_pos.1 <= slider_y + slider_spacing + slider_height {
                        viewer.dragging_slider = Some(1);
                    }
                    // Z velocity slider
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
        }
        
        // Handle slider dragging
        if let Some(slider_index) = viewer.dragging_slider {
            let mouse_pos = mouse_position();
            let panel_x = 10.0;
            let slider_x = panel_x + 20.0;
            let slider_width = 200.0;
            
            let normalized = ((mouse_pos.0 - slider_x) / slider_width).clamp(0.0, 1.0);
            // Increased range: -0.05 to 0.05 with discrete steps
            let raw_velocity = (normalized * 0.10) - 0.05; // Range: -0.05 to 0.05
            
            // Create discrete steps: -0.05, -0.045, -0.04, ..., 0.00, ..., 0.04, 0.045, 0.05
            let step_size = 0.005;
            let velocity = (raw_velocity / step_size).round() * step_size;
            let velocity = velocity.clamp(-0.05, 0.05);
            
            match slider_index {
                0 => viewer.rotation_velocity_x = velocity,
                1 => viewer.rotation_velocity_y = velocity,
                2 => viewer.rotation_velocity_z = velocity,
                _ => {}
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
