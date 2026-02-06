# src/webgl/ — Rendering Engine

> Back to [ARCHITECTURE.md](../../ARCHITECTURE.md) | See also [src/shaders/README.md](../shaders/README.md)

This module contains the Rust-side rendering infrastructure: WebGL context
setup, shader compilation, the two-pass render pipeline, and the animation
frame scheduler.

---

## Module Map

```
webgl/
├── mod.rs          Public re-exports
├── context.rs      WebGL2 context initialization
├── shader.rs       GLSL compilation & program linking
├── pipeline.rs     AsciiPipeline: two-pass render orchestration
└── animation.rs    AnimationLoop: requestAnimationFrame manager
```

---

## context.rs

**Function:** `init_webgl2(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, String>`

Acquires a WebGL2 rendering context from an HTML canvas element.

1. Calls `canvas.get_context("webgl2")`
2. Casts the result to `WebGl2RenderingContext` via `dyn_into`
3. Sets initial clear color to `(0.04, 0.04, 0.04, 1.0)` — near-black
4. Performs an initial `clear(COLOR_BUFFER_BIT)`

Returns `Err` if WebGL2 is not supported or the cast fails.

**Note:** This function creates a *new* context each call. When called during
resize, the old context (and all its resources) becomes invalid.

---

## shader.rs

### `compile_shader(gl, shader_type, source) -> Result<WebGlShader, String>`

Compiles a single GLSL shader (vertex or fragment).

- Creates shader object with `gl.create_shader(shader_type)`
- Attaches source via `gl.shader_source()`
- Compiles with `gl.compile_shader()`
- On failure: retrieves info log, deletes the shader, returns error

### `link_program(gl, vert_src, frag_src) -> Result<WebGlProgram, String>`

Compiles both shaders and links them into a program.

1. `compile_shader(VERTEX_SHADER, vert_src)`
2. `compile_shader(FRAGMENT_SHADER, frag_src)`
3. `create_program()` → `attach_shader()` × 2 → `link_program()`
4. Immediately deletes both shader objects (program retains them internally)
5. On link failure: retrieves info log, deletes program, returns error

**Design note:** Shader sources are embedded at compile time via
`include_str!()` in `pipeline.rs`, not loaded at runtime.

---

## pipeline.rs

### Struct: `AsciiPipeline`

The central rendering orchestrator. Owns all GPU resources for the two-pass
ASCII rendering pipeline.

**Fields:**

| Field                | Type                         | Purpose                          |
|----------------------|------------------------------|----------------------------------|
| `gl`                 | `WebGl2RenderingContext`     | GPU API handle                   |
| `scene_program`      | `WebGlProgram`               | Pass 1 shader program            |
| `ascii_program`      | `WebGlProgram`               | Pass 2 shader program            |
| `framebuffer`        | `WebGlFramebuffer`           | Offscreen render target          |
| `fb_texture`         | `WebGlTexture`               | Color attachment for framebuffer |
| `empty_vao`          | `WebGlVertexArrayObject`     | Empty VAO (fullscreen triangle)  |
| `u_time`             | `Option<WebGlUniformLocation>` | scene: animation time          |
| `u_resolution_scene` | `Option<WebGlUniformLocation>` | scene: canvas dimensions       |
| `u_scene_tex`        | `Option<WebGlUniformLocation>` | ascii: scene texture sampler   |
| `u_resolution_ascii` | `Option<WebGlUniformLocation>` | ascii: canvas dimensions       |
| `u_cell_size`        | `Option<WebGlUniformLocation>` | ascii: character cell size     |
| `u_color_mix`        | `Option<WebGlUniformLocation>` | ascii: green ↔ color blend     |
| `width`, `height`    | `u32`                        | Canvas dimensions in pixels      |

### `AsciiPipeline::new(gl, width, height) -> Result<Self, String>`

Initialization sequence:

1. **Compile programs** — `link_program()` for scene and ascii shader pairs
2. **Cache uniforms** — `get_uniform_location()` for all 6 uniforms
3. **Create VAO** — Empty vertex array object (fullscreen triangle needs none)
4. **Create texture** — RGBA, `width × height`, LINEAR filtering, CLAMP_TO_EDGE
5. **Create framebuffer** — Attach texture as `COLOR_ATTACHMENT0`
6. **Verify** — `check_framebuffer_status()` must return `FRAMEBUFFER_COMPLETE`
7. **Unbind** — Restore default framebuffer and texture bindings

### `AsciiPipeline::render(time, cell_size, color_mix)`

Per-frame rendering. Called from the animation loop ~60× per second.

```
1. Bind empty VAO

2. Pass 1 — Scene → Framebuffer
   ├── Bind framebuffer (offscreen)
   ├── Set viewport to (width × height)
   ├── Clear color buffer
   ├── Use scene_program
   ├── Set u_time = time
   ├── Set u_resolution = (width, height)
   └── drawArrays(TRIANGLES, 0, 3)

3. Pass 2 — ASCII → Screen
   ├── Bind default framebuffer (screen)
   ├── Set viewport to (width × height)
   ├── Clear color buffer
   ├── Use ascii_program
   ├── Bind fb_texture to TEXTURE0
   ├── Set u_scene = 0 (texture unit)
   ├── Set u_resolution = (width, height)
   ├── Set u_cell_size = cell_size
   ├── Set u_color_mix = color_mix
   └── drawArrays(TRIANGLES, 0, 3)

4. Unbind VAO
```

---

## animation.rs

### Struct: `AnimationLoop`

Manages the `requestAnimationFrame` loop lifecycle. Dropping this struct
effectively stops the loop (the closure is freed).

**Field:** `_keep_alive: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>` — Prevents
the JavaScript closure from being garbage collected.

### `AnimationLoop::start(pipeline, cell_size, color_mix) -> Self`

Starts the rendering loop:

1. Wraps `pipeline` in `Rc<RefCell<_>>` for shared ownership
2. Creates a recursive `Closure<dyn FnMut(f64)>` that:
   - Converts timestamp (ms) to seconds: `time = timestamp / 1000.0`
   - Borrows `cell_size` and `color_mix` from their `Rc<RefCell<f32>>`
   - Calls `pipeline.borrow().render(time, cs, cm)`
   - Schedules the next frame via `window().request_animation_frame()`
3. Kicks off the first frame
4. Stores the closure in the `Rc<RefCell>` to prevent drop

**Ownership pattern:**
```
AnimationLoop
  └── _keep_alive: Rc<RefCell<Option<Closure>>>
        └── Closure captures:
              ├── pipeline: Rc<RefCell<AsciiPipeline>>
              ├── cell_size: Rc<RefCell<f32>>
              ├── color_mix: Rc<RefCell<f32>>
              └── cb_clone: Rc<RefCell<Option<Closure>>>  (self-reference for rAF)
```

The self-referential `Rc` pattern is necessary because `requestAnimationFrame`
needs a reference to the same closure to schedule the next frame.

---

## Interaction with Components

The `Hero` component in `src/components/hero.rs` is the sole consumer of this
module. It:

1. Creates the canvas element (Leptos `NodeRef`)
2. Calls `init_webgl2()` → `AsciiPipeline::new()` → `AnimationLoop::start()`
3. Bridges Leptos reactive signals to `Rc<RefCell<f32>>` via `Effect` hooks
4. Handles window resize by re-creating the entire pipeline

See [ARCHITECTURE.md](../../ARCHITECTURE.md) for the full data flow diagram.
