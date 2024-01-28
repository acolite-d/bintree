
// TODO: Figure out how to spawn a compile error when two or more features
// are defined. Use cfg-if package from here on out.
// #[cfg(all(feature = "unique_pointer", feature = "g4g_pointer"))]
// compile_error!("Please specify only one pointer feature for building the binary tree lib!");

// #[cfg(all(feature = "unique_pointer", feature = "raw_pointer"))]
// compile_error!("Please specify only one pointer feature for building the binary tree lib!");

// #[cfg(all(feature = "g4g_pointer", feature = "raw_pointer"))]
// compile_error!("Please specify only one pointer feature for building the binary tree lib!");

// #[cfg(all(feature = "unique_pointer", feature = "g4g_pointer", feature = "raw_pointer"))]
// compile_error!("Please specify only one pointer feature for building the binary tree lib!");


// #[cfg(feature = "unique_pointer")]
// pub mod box_bintree;

// #[cfg(feature = "g4g_pointer")]
// pub mod rc_refcell_bintree;

// #[cfg(feature = "raw_pointer")]
// pub mod raw_bintree;


// Hey this is pretty useful!
use cfg_if;

cfg_if::cfg_if! {
    if #[cfg(feature = "unique_pointer")] {
        pub mod box_bintree;
    } else if #[cfg(feature = "g4g_pointer")] {
        pub mod rc_refcell_bintree;
    } else if #[cfg(feature = "raw_pointer")] {
        pub mod raw_bintree;
    } else {
        compile_error!("No pointer feature supplied, please specify at least one to compile!");
    }
}