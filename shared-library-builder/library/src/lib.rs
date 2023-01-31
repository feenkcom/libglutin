use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libglutin(version: Option<impl Into<String>>) -> RustLibrary {
    RustLibrary::new(
        "Glutin",
        LibraryLocation::Git(
            GitLocation::github("feenkcom", "libglutin").tag_or_latest(version),
        ),
    )
        .package("libglutin")
}

pub fn latest_libglutin() -> RustLibrary {
    let version: Option<String> = None;
    libglutin(version)
}
