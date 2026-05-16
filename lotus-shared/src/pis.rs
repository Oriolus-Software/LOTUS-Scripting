use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PisGroup {
    pub name: String,                       // NAME
    pub stations: Vec<PisStation>,          // STNS
    pub special_chars: Vec<PisSpecialChar>, // SPEC
    pub routes: Vec<PisRoute>,              // ROUT
    pub server_name: Option<String>,        // SERV
}

impl PisGroup {
    #[cfg(feature = "ffi")]
    pub fn get_name() -> String {
        let name =
            lotus_script_sys::FfiObject::from_packed(unsafe { lotus_script_sys::pis::get_name() });
        name.deserialize()
    }

    #[cfg(feature = "ffi")]
    pub fn get_station(code: i32) -> Option<PisStation> {
        let station = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_station(code)
        });
        station.deserialize()
    }

    #[cfg(feature = "ffi")]
    pub fn get_special_chars() -> Vec<PisSpecialChar> {
        let special_chars = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_special_chars()
        });
        special_chars.deserialize()
    }

    #[cfg(feature = "ffi")]
    pub fn get_route(line: i32, code: i32) -> Option<PisRoute> {
        let route = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_route(line, code)
        });
        route.deserialize()
    }

    /// bereits sortiert und ohne Duplikate
    #[cfg(feature = "ffi")]
    pub fn get_route_codes_by_line(line: i32) -> Vec<i32> {
        let route_codes = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_route_codes_by_line(line)
        });
        route_codes.deserialize()
    }

    #[cfg(feature = "ffi")]
    pub fn get_server_name() -> Option<String> {
        let server_name = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_server_name()
        });
        server_name.deserialize()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PisStation {
    pub id: String, // ID__
    pub code: i32,  // CODE
    pub strings_station: [Option<String>; 2],
    pub strings_front: [Option<String>; 2],
    pub strings_side: [Option<String>; 2],
    pub string_side_oneline: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PisSpecialChar {
    pub code: i32,     // CODE
    pub chars: String, // STRN
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PisRoute {
    pub line: i32,                      // LINE
    pub code: i32,                      // CODE
    pub stop_codes: Vec<i32>,           // STPS
    pub special_char_code: Option<i32>, // SPCR
    pub text: Option<String>,           // TEXT
    pub termini: Vec<PisRouteTerminus>, // TMIN
    pub following: Option<i32>,         // FOLW
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PisRouteTerminus {
    pub code: i32, // TERM
    /// Index of the stop of the route, from which this terminus applies    
    pub stop_index: usize, // STOP
}
