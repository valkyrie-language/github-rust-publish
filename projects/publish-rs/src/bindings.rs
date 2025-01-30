#[derive(Clone)]
pub enum GithubError {
    Custom(_rt::String),
}
impl ::core::fmt::Debug for GithubError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            GithubError::Custom(e) => {
                f.debug_tuple("GithubError::Custom").field(e).finish()
            }
        }
    }
}
impl ::core::fmt::Display for GithubError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for GithubError {}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_run_with_config_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::run_with_config(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(_) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            match e {
                GithubError::Custom(e) => {
                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                    let vec3 = (e.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(12).cast::<usize>() = len3;
                    *ptr2.add(8).cast::<*mut u8>() = ptr3.cast_mut();
                }
            }
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_run_with_config<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {}
        _ => {
            let l1 = i32::from(*arg0.add(4).cast::<u8>());
            match l1 {
                _ => {
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
            }
        }
    }
}
pub trait Guest {
    fn run_with_config(config: _rt::String) -> Result<(), GithubError>;
}
#[doc(hidden)]
macro_rules! __export_world_action_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "run-with-config"] unsafe extern "C" fn
        export_run_with_config(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
        $($path_to_types)*:: _export_run_with_config_cabi::<$ty > (arg0, arg1) }
        #[export_name = "cabi_post_run-with-config"] unsafe extern "C" fn
        _post_return_run_with_config(arg0 : * mut u8,) { $($path_to_types)*::
        __post_return_run_with_config::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_action_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 16]);
mod _rt {
    pub use alloc_crate::string::String;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_action_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_action_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_action_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:github:rust-release:action:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 230] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07j\x01A\x02\x01A\x05\x01\
q\x01\x06custom\x01s\0\x03\0\x0cgithub-error\x03\0\0\x01j\0\x01\x01\x01@\x01\x06\
configs\0\x02\x04\0\x0frun-with-config\x01\x03\x04\0\x1agithub:rust-release/acti\
on\x04\0\x0b\x0c\x01\0\x06action\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
