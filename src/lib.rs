//! A collection of exponentially-smoothed camera controllers for the Bevy Engine.
//!
//! All controllers are based on a `LookTransform` component, which is just an `eye` point that looks at a `target` point. By
//! modifying this component, the scene graph `Transform` will automatically be synchronized.
//!
//! A `LookTransform` can be smoothed by adding a `Smoother` component, and the smoothing will happen automatically.
//!
//! ```rust
//! // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
//! app.add_plugin(LookTransformPlugin);
//!
//! ...
//!
//! commands
//!     .spawn_bundle(LookTransformBundle {
//!         transform: LookTransform { eye, target },
//!         smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
//!     })
//!
//! ...
//!
//! fn move_camera_system(mut cameras: Query<&mut LookTransform>) {
//!     // Later, another system will update the `Transform` and apply smoothing automatically.
//!     for c in cameras.iter_mut() { c.target += Vec3::new(1.0, 1.0, 1.0); }
//! }
//! ```
//!
//! # Look Angles
//!
//! When implementing a camera controller, it's often useful to work directly with the angles (pitch and yaw) of your look
//! direction. You can do this with the `LookAngles` type:
//!
//! ```rust
//! let mut angles = LookAngles::from_vector(transform.look_direction());
//! angles.add_pitch(0.1);
//! angles.add_yaw(0.1);
//! transform.offset_target_in_direction(angles.unit_vector());
//! ```
//!
//! This is how the built-in controllers implement rotation controls.
//!
//! # Built-In Controllers
//!
//! These plugins depend on the `LookTransformPlugin`:
//!
//! - `UnrealCameraPlugin + UnrealCameraBundle`
//!   - Left mouse drag: Locomotion
//!   - Right mouse drag: Rotate camera
//!   - Left and Right mouse drag: Pan camera
//! - `OrbitCameraPlugin + OrbitCameraBundle`
//!   - CTRL + mouse drag: Rotate camera
//!   - Right mouse drag: Pan camera
//!   - Mouse wheel: Zoom

pub mod controllers;

mod look_transform;
mod polar_direction;

pub use look_transform::*;
pub use polar_direction::*;
