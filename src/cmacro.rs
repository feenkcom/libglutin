macro_rules! to_rust_reference {
    ($name:ident) => { unsafe { &mut *$name } };
}

macro_rules! to_c_string {
    ($name:ident) => {
        unsafe {
            assert!(!$name.is_null());
            CStr::from_ptr($name)
        };
    };
}

macro_rules! to_rust_string {
    ($name:ident) => { to_c_string!($name).to_str().unwrap() };
}

macro_rules! for_delete {
    ($name:ident) => (unsafe { transmute($name) });
}

macro_rules! for_create {
    ($expression:expr) => (unsafe { transmute(Box::new($expression)) });
}