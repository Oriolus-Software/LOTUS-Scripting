use lotus_script_sys::{FfiObject, FromFfi};
use lotus_shared::content::ContentId;

pub trait VariableType {
    type Output;

    fn get_var(name: &str) -> Self::Output;
    fn set_var(var: &Self, name: &str);
}

pub enum Persistence {
    Session,
    Permanent,
}

macro_rules! impl_variable_type {
    ($type:ty, $get:ident, $set:ident) => {
        impl VariableType for $type {
            type Output = $type;

            fn get_var(name: &str) -> Self::Output {
                let name = FfiObject::new(&name);
                unsafe { lotus_script_sys::var::$get(name.packed()) as _ }
            }

            fn set_var(var: &Self, name: &str) {
                let name = FfiObject::new(&name);
                unsafe { lotus_script_sys::var::$set(name.packed(), *var as _) }
            }
        }
    };
}

impl_variable_type!(i8, get_i64, set_i64);
impl_variable_type!(i16, get_i64, set_i64);
impl_variable_type!(i32, get_i64, set_i64);
impl_variable_type!(i64, get_i64, set_i64);

impl_variable_type!(u8, get_i64, set_i64);
impl_variable_type!(u16, get_i64, set_i64);
impl_variable_type!(u32, get_i64, set_i64);
impl_variable_type!(u64, get_i64, set_i64);

impl_variable_type!(f32, get_f64, set_f64);
impl_variable_type!(f64, get_f64, set_f64);

impl VariableType for bool {
    type Output = bool;

    fn get_var(name: &str) -> Self::Output {
        let name = FfiObject::new(&name);
        unsafe { lotus_script_sys::var::get_bool(name.packed()) != 0 }
    }

    fn set_var(var: &Self, name: &str) {
        let name = FfiObject::new(&name);
        unsafe {
            lotus_script_sys::var::set_bool(
                name.packed(),
                match var {
                    true => 1,
                    false => 0,
                },
            )
        }
    }
}

impl VariableType for String {
    type Output = String;

    fn get_var(name: &str) -> Self::Output {
        let name = FfiObject::new(&name);
        let ptr = unsafe { lotus_script_sys::var::get_string(name.packed()) };
        String::from_ffi(ptr)
    }

    fn set_var(var: &Self, name: &str) {
        let name = FfiObject::new(&name);
        let value = FfiObject::new(&var);
        unsafe { lotus_script_sys::var::set_string(name.packed(), value.packed()) }
    }
}

impl VariableType for &str {
    type Output = String;

    fn get_var(name: &str) -> Self::Output {
        let name = FfiObject::new(&name);
        let ptr = unsafe { lotus_script_sys::var::get_string(name.packed()) };
        String::from_ffi(ptr)
    }

    fn set_var(var: &Self, name: &str) {
        let name = FfiObject::new(&name);
        let value = FfiObject::new(&var.to_string());
        unsafe { lotus_script_sys::var::set_string(name.packed(), value.packed()) }
    }
}

impl VariableType for ContentId {
    type Output = ContentId;

    fn get_var(name: &str) -> Self {
        let name = FfiObject::new(&name);
        let ptr = unsafe { lotus_script_sys::var::get_content_id(name.packed()) };
        FfiObject::from_packed(ptr).deserialize()
    }

    fn set_var(var: &Self, name: &str) {
        let name = FfiObject::new(&name);
        let value = FfiObject::new(var);
        unsafe { lotus_script_sys::var::set_content_id(name.packed(), value.packed()) }
    }
}

pub struct Variable<T> {
    name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Variable<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: VariableType> Variable<T> {
    pub fn get(&self) -> T::Output {
        T::get_var(&self.name)
    }

    pub fn set(&self, value: &T) {
        T::set_var(value, &self.name);
    }
}

pub fn get_var<T: VariableType>(name: &str) -> T::Output {
    T::get_var(name)
}

pub fn set_var<T: VariableType>(var: &T, name: &str) {
    T::set_var(var, name);
}
