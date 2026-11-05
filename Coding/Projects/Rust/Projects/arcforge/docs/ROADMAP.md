# Project Arcforge: A Holographic Workshop in Rust

A 44-week, single-project course. You learn Rust by building one app: a 3D object on your screen that you control with touch gestures, voice, and hand tracking, and that can be scanned from a real object with your camera.

## The Goal

By the end you will have **Arcforge**, a Rust application for Windows and Android that can:

- Render a lit 3D model on screen with an orbit camera
- Respond to mouse and touch gestures (drag, pinch-zoom, two-finger rotate)
- Accept typed and spoken commands ("rotate left", "make it red") with undo and redo
- Track your hand through the camera and let you grab, rotate and scale the object in mid-air
- Scan a real object from photos and bring it into the viewer as a 3D model

There are no mini projects. Every milestone adds a feature to Arcforge, and each Rust concept is learned when a feature needs it.

## Setup and Assumptions

- **Platforms:** Windows for development, Android for mobile and photo capture.
- **Pace:** about 10-12 hours per week, roughly 450-500 hours in total.
- **Third-party crates are allowed.** Where a crate hides a concept worth learning (math types, for example), you write a small version first, then switch to the crate.
- **Pre-trained models:** Rust runs the speech and hand-tracking models but does not create them.
- **Crate names and versions** are listed from memory. Check crates.io or lib.rs for current versions and maintenance status before committing to one.

## How We Learn

Each concept follows the same loop:

1. **Short explanation** with a mental model and a small example.
2. **Predict, then run:** guess whether code compiles and what it prints, then check.
3. **Fix the compiler error:** deliberately broken code teaches you to read Rust's error messages.
4. **Build it into Arcforge** from a task description and acceptance criteria, not from a solution.
5. **Code review:** you share your code and get feedback on correctness, idiomatic style and design.
6. **Check understanding** with short quizzes and "explain it back" questions.

When stuck, hints come in stages: a nudge, then the relevant concept, then a partial example, then the full solution only on request.

**Weekly rhythm:** 2 reading and drill sessions, 2 build sessions, 1 refactor and testing session.

**Starting a new chat:** paste your milestone, week, what you finished, what you are stuck on, and the relevant code.

## The Crate Stack

| Need | Crate(s) | Notes |
| --- | --- | --- |
| Window, input, touch | `winit` | Desktop and Android |
| GPU rendering | `wgpu` | Vulkan, Metal, DX12 |
| Math | `glam`, `nalgebra` | Write your own small version first |
| Model loading | `gltf`, `image` | glTF is the standard format |
| UI overlay | `egui` | Buttons, sliders, command box |
| Save and load | `serde` |  |
| Audio capture | `cpal` |  |
| Speech to text | `whisper-rs` | Offline, bindings to whisper.cpp |
| Camera | `nokhwa` (desktop), JNI (Android) |  |
| ML inference | `tract` or `ort` | Hand-tracking and depth models |
| Computer vision | `opencv` | Needed for scanning Route C |
| Parallelism | `rayon` |  |
| Mobile build | `android-activity`, `cargo-ndk` |  |

## Roadmap Overview

| Milestone | Weeks | Feature added to Arcforge |
| --- | --- | --- |
| M0 | 1-4 | Rust foundations and the OBJ model inspector |
| M1 | 5-9 | Core engine data: math, scene graph, commands |
| M2 | 10-15 | Window and GPU rendering |
| M3 | 16-18 | Mouse, touch and gestures |
| M4 | 19-22 | Command parser, UI, undo and redo |
| M5 | 23-26 | Voice control |
| M6 | 27-31 | Camera and hand tracking |
| M7 | 32-37 | Real-world scanning (Route C photogrammetry) |
| M8 | 38-43 | Android |
| M9 | 44 | Release |

## M0: Rust Foundations and the Model Inspector (Weeks 1-4)

**Feature:** a command-line tool that loads OBJ models and reports vertex and face counts, a bounding box and basic statistics.

**Week 1:** install Rust with rustup (and the Visual Studio C++ build tools on Windows), learn Cargo, variables, mutability, shadowing, scalar types and control flow. Do the three predict-then-run snippets and the temperature converter.

**Weeks 2-4 concepts:**

- Functions, expressions versus statements, tuples, arrays, slices
- `String` versus `&str`, `std::io`, `std::fs`, `std::fmt`
- Ownership, moves, `Copy` versus `Clone`, `Drop`
- References and the borrowing rules
- Structs, enums, `Option`, `match`, `if let`, `let else`
- `Result`, the `?` operator, custom error enums, `impl std::error::Error`
- `Vec`, `HashMap`, `HashSet`
- Modules, visibility, unit tests

**Reading:** The Rust Book chapters 1-10, plus Rustlings alongside.

**Done when:** the inspector loads real models, has no `unwrap` in its error paths, passes `cargo clippy` cleanly and has unit tests.

## M1: Core Engine Data (Weeks 5-9)

**Feature:** a headless library that loads a model, transforms it and runs a list of commands, with no window yet.

**Concepts:**

- Generics, trait bounds, `where` clauses, `impl Trait`
- Traits: default methods, associated types, supertraits, trait objects, object safety
- Standard traits: `Iterator`, `Default`, `PartialEq`, `Eq`, `Ord`, `Hash`, `From`, `Into`, `TryFrom`, `AsRef`, `Display`, `FromStr`
- Operator overloading by writing your own `Vec3` and `Mat4`, then replacing them with `glam`
- Closures: `Fn`, `FnMut`, `FnOnce`, capturing and returning closures
- Iterator adapters and writing your own iterator
- Lifetimes: annotations, elision, structs holding references
- Workspaces: split the project into crates such as `arc-math`, `arc-scene` and `arc-cli`

**Build:** a scene graph with transforms, and a `Command` enum (rotate, scale, set colour) that mutates it.

**Done when:** a scripted command list produces the expected scene state, verified by tests.

## M2: Window and GPU Rendering (Weeks 10-15)

**Feature:** a real window showing a lit, rotating glTF model with an orbit camera at 60 FPS.

**Concepts:**

- `Box`, `Rc`, `Arc`, `RefCell`, `Cell`, `Cow`, `Deref`
- `#[repr(C)]`, `bytemuck`-style casting, and your first `unsafe` with a written safety contract
- Event loops and the application lifecycle in `winit`
- GPU basics: buffers, pipelines, bind groups, depth buffers
- Shaders in WGSL: vertex and fragment stages, basic Lambert or Blinn-Phong lighting
- Fixed timestep and frame timing with `std::time::Instant`

**Done when:** a textured, lit model rotates smoothly and the camera orbits with the mouse.

## M3: Mouse, Touch and Gestures (Weeks 16-18)

**Feature:** your own gesture recognizer for drag, pinch-zoom and two-finger rotate.

**Concepts:**

- Pattern matching in depth: guards, `@` bindings, slice patterns
- State machines and the typestate pattern
- Channels (`std::sync::mpsc`) to decouple input from logic
- Input abstraction: turning raw events into semantic actions

**Done when:** all gestures work on a touchscreen or trackpad, and the gesture logic is covered by tests that replay recorded event sequences.

## M4: Commands, UI and Undo (Weeks 19-22)

**Feature:** every action in Arcforge is reachable through a typed command, with undo and redo and an `egui` overlay.

**Concepts:**

- Declarative macros (`macro_rules!`)
- Procedural macros: a `#[derive(Command)]` that removes parsing boilerplate
- Trait objects for the command pattern, `Box<dyn Command>`
- `serde` for saving and loading scenes
- Writing a small tokenizer and parser, and `FromStr`

**Done when:** you can type "make it red, rotate left 30", undo it, redo it, and save and reload the scene.

## M5: Voice Control (Weeks 23-26)

**Feature:** speak a command and Arcforge executes it.

**Concepts:**

- Threads, `thread::scope`, `Send` and `Sync`
- `Arc`, `Mutex`, `RwLock`, `Condvar`
- Atomics and memory ordering
- Lock-free ring buffer for audio samples
- Async fundamentals: `Future`, `Pin`, `Waker`. Write a tiny executor, then compare with `tokio`

**Build:** capture audio with `cpal`, detect speech, transcribe with `whisper-rs` on a worker thread, and feed the text into the M4 command parser.

**Done when:** saying "rotate left" rotates the object without freezing the render loop.

## M6: Camera and Hand Tracking (Weeks 27-31)

**Feature:** a live camera feed shown as a GPU texture, and hand-tracking control of the object.

**Concepts:**

- FFI in depth: `extern "C"`, `CString`, `CStr`, `MaybeUninit`, `ManuallyDrop`
- Unsafe Rust discipline: safe wrappers around unsafe code
- SIMD with `std::arch` for image preprocessing
- Performance: profiling, cache-friendly layouts, avoiding allocations in the frame loop
- Running ML inference with `tract` or `ort`

**Build:** run a hand-landmark model on camera frames, then map fingertip distances and positions to grab, rotate and scale actions.

**Done when:** you rotate and scale the object in mid-air with your hand at an acceptable frame rate.

## M7: Real-World Scanning, Route C Photogrammetry (Weeks 32-37)

**Feature:** photograph a real object with your Android phone, reconstruct it on the PC, and view it in Arcforge.

### What you need

| Item | Requirement |
| --- | --- |
| Camera | Your Android phone, with fixed focus and exposure if possible |
| PC | Any modern Windows machine, 16 GB or more of RAM recommended |
| Calibration target | A printed checkerboard on a rigid flat board |
| Optional | Turntable, tripod, diffuse lighting |
| Not needed | LiDAR, depth sensors, stereo rigs |

**Photography rules:** use textured, matte objects (shiny, transparent and plain white objects fail). Take 30-100 sharp photos with 60-80% overlap, in rings at two or three heights, with consistent lighting and no flash.

### Software on Windows

- Visual Studio Build Tools (C++ workload), LLVM/Clang, CMake
- OpenCV through vcpkg or a prebuilt package, and the `opencv` crate. Budget a few days for the setup
- `nalgebra`, `rayon`, and a sparse solver or `levenberg-marquardt`
- Optional external tools: COLMAP, OpenMVS, PoissonRecon

### The pipeline

1. **Camera calibration:** focal length, principal point and lens distortion from checkerboard photos (OpenCV).
2. **Feature detection:** SIFT or AKAZE (OpenCV).
3. **Matching and outlier rejection:** matching plus RANSAC with the essential matrix (OpenCV).
4. **Two-view initialization:** recover relative pose and triangulate the first points.
5. **Incremental registration:** add each image with PnP, triangulate new points, repeat. You write this orchestration.
6. **Bundle adjustment:** jointly refine cameras and points by minimizing reprojection error. OpenCV has no general bundle adjuster, so you write a sparse Levenberg-Marquardt solver in Rust.
7. **Dense reconstruction:** multi-view stereo to turn sparse points into a dense cloud.
8. **Meshing:** Poisson reconstruction or TSDF fusion with marching cubes.
9. **Texturing and export:** project photos onto the mesh and write glTF.

### Scope decision

Write steps 1-6 yourself in Rust. This is about 4-6 weeks and teaches the core ideas. For steps 7-9, use COLMAP or OpenMVS and import their output into Arcforge. Writing dense reconstruction, meshing and texturing from scratch is a research-level effort that could add 2-4 months. You can replace the external tools later if you want.

### Maths you will need

Linear algebra (SVD and least squares), the pinhole camera model, homographies, epipolar geometry (fundamental and essential matrices), RANSAC, and nonlinear optimization (Gauss-Newton and Levenberg-Marquardt). Learn these alongside the code.

### Rust concepts in this milestone

- Parallelism with `rayon` and your own thread pool
- Const generics for fixed-size matrices
- Advanced traits: GATs, blanket implementations, coherence
- Higher-ranked trait bounds (`for<'a>`) and variance
- Benchmarking and optimization of the solver

**Done when:** you scan a textured mug, reconstruct it, and load the result into Arcforge with correct scale and orientation.

## M8: Android (Weeks 38-43)

**Feature:** the core of Arcforge runs on your phone with touch, voice and camera.

**Concepts:**

- `#[cfg(target_os = ...)]` and platform-specific code
- Cross-compiling with `cargo-ndk`, and `android-activity` with `winit`
- JNI for permissions, camera and microphone access
- `no_std` and `alloc` basics, binary size, release profiles
- Performance and thermal limits on mobile

**Done when:** the viewer, gestures and voice commands work on your phone. Camera capture for scanning is the stretch goal.

**Risk:** build and permission issues are the main source of lost time here. Desktop is fully working by M7 regardless.

## M9: Release (Week 44)

- Profiling and optimization pass
- Integration tests and a test scene library
- `cargo doc`, a clear README and a recorded demo
- CI to build and test the project
- A packaged Windows build and an Android APK

## Concept Coverage Map

| Concept | Where it is learned |
| --- | --- |
| Ownership, borrowing, errors | M0 |
| Traits, generics, iterators, closures | M1 |
| Lifetimes, smart pointers, interior mutability | M1-M2 |
| Unsafe, `repr(C)`, FFI | M2, M6 |
| Pattern matching, typestate | M3 |
| Macros (declarative and procedural) | M4 |
| Concurrency, atomics, async | M5 |
| SIMD, const generics, GATs, HRTBs | M6-M7 |
| `cfg`, `no_std`, cross-compiling | M8 |
| Testing, docs, CI | Throughout, M9 |

## Risks and Mitigations

- **Mobile builds** are the largest risk. Get a minimal Android app running early in M2 to find problems sooner.
- **Scanning quality** depends on your photos. Start with a well-textured object and fixed lighting.
- **Setup friction on Windows** (OpenCV, C++ tools). Do the install in a dedicated session and keep notes.
- **Scope creep.** Finish each milestone's "done when" before starting the next one.
- **Crate churn.** Pin versions in `Cargo.lock` and check maintenance status before adopting a crate.

## Habits That Make This Work

- Keep one Cargo workspace with a crate per subsystem.
- Run `cargo clippy` and `cargo fmt` on every commit and use Git from day one.
- When the borrow checker blocks you, redesign the data layout rather than reaching for `Rc<RefCell<>>`.
- Keep a journal: each concept with a small example and one mistake you made.
- Record a short demo clip at the end of every milestone.

## Progress Tracker

| Milestone | Status | Notes |
| --- | --- | --- |
| M0 | Not started |  |
| M1 | Not started |  |
| M2 | Not started |  |
| M3 | Not started |  |
| M4 | Not started |  |
| M5 | Not started |  |
| M6 | Not started |  |
| M7 | Not started |  |
| M8 | Not started |  |
| M9 | Not started |  |
