//! Öffentliche Script-Variablen für die Plugin-API.
//!
//! Public script variables exposed to the plugin API.

use crate::var::VariableType;

/// Referenz auf eine öffentliche Script-Variable mit festem Namen.
///
/// Reference to a public script variable with a fixed name.
pub struct PublicVar<T> {
    name: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PublicVar<T>
where
    T: PublicVarType + VariableType,
{
    /// Erstellt eine öffentliche Variable mit dem angegebenen Namen.
    ///
    /// Creates a public variable with the given name.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Gibt den Typnamen für die Plugin-API zurück.
    ///
    /// Returns the type name for the plugin API.
    pub fn type_name(&self) -> &'static str {
        T::type_name()
    }

    /// Liest den aktuellen Wert der Variable.
    ///
    /// Reads the current value of the variable.
    pub fn get(&self) -> T::Output {
        T::get_var(self.name)
    }

    /// Schreibt einen neuen Wert in die Variable.
    ///
    /// Writes a new value to the variable.
    pub fn set(&self, value: T) {
        crate::var::set_var(self.name, value)
    }
}

/// Typinformation für öffentliche Variablen in der Plugin-API.
///
/// Type information for public variables in the plugin API.
pub trait PublicVarType {
    /// Gibt den Typnamen als String zurück.
    ///
    /// Returns the type name as a string.
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

/// Deklariert öffentliche Script-Variablen und exportiert sie für die Plugin-API.
///
/// Declares public script variables and exports them for the plugin API.
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
