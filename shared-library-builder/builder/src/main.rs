use shared_library_builder::{
    Library, LibraryCompilationContext, LibraryLocation, LibraryTarget, PathLocation, RustLibrary,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let library = RustLibrary::new(
        "Glutin",
        LibraryLocation::Path(PathLocation::new(std::env::current_dir().unwrap())),
    )
    .package("libglutin");

    let context = LibraryCompilationContext::new(
        "target",
        "target",
        LibraryTarget::for_current_platform(),
        false,
    );
    let compiled_library = library.compile(&context)?;
    println!("Compiled {}", compiled_library.display());
    Ok(())
}
