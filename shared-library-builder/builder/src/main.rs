use std::error::Error;

use shared_library_builder::build_standalone;

use libglutin_library::latest_libglutin;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libglutin())))
}