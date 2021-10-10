use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libglutin(version: impl Into<String>) -> RustLibrary {
    RustLibrary::new(
        "Glutin",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libglutin").tag(version)),
    )
    .package("libglutin")
}
