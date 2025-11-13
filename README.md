# Stereo3D - 3D Stereogram Platonic Solids Viewer

🌐 **[Try it live in your browser!](https://dmaynard.github.io/stero3d/)**

A real-time 3D stereogram viewer built with Rust and Macroquad that displays rotating wireframe Platonic solids as parallel stereograms for 3D viewing without special glasses.

## Features

- **5 Platonic Solids**: Cycle through Tetrahedron, Cube, Octahedron, Dodecahedron, and Icosahedron
- **4D Hypersolids**: View 4-dimensional objects including Tesseract (4D hypercube), 4-Simplex, and 4-Orthoplex
- **3D/4D Mode Toggle**: Switch between 3D Platonic solids and 4D hypersolids
- **3D Stereogram Rendering**: View rotating 3D/4D objects in true 3D using parallel stereogram technique
- **Dual Projection Modes**: Switch between perspective and orthographic projection
- **Wireframe Display**: Clean 2-pixel wireframe rendering with depth-based coloring
- **Z-Depth Coloring**: Visualize 3D depth using gradient coloring based on Z-coordinate
- **W-Depth Coloring**: Visualize 4th dimension depth using gradient coloring based on W-coordinate (4D mode only)
- **Real-time Animation**: Smooth rotation with pause/resume functionality
- **Manual Rotation Controls**: Adjust rotation angles and velocities with interactive sliders
- **Adjustable Viewing**: Fine-tune eye separation and perspective for optimal 3D fusion
- **Visual Fusion Aids**: Red guide circles to help merge the two images
- **Multiple Display Modes**: Toggle between dark/light backgrounds and depth coloring
- **Organized UI**: Clean help panel layout that doesn't obstruct the stereogram view
- **Cross-Platform**: Native desktop app and web WASM version

## How It Works

A stereogram creates a 3D illusion by presenting slightly different images to each eye. This app renders the same 3D/4D object from two camera positions (left and right eye), separated by a configurable distance. For 4D objects, they are first projected from 4D space into 3D space, then rendered as a stereogram. When viewed correctly, your brain merges these two images into a single 3D scene, allowing you to perceive the depth and structure of the object.

## Controls

### Animation
- **SPACE** - Toggle pause/resume animation

### Viewing Adjustments
- **LEFT/RIGHT Arrow** - Adjust eye separation (0.01 to 0.20)
- **UP/DOWN Arrow** - Adjust perspective distance and object scale (2.0 to 20.0)

### Mode Selection
- **H** - Toggle between 3D and 4D modes
- **S** - Cycle through Platonic solids (3D mode only): Tetrahedron → Cube → Octahedron → Dodecahedron → Icosahedron
- **J** - Cycle through hypersolids (4D mode only): Tesseract → 4-Simplex → 4-Orthoplex

### Display Options
- **G** - Toggle fusion guides (red circles)
- **Z** - Toggle Z-depth coloring (visualizes 3D depth using Z-coordinate)
- **W** - Toggle W-depth coloring (visualizes 4th dimension depth using W-coordinate, 4D mode only)
- **B** - Toggle between black/white backgrounds
- **O** - Toggle orthographic/perspective projection
- **T** - Toggle all on-screen text and UI elements

### Manual Controls
- **3D/4D Button** (top left) - Open rotation controls panel with sliders
  - **3D Mode**: Control X, Y, Z rotation angles and velocities
  - **4D Mode**: Control 6 rotation planes (XY, XZ, YZ, XW, YW, ZW) with angles and velocities

## How to View the Stereogram

1. **Start close to the screen** (12-18 inches) for easier initial fusion
2. **Relax your eyes** and look "through" the screen (like looking into the distance)
3. **Focus on the red guide circles** at the top of each view
4. **Let your eyes merge** the two images - you should see three shapes (left, right, and a floating 3D shape in the center)
5. **Focus on the center shape** - this is your 3D stereogram!
6. **Once fused, slowly move back** to 2-3 feet to maintain the 3D effect

**Tips for easier fusion:**
- Start with the default settings
- Use the red guide circles as a target
- Start close (12-18 inches) for easier initial fusion
- Once fused, move back to 2-3 feet to maintain the effect
- Adjust eye separation if needed (LEFT/RIGHT arrows)
- Try different perspective distances (UP/DOWN arrows)
- Practice with the pause feature (SPACE) to hold still images

## Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo

### Development Build
```bash
cargo run
```

### Release Build (Recommended)
```bash
cargo run --release
```

### Build Only
```bash
cargo build --release
```

## Platonic Solids

The app features all five Platonic solids, which can be cycled through with the **S** key:

- **Tetrahedron**: 4 triangular faces, 4 vertices, 6 edges - the simplest 3D solid
- **Cube**: 6 square faces, 8 vertices, 12 edges - the most familiar solid
- **Octahedron**: 8 triangular faces, 6 vertices, 12 edges - dual of the cube
- **Dodecahedron**: 12 pentagonal faces, 20 vertices, 30 edges - uses golden ratio proportions
- **Icosahedron**: 20 triangular faces, 12 vertices, 30 edges - dual of the dodecahedron

Each solid demonstrates different geometric relationships and provides unique stereogram viewing experiences.

## 4D Hypersolids

The app features three 4-dimensional hypersolids that can be viewed by pressing the **H** key to enter 4D mode:

- **Tesseract (4D Hypercube)**: The 4D analogue of a cube, with 16 vertices, 32 edges, 24 square faces, and 8 cubic cells
- **4-Simplex (4D Tetrahedron)**: The 4D analogue of a tetrahedron, with 5 vertices, 10 edges, 10 triangular faces, and 5 tetrahedral cells
- **4-Orthoplex (4D Octahedron)**: The 4D analogue of an octahedron, with 8 vertices, 24 edges, 32 triangular faces, and 16 tetrahedral cells

These hypersolids are projected from 4D space into 3D space, then rendered as 3D stereograms. The **W-depth coloring** feature (W key) helps visualize the 4th dimension by coloring edges based on their W-coordinate values. The app supports rotation in all 6 rotation planes of 4D space (XY, XZ, YZ, XW, YW, ZW), allowing you to explore these fascinating 4D objects interactively.

## Projection Modes

The app supports two projection modes that can be toggled with the **O** key:

- **Perspective (default)**: Traditional 3D view with realistic depth perspective where objects appear smaller as they get farther away
- **Orthographic**: Parallel projection with no perspective distortion - can be easier to fuse and shows geometric relationships more clearly

## Technical Details

- **Language**: Rust
- **Graphics Engine**: Macroquad
- **Rendering**: Manual 3D-to-2D and 4D-to-3D-to-2D projection for precise stereogram control with both perspective and orthographic modes
- **Geometry**: 
  - All five Platonic solids with mathematically accurate vertices and edges
  - Three 4D hypersolids with proper 4D geometry and edge connections
- **Window Size**: 663x852 pixels (native app with help panel), 393x852 pixels optimal viewing width
- **3D/4D Math**: 
  - Custom rotation matrices for 3D (X, Y, Z axes)
  - 4D rotations in all 6 planes (XY, XZ, YZ, XW, YW, ZW)
  - Configurable projection types (perspective and orthographic)
  - Golden ratio calculations for Dodecahedron and Icosahedron
- **Depth Visualization**: 
  - Z-depth coloring uses gradient based on Z-coordinate (closer = darker on white background)
  - W-depth coloring uses gradient based on W-coordinate for 4D visualization
- **UI**: Organized help panel layout that doesn't obstruct the stereogram view
- **Performance**: Optimized for smooth 60fps animation

## Project Structure

```
stero3d/
├── Cargo.toml          # Rust dependencies and project configuration
├── src/
│   └── main.rs         # Main application code
└── README.md           # This file
```

## Dependencies

- `macroquad = "0.4"` - Cross-platform game framework
- `glam = "0.27"` - Math library for 3D vectors and matrices

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests for:
- Bug fixes
- New features
- Performance improvements
- Documentation updates

## Acknowledgments

- Built with [Macroquad](https://github.com/not-fl3/macroquad) - an excellent cross-platform game framework
- Inspired by classic stereogram techniques and 3D visualization methods
