#[doc(hidden)]
pub struct GlobalVar<T> {
    _name: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> GlobalVar<T> {}

struct _GlobalVarDef {
    name: &'static str,
    kind: &'static str,
}

#[doc(hidden)]
#[macro_export]
macro_rules! globals_vars {
    ($($name:ident: $ty:ty),*) => {
        #[no_mangle]
        pub fn global_vars() -> i32 {


            $(
            )*
        }
    };
}
