use crate::var::VariableType;

pub struct PublicVar<T> {
    name: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PublicVar<T>
where
    T: PublicVarType + VariableType,
{
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn type_name(&self) -> &'static str {
        T::type_name()
    }

    pub fn get(&self) -> T::Output {
        T::get(self.name)
    }

    pub fn set(&self, value: T) {
        value.set(self.name)
    }
}

pub trait PublicVarType {
    fn type_name() -> &'static str;
}

macro_rules! impl_public_var_type {
    ($type:ty, $name:ident) => {
        impl PublicVarType for $type {
            fn type_name() -> &'static str {
                stringify!($name)
            }
        }
    };
}

impl_public_var_type!(i32, i32);
impl_public_var_type!(i64, i64);

impl_public_var_type!(u32, u32);
impl_public_var_type!(u64, u64);

impl_public_var_type!(f32, f32);
impl_public_var_type!(f64, f64);

impl_public_var_type!(bool, bool);
impl_public_var_type!(String, string);

#[macro_export]
macro_rules! public_vars {
    ($($name:ident: $type:ty),* $(,)?) => {
        pub mod pub_var {
            $(
                #[allow(non_upper_case_globals)]
                pub const $name: $crate::public_vars::PublicVar<$type> = $crate::public_vars::PublicVar::new(stringify!($name));
            )*
        }
        #[no_mangle]
        pub fn public_vars() -> u64 {
            use $crate::public_vars::PublicVarType;
            let vars = vec![
                $(
                    (stringify!($name), pub_var::$name.type_name()),
                )*
            ];

            $crate::FfiObject::new(&vars).packed_forget()
        }
    };
}
