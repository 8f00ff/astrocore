//! # Build Script
//! Configures build environment settings for optimal compilation.
//! 
//! This script detects system capabilities and sets appropriate build 
//! environment variables to optimize parallel compilation.

/// Sets up environment variables for parallel compilation.
/// 
/// # Functionality
/// - Detects available CPU cores
/// - Sets NUM_JOBS environment variable for optimal parallel builds
/// - Sets CARGO_BUILD_JOBS to match available parallelism
/// - Ensures the script reruns if modified
fn main() {
  let num_cores = std::thread::available_parallelism()
    .map(|p| p.get())
    .unwrap_or(1);
  println!("cargo:rustc-env=NUM_JOBS={}", num_cores);
  println!("cargo:rustc-env=CARGO_BUILD_JOBS={}", num_cores);
  println!("cargo:rerun-if-changed=build.rs");
}
