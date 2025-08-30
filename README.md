# Stereo3D - 3D Stereogram Cube Viewer

A real-time 3D stereogram viewer built with Rust and Macroquad that displays a rotating wireframe cube as a parallel stereogram for 3D viewing without special glasses.

## Features

- **3D Stereogram Rendering**: View a rotating 3D cube in true 3D using parallel stereogram technique
- **Wireframe Display**: Clean wireframe rendering with depth-based coloring
- **Real-time Animation**: Smooth rotation with pause/resume functionality
- **Adjustable Viewing**: Fine-tune eye separation and perspective for optimal 3D fusion
- **Visual Fusion Aids**: Red guide circles to help merge the two images
- **Multiple Display Modes**: Toggle between dark/light backgrounds and depth coloring
- **iPhone-Optimized**: Window dimensions optimized for iPhone 16 portrait mode

## How It Works

A stereogram creates a 3D illusion by presenting slightly different images to each eye. This app renders the same 3D cube from two camera positions (left and right eye), separated by a configurable distance. When viewed correctly, your brain merges these two images into a single 3D scene.

## Controls

### Animation
- **SPACE** - Toggle pause/resume animation

### Viewing Adjustments
- **LEFT/RIGHT Arrow** - Adjust eye separation (0.01 to 0.20)
- **UP/DOWN Arrow** - Adjust perspective distance and cube scale (2.0 to 20.0)

### Display Options
- **G** - Toggle fusion guides (red circles)
- **C** - Toggle depth-based wireframe coloring
- **B** - Toggle between black/white backgrounds
- **O** - Toggle orthographic/perspective projection
- **T** - Toggle all on-screen text and UI elements

## How to View the Stereogram

1. **Start close to the screen** (12-18 inches) for easier initial fusion
2. **Relax your eyes** and look "through" the screen (like looking into the distance)
3. **Focus on the red guide circles** at the top of each view
4. **Let your eyes merge** the two images - you should see three cubes (left, right, and a floating 3D cube in the center)
5. **Focus on the center cube** - this is your 3D stereogram!
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

## Projection Modes

The app supports two projection modes that can be toggled with the **O** key:

- **Perspective (default)**: Traditional 3D view with realistic depth perspective where objects appear smaller as they get farther away
- **Orthographic**: Parallel projection with no perspective distortion - can be easier to fuse and shows geometric relationships more clearly

## Technical Details

- **Language**: Rust
- **Graphics Engine**: Macroquad
- **Rendering**: Manual 3D-to-2D projection for precise stereogram control with both perspective and orthographic modes
- **Window Size**: 393x852 pixels (iPhone 16 portrait dimensions)
- **3D Math**: Custom rotation matrices and configurable projection types
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
