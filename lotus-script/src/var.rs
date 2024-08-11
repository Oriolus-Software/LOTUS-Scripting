use lotus_shared::content::ContentId;

use crate::{
    ffi::{self, FromFfi},
    FfiObject,
};

pub trait VariableType {
    fn get(name: &str) -> Self;
    fn set(&self, name: &str);
}

pub enum Persistence {
    Session,
    Permanent,
}

macro_rules! impl_variable_type {
    ($type:ty, $get:ident, $set:ident) => {
        impl VariableType for $type {
            fn get(name: &str) -> Self {
                let name = FfiObject::new(&name);
                unsafe { ffi::var::$get(name.packed()) as _ }
            }

            fn set(&self, name: &str) {
                let name = FfiObject::new(&name);
                unsafe { ffi::var::$set(name.packed(), *self as _) }
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
    fn get(name: &str) -> Self {
        let name = FfiObject::new(&name);
        unsafe { ffi::var::get_bool(name.packed()) }
    }

    fn set(&self, name: &str) {
        let name = FfiObject::new(&name);
        unsafe { ffi::var::set_bool(name.packed(), *self) }
    }
}

impl VariableType for String {
    fn get(name: &str) -> Self {
        let name = FfiObject::new(&name);
        let ptr = unsafe { ffi::var::get_string(name.packed()) };
        String::from_ffi(ptr)
    }

    fn set(&self, name: &str) {
        let name = FfiObject::new(&name);
        let value = FfiObject::new(self);
        unsafe { ffi::var::set_string(name.packed(), value.packed()) }
    }
}

impl VariableType for ContentId {
    fn get(name: &str) -> Self {
        let name = FfiObject::new(&name);
        let ptr = unsafe { ffi::var::get_content_id(name.packed()) };
        ContentId::from_ffi(ptr)
    }

    fn set(&self, name: &str) {
        let name = FfiObject::new(&name);
        let value = FfiObject::new(self);
        unsafe { ffi::var::set_content_id(name.packed(), value.packed()) }
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
    pub fn get(&self) -> T {
        T::get(&self.name)
    }

    pub fn set(&self, value: &T) {
        value.set(&self.name);
    }
}