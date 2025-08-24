use macroquad::prelude::*;

// Cube vertex data (doubled in size)
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

// Cube edges (pairs of vertex indices)
const CUBE_EDGES: [(usize, usize); 12] = [
    // Back face edges
    (0, 1), (1, 2), (2, 3), (3, 0),
    // Front face edges
    (4, 5), (5, 6), (6, 7), (7, 4),
    // Connecting edges
    (0, 4), (1, 5), (2, 6), (3, 7),
];

struct StereogramViewer {
    rotation: f32,
    eye_separation: f32,
    is_paused: bool,
    show_guides: bool,
    depth_coloring: bool,
}

impl StereogramViewer {
    fn new() -> Self {
        Self {
            rotation: 0.0,
            eye_separation: 0.06, // Reduced for iPhone dimensions
            is_paused: false,
            show_guides: true,
            depth_coloring: true,
        }
    }

    fn update(&mut self) {
        if !self.is_paused {
            self.rotation += 0.01;
        }
    }

    fn draw_cube_wireframe(&self, camera_offset: f32, screen_offset_x: f32) {
        // Apply rotation to vertices first
        let rotation_y = self.rotation;
        let rotation_x = self.rotation * 0.5;

        // Create rotation matrices
        let rot_y_matrix = Mat4::from_rotation_y(rotation_y);
        let rot_x_matrix = Mat4::from_rotation_x(rotation_x);
        let combined_rotation = rot_y_matrix * rot_x_matrix;

        // Transform and project vertices to 2D screen space manually
        let screen_center_x = screen_width() / 4.0 + screen_offset_x; // Quarter width + offset for each view
        let screen_center_y = screen_height() / 2.0;
        let scale = 180.0; // Reduced for iPhone dimensions
        let perspective_distance = 4.0; // Doubled from 2.0 to reduce perspective distortion

        // Project each vertex to 2D screen coordinates
        let mut projected_vertices = Vec::new();
        let mut transformed_vertices = Vec::new();
        
        for &vertex in &CUBE_VERTICES {
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
            
            // Simple perspective projection
            let projected_x = screen_center_x + (camera_adjusted.x * scale) / camera_adjusted.z;
            let projected_y = screen_center_y - (camera_adjusted.y * scale) / camera_adjusted.z;
            
            projected_vertices.push(Vec2::new(projected_x, projected_y));
        }

        // Draw wireframe edges using 2D lines with optional depth-based coloring
        for &(start_idx, end_idx) in &CUBE_EDGES {
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
                // Typical Z range is about 3.0 to 5.0, so we'll normalize around that
                let min_z = 3.0;
                let max_z = 5.0;
                let intensity = ((max_z - avg_z) / (max_z - min_z)).clamp(0.2, 1.0);
                
                // Create color based on depth
                Color::new(intensity, intensity, intensity, 1.0)
            } else {
                // Use plain white color
                WHITE
            };
            
            draw_line(start_2d.x, start_2d.y, end_2d.x, end_2d.y, 3.0, wire_color);
        }
    }

    fn render_stereogram(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let half_width = screen_width / 2.0;
        
        // Clear the screen
        clear_background(BLACK);
        
        // Use default 2D camera for all rendering
        set_default_camera();
        
        // Render left eye view (left half of screen)
        self.draw_cube_wireframe(-self.eye_separation, 0.0);
        
        // Render right eye view (right half of screen)  
        self.draw_cube_wireframe(self.eye_separation, half_width);
        
        // Draw guides only if show_guides is true
        if self.show_guides {
            // Draw a dividing line between the two views
            draw_line(half_width, 0.0, half_width, screen_height, 2.0, GRAY);
            
            // Add fusion aids - small circles at the top to help with alignment
            let fusion_aid_y = 30.0;
            let fusion_aid_size = 6.0;
            
            // Left fusion aid (in left half)
            let left_aid_x = half_width / 2.0;
            draw_circle(left_aid_x, fusion_aid_y, fusion_aid_size, RED);
            draw_circle(left_aid_x, fusion_aid_y, fusion_aid_size - 2.0, WHITE);
            
            // Right fusion aid (in right half)
            let right_aid_x = half_width + half_width / 2.0;
            draw_circle(right_aid_x, fusion_aid_y, fusion_aid_size, RED);
            draw_circle(right_aid_x, fusion_aid_y, fusion_aid_size - 2.0, WHITE);
            
            // Add corner markers for easier alignment
            let corner_size = 4.0;
            // Top corners of each view
            draw_circle(5.0, 5.0, corner_size, GREEN);
            draw_circle(half_width - 5.0, 5.0, corner_size, GREEN);
            draw_circle(half_width + 5.0, 5.0, corner_size, GREEN);
            draw_circle(screen_width - 5.0, 5.0, corner_size, GREEN);
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
        
        draw_text(
            "iPhone Stereogram Viewer",
            10.0,
            30.0,
            25.0,
            WHITE
        );
        
        draw_text(
            "Left Eye | Right Eye",
            10.0,
            60.0,
            20.0,
            WHITE
        );
        
        draw_text(
            &format!("Rotation: {:.1}°", viewer.rotation.to_degrees()),
            10.0,
            90.0,
            20.0,
            WHITE
        );
        
        // Show pause status
        if viewer.is_paused {
            draw_text(
                "PAUSED",
                10.0,
                115.0,
                25.0,
                RED
            );
        }
        
        draw_text(
            "Look \"through\" the image to see 3D effect!",
            10.0,
            120.0,
            18.0,
            YELLOW
        );
        
        draw_text(
            "Press SPACE to pause/resume animation",
            10.0,
            150.0,
            18.0,
            LIME
        );
        
        draw_text(
            "Press G to toggle guides",
            10.0,
            175.0,
            18.0,
            LIME
        );
        
        draw_text(
            "Press C to toggle depth coloring",
            10.0,
            200.0,
            18.0,
            LIME
        );
        
        draw_text(
            "Press LEFT/RIGHT to adjust eye separation",
            10.0,
            225.0,
            18.0,
            LIME
        );
        
        draw_text(
            "Fusion tips: Focus THROUGH screen, merge red circles",
            10.0,
            330.0,
            18.0,
            YELLOW
        );
        
        // Show current eye separation value
        draw_text(
            &format!("Eye Separation: {:.3}", viewer.eye_separation),
            10.0,
            255.0,
            18.0,
            ORANGE
        );
        
        // Visual representation of camera positions
        let camera_visual_y = 295.0;
        let screen_center = screen_width() / 2.0;
        let visual_scale = 100.0; // Scale factor for visual representation
        
        // Draw center line
        draw_line(screen_center, camera_visual_y - 20.0, screen_center, camera_visual_y + 20.0, 2.0, GRAY);
        
        // Draw left camera position
        let left_cam_x = screen_center - (viewer.eye_separation * visual_scale);
        draw_circle(left_cam_x, camera_visual_y, 8.0, RED);
        draw_text("L", left_cam_x - 5.0, camera_visual_y + 5.0, 15.0, WHITE);
        
        // Draw right camera position
        let right_cam_x = screen_center + (viewer.eye_separation * visual_scale);
        draw_circle(right_cam_x, camera_visual_y, 8.0, BLUE);
        draw_text("R", right_cam_x - 5.0, camera_visual_y + 5.0, 15.0, WHITE);
        
        // Draw distance indicator
        draw_text(
            &format!("Distance: {:.1} units", viewer.eye_separation * 2.0),
            10.0,
            325.0,
            16.0,
            ORANGE
        );
        
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
        
        // Adjust eye separation for parallel viewing
        if is_key_pressed(KeyCode::Left) {
            viewer.eye_separation = (viewer.eye_separation - 0.01).max(0.05);
        }
        if is_key_pressed(KeyCode::Right) {
            viewer.eye_separation = (viewer.eye_separation + 0.01).min(0.3);
        }
        
        next_frame().await;
    }
}
