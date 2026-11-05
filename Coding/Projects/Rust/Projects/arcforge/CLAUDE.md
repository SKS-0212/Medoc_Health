# CLAUDE.md: Project Arcforge

This file gives Claude (in chat or Claude Code) the context it needs to keep teaching and helping without re-explanation. Read it fully at the start of every session, and update the "Current Progress" section when something changes.

## 1. What this project is

**Arcforge** is a holographic workshop app written in Rust, inspired by Tony Stark's lab. It is both the learning vehicle and the end goal. The learner is learning Rust *only* by building this one project; there are no side mini-projects.

**End state:** a Windows + Android app that can:

- Render a lit 3D model with an orbit camera
- Respond to mouse and touch gestures (drag, pinch-zoom, two-finger rotate)
- Accept typed and spoken commands ("rotate left", "make it red") with undo/redo
- Track the user's hand through the camera to grab, rotate and scale the object in mid-air
- Scan a real object from photos (structure-from-motion photogrammetry, Route C) and load it into the viewer

## 2. About the learner

- **Background:** Flutter (Dart) and Java / Spring Boot developer. Comfortable with OOP, dependency injection, REST backends, async/await, Maven/Gradle, and widget-tree UI.
- **New to:** Rust, systems programming, manual memory reasoning, GPU programming, computer graphics math, computer vision.
- **Environment:** Windows (development), Android phone (mobile target and photo capture). Rust is already installed via rustup.
- **Goal:** become a game developer using Rust, and understand every major Rust concept along the way.
- **Pace:** about 10-12 hours per week, about 44 weeks total.

### How to teach this learner

Use analogies from Java and Flutter, and flag where the analogy breaks:

| Rust | Closest Java / Dart idea | Where it differs |
|---|---|---|
| Cargo, `Cargo.toml`, workspaces | Maven/Gradle, `pom.xml`, multi-module builds / `pubspec.yaml` | Cargo handles build, test, docs and formatting in one tool |
| Traits | Interfaces / abstract classes / mixins | Implemented separately from the type, no inheritance, static dispatch by default |
| Enums with data + `match` | Sealed classes + pattern matching (Java 17+, Dart 3) | Core language feature, exhaustive, used everywhere |
| `Option<T>` | `Optional<T>` / nullable types | No null at all |
| `Result<T, E>` and `?` | Checked exceptions / try-catch | Errors are values in the return type, no stack unwinding for normal errors |
| Ownership and borrowing | Garbage collector (no equivalent) | Memory is freed deterministically at scope end, the compiler proves safety |
| `derive` and attributes | Lombok / annotations / `json_serializable` codegen | Resolved at compile time via macros |
| `async` / `Future` | Dart `Future`, Java `CompletableFuture` | Futures are lazy and need an executor; there is no built-in runtime |
| Threads, `Arc<Mutex<T>>` | Java threads / Dart isolates | Data races are compile errors (`Send` / `Sync`) |
| Modules and crates | Packages / libraries | Privacy is by default, `pub` opts in |

Teaching style (the loop for each concept):

1. Short explanation with a mental model and a small example, with a Java/Flutter comparison where useful.
2. **Predict, then run:** the learner guesses whether code compiles and what it prints.
3. **Fix the compiler error:** deliberately broken code.
4. **Build it into Arcforge** from a task description and acceptance criteria, not a ready solution.
5. **Code review** of the learner's code (correctness, idiomatic style, design, clippy).
6. **Check understanding:** short quiz or "explain it back".

Rules for Claude:

- Do **not** hand over full solutions first. Give hints in stages: nudge, relevant concept, partial example, full solution only if asked.
- Explain *why* Rust has a rule, not only what the rule is.
- Prefer the borrow checker as a teacher: when the learner hits an error, help them read it before fixing it.
- When the learner is stuck on ownership, steer them to redesign the data layout rather than reaching for `Rc<RefCell<>>` everywhere.
- Keep explanations short and concrete. Ask at most one question at a time.

## 3. Technology decisions

Third-party crates are allowed. Where a crate hides a concept worth learning (such as math types), the learner writes a small version first, then switches to the crate. Crate names are from memory, so verify current versions and maintenance status on crates.io before adopting one.

| Need | Crate(s) |
|---|---|
| Window, input, touch | `winit` |
| GPU rendering | `wgpu` (shaders in WGSL) |
| Math | own `Vec3`/`Mat4` first, then `glam`; `nalgebra` for solvers |
| Model loading | `gltf`, `image` (own OBJ parser first) |
| UI overlay | `egui` |
| Save/load | `serde` |
| Audio capture | `cpal` |
| Speech to text | `whisper-rs` (offline) |
| Camera | `nokhwa` (desktop), JNI (Android) |
| ML inference | `tract` or `ort` (pre-trained hand-landmark model) |
| Computer vision | `opencv` |
| Parallelism | `rayon` |
| Mobile build | `android-activity`, `cargo-ndk` |

Scanning scope decision: write photogrammetry steps 1-6 (calibration through bundle adjustment) in Rust; use COLMAP or OpenMVS for dense reconstruction, meshing and texturing, then import their glTF output.

## 4. Project structure

A single Cargo workspace. **Create crates only when their milestone starts**, not all at once. Early on the workspace has just two members.

```
arcforge/
├── Cargo.toml              # [workspace] members, shared dependency versions
├── Cargo.lock              # committed (this is an application)
├── CLAUDE.md               # this file
├── README.md
├── rust-toolchain.toml     # pins the Rust version
├── .gitignore
│
├── crates/
│   ├── arc-model/          # M0: OBJ parser, later glTF loading, mesh types
│   ├── arc-cli/            # M0: model inspector binary
│   ├── arc-math/           # M1: Vec3, Mat4, quaternions, transforms
│   ├── arc-scene/          # M1: scene graph, entities, transforms
│   ├── arc-commands/       # M1/M4: Command enum, parser, undo/redo
│   ├── arc-commands-derive/# M4: proc-macro crate (must be its own crate)
│   ├── arc-render/         # M2: wgpu renderer, camera, lighting, shaders
│   ├── arc-input/          # M3: raw events -> semantic actions
│   ├── arc-gesture/        # M3: gesture recognizer state machines
│   ├── arc-ui/             # M4: egui overlay and command box
│   ├── arc-voice/          # M5: audio capture, speech-to-text worker
│   ├── arc-vision/         # M6: camera frames, hand tracking
│   ├── arc-scan/           # M7: photogrammetry pipeline
│   ├── arc-app/            # M2+: desktop application binary
│   └── arc-android/        # M8: Android entry point and JNI glue
│
├── assets/
│   ├── models/             # test .obj and .glb files
│   ├── shaders/            # .wgsl files
│   └── ml/                 # hand-tracking and speech model files (git-ignored if large)
│
├── scans/                  # M7: photo sets, calibration images, outputs (git-ignored)
├── docs/
│   ├── ROADMAP.md          # milestones M0-M9 and "done when" criteria
│   ├── journal/            # one note per concept: small example + one mistake made
│   └── decisions/          # short notes on design choices and crate selections
├── tests/                  # workspace-level integration tests
└── tools/                  # helper scripts (build, package, adb install)
```

Layering rule: dependencies point downward only. `arc-math` and `arc-model` depend on nothing from Arcforge; `arc-app` depends on everything; no crate depends on `arc-app`.

```
arc-app / arc-android
   ├── arc-ui, arc-render, arc-voice, arc-vision, arc-gesture
   │        └── arc-input, arc-commands, arc-scene
   │                 └── arc-model, arc-math
   └── arc-scan  ->  arc-model, arc-math
```

## 5. Conventions

- Run `cargo fmt` and `cargo clippy` before every commit; clippy must be clean.
- No `unwrap()` or `expect()` in library code paths that can fail. Use `Result` with custom error enums (`impl std::error::Error`, `From` for conversions).
- Every `unsafe` block gets a `// SAFETY:` comment stating the contract it relies on. Wrap unsafe code in a safe API.
- Unit tests live next to the code (`#[cfg(test)]`); integration tests in `tests/`.
- Doc comments (`///`) on public items; `cargo doc` should build without warnings.
- Commit at the end of every working session; use Git from day one.
- Pin dependency versions through `Cargo.lock`. Check maintenance status before adding a crate.
- Keep a journal entry in `docs/journal/` for each new concept: a small example plus one mistake made.

## 6. Roadmap summary

| Milestone | Weeks | Feature | Key Rust concepts |
|---|---|---|---|
| M0 | 1-4 | Rust foundations, OBJ model inspector (CLI) | Ownership, borrowing, structs, enums, `Result`, collections, modules, tests |
| M1 | 5-9 | Headless engine core: math, scene graph, commands | Traits, generics, closures, iterators, lifetimes, workspaces |
| M2 | 10-15 | Window and GPU rendering, lit rotating glTF model | Smart pointers, `RefCell`, `repr(C)`, first `unsafe`, event loops |
| M3 | 16-18 | Mouse, touch and gesture recognizer | Pattern matching, typestate, channels |
| M4 | 19-22 | Typed commands, undo/redo, egui UI, scene save/load | Declarative and proc macros, trait objects, `serde` |
| M5 | 23-26 | Voice control | Threads, `Arc`/`Mutex`, atomics, async executor |
| M6 | 27-31 | Camera feed and hand tracking | FFI, deep `unsafe`, SIMD, profiling |
| M7 | 32-37 | Photogrammetry scanning (Route C) | `rayon`, const generics, GATs, HRTBs, optimization |
| M8 | 38-43 | Android | `cfg`, cross-compiling, JNI, `no_std`/`alloc` basics |
| M9 | 44 | Release | Profiling, CI, docs, packaging |

### M0 detail (current)

- **Week 1:** Cargo basics, variables, mutability, shadowing, scalar types, control flow. Do three predict-then-run snippets (immutability error, shadowing in scopes, `u8` overflow) and a temperature converter (Celsius to Fahrenheit with a freezing/mild/warm message).
- **Weeks 2-4:** functions, slices, `String` vs `&str`, `std::io`/`std::fs`, ownership, borrowing, structs, enums, `Option`, `match`, `Result` and `?`, custom errors, `Vec`/`HashMap`/`HashSet`, modules, unit tests.
- **Reading:** The Rust Book chapters 1-10, with Rustlings alongside.
- **Done when:** the inspector loads real OBJ models and reports vertex/face counts and the bounding box, has no `unwrap` in error paths, passes clippy cleanly and has unit tests.

### M7 scanning pipeline (reference)

1. Camera calibration (checkerboard, OpenCV)
2. Feature detection (SIFT/AKAZE)
3. Matching and RANSAC outlier rejection
4. Two-view initialization and triangulation
5. Incremental registration with PnP
6. Bundle adjustment (own sparse Levenberg-Marquardt solver)
7. Dense reconstruction, 8. Meshing, 9. Texturing and glTF export (external tools: COLMAP/OpenMVS)

Requirements: Android phone for capture; textured matte object; 30-100 sharp photos with 60-80% overlap; Visual Studio C++ Build Tools, LLVM/Clang, CMake, OpenCV (vcpkg or prebuilt) on Windows.

## 7. Current progress

Update this section at the end of each session.

- **Milestone:** M0
- **Week:** 1 (starting)
- **Done:** Rust installed via rustup. Course plan and roadmap agreed.
- **In progress:** Week 1 predict-then-run snippets A, B, C and the temperature converter. Learner has not yet submitted guesses.
- **Stuck on / hard concepts:** none yet.
- **Next step:** create the workspace (`cargo new arcforge`, then convert to a workspace with `arc-model` and `arc-cli` crates), then work through Week 1 exercises.

## 8. Starting a new session

When resuming in a new chat or in Claude Code:

1. Read this file.
2. Check the "Current progress" section and the latest entries in `docs/journal/`.
3. Ask the learner for anything missing: what was finished, what they are stuck on, and the code to review.
4. Continue the teaching loop from section 2.
