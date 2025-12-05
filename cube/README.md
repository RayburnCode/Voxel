<!-- @format -->

# Voxel Cube Renderer

A modular 3D cube renderer built with Rust, wgpu, and winit.

## Project Structure

The project has been organized into modular components for better maintainability:

```
cube/src/
├── main.rs         # Entry point and event loop
├── vertex.rs       # Vertex structure and layout
├── camera.rs       # Camera system with view/projection matrices
├── cube.rs         # Cube geometry and rendering
├── state.rs        # Application state and rendering pipeline
└── shader.wgsl     # WGSL shaders for vertex and fragment processing
```

## Modules

### `vertex.rs`

- Defines the `Vertex` structure with position, normal, and color data
- Provides vertex buffer layout for GPU rendering

### `camera.rs`

- `Camera`: Manages view and projection matrices
- `CameraUniform`: GPU uniform buffer for camera data
- `CameraController`: Handles camera updates and bind groups

### `cube.rs`

- `CubeRenderer`: Creates and renders a 3D cube
- Generates cube geometry with proper normals for lighting

### `state.rs`

- `State`: Main application state
- `Texture`: Depth texture for proper 3D rendering
- Manages the rendering pipeline and GPU resources

### `shader.wgsl`

- Vertex shader with camera transformations
- Fragment shader with:
  - Ambient lighting
  - Diffuse lighting
  - Shadow approximation based on surface orientation

## Features

✨ **3D Perspective Rendering** - Proper camera with perspective projection  
🎨 **Advanced Lighting** - Ambient and diffuse lighting with shadow effects  
📦 **Depth Buffer** - Correct depth testing for 3D scenes  
🏗️ **Modular Architecture** - Clean separation of concerns  
⚡ **GPU Accelerated** - Built on wgpu for modern graphics APIs

## Shadows Implementation

The current implementation includes:

- **Depth Testing**: Uses a depth buffer to ensure proper face ordering
- **Shadow Approximation**: Surfaces are darkened based on their orientation (downward-facing surfaces are darker)
- **Lighting Model**: Combines ambient and diffuse lighting for realistic appearance

The shadow effect is achieved through:

1. Surface normal analysis (faces pointing down get darker)
2. Smooth transitions using `smoothstep` function
3. Combined with directional lighting from top-right

## Controls

- **ESC** or **Close Button**: Exit the application
- Window is resizable and maintains proper aspect ratio

## Running

```bash
cargo run
```

## Dependencies

- `wgpu`: Modern GPU API
- `winit`: Window creation and event handling
- `cgmath`: Math library for 3D graphics
- `bytemuck`: Safe casting for GPU buffers
- `env_logger`: Logging support
- `pollster`: Async runtime for wgpu setup
