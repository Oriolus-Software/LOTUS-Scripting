use serde::{Deserialize, Serialize};

#[doc(hidden)]
#[derive(Clone)]
pub struct PisGroup {
    pub name: String,                       // NAME
    pub stations: Vec<PisStation>,          // STNS
    pub special_chars: Vec<PisSpecialChar>, // SPEC
    pub routes: Vec<PisRoute>,              // ROUT
    pub server_name: Option<String>,        // SERV
}

impl PisGroup {
    /// Holt den Namen der aktiven PIS-Gruppe.
    #[cfg(feature = "ffi")]
    pub fn get_name() -> String {
        let name =
            lotus_script_sys::FfiObject::from_packed(unsafe { lotus_script_sys::pis::get_name() });
        name.deserialize()
    }

    /// Holt die Station mit der gegebenen Code.
    #[cfg(feature = "ffi")]
    pub fn get_station(code: u32) -> Option<PisStation> {
        let station = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_station(code)
        });
        station.deserialize()
    }

    /// Holt die gesamte Liste sämtlicher Sonderzeichen
    #[cfg(feature = "ffi")]
    pub fn get_special_chars() -> Vec<PisSpecialChar> {
        let special_chars = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_special_chars()
        });
        special_chars.deserialize()
    }

    /// Holt die Route mit der gegebenen Linie und Code.
    #[cfg(feature = "ffi")]
    pub fn get_route(line_code: (u32, u32)) -> Option<PisRoute> {
        let route = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_route(line_code.0, line_code.1)
        });
        route.deserialize()
    }

    /// Liefert eine Liste sämtlicher Route-Codes, die es für die gegebenen Linie gibt.
    /// Die Liste ist bereits sortiert und frei von Duplikaten.
    #[cfg(feature = "ffi")]
    pub fn get_special_char_with_line(line: u32, special_char_code: u32) -> String {
        let route_codes = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_special_char_with_line(line, special_char_code)
        });
        route_codes.deserialize()
    }

    /// Holt den Namen des Leitstellen-Servers.
    #[cfg(feature = "ffi")]
    pub fn get_server_name() -> Option<String> {
        let server_name = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_server_name()
        });
        server_name.deserialize()
    }
}

/// Datensatz für eine Station im PIS.
#[derive(Clone, Serialize, Deserialize)]
pub struct PisStation {
    /// Mit der ID wird die Station mit der Map verknüpft (auch dort gibt es eine ID für jede Station).
    /// Dies ist u.A. für die Fahrgäste und die öffentlichen KI-Fahrzeuge notwendig.
    pub id: String,
    /// Der Code ist die Zahl, mit der die Station innerhalb des PIS identifiziert wird.
    /// Dieser Code wird für die Anzeige auf dem PIS-Display verwendet.
    pub code: u32,
    /// Die Strings sind die Texte, die auf den Innenanzeigen angezeigt werden. Zwei Strings gibt es
    /// z.B. für Wechselanzeigen
    pub strings_station: [Option<String>; 2],
    /// Diese Strings sind für die Front-Außenanzeige.
    pub strings_front: [Option<String>; 2],
    /// Diese Strings sind für die Seiten-Außenanzeige, wenn diese zweizeilig ist.
    pub strings_side: [Option<String>; 2],
    /// Dieser String ist für die Seiten-Außenanzeige, wenn diese einzeilig ist.
    pub string_side_oneline: Option<String>,
}

/// Datensatz für ein Sonderzeichen im PIS.
#[derive(Clone, Serialize, Deserialize)]
pub struct PisSpecialChar {
    /// Der Code ist die Zahl, mit der das Sonderzeichen innerhalb des PIS identifiziert wird.
    pub code: u32,
    /// Der Sonderzeichen-String, wobei dieser auch über bestimmte Codes verfügen kann, mit denen
    /// die originale Liniennummer eingefügt werden kann.
    ///
    /// So bedeutet z.B. "M(R2-R1)", dass das auf dem Linienfeld ein M, gefolgt von
    /// den Ziffern 2 bis 1 von RECHTS aus gezählt, angezeigt wird. Wenn die Liniennummer
    /// z.B. 123 ist, dann wird hier M23 angezeigt.
    /// Soll nur M2 angezeigt werden, müsste man "M(R2-R2)" als chars eintragen.
    pub chars: String, // STRN
}

/// Datensatz für eine Route im PIS. Jeder Datensatz wird über eine
/// Liniennummer und einen Code innerhalb der Linie identifiziert.
#[derive(Clone, Serialize, Deserialize)]
pub struct PisRoute {
    /// Liniennummer und Code innerhalb der Linie zur Zuordnung
    pub line_code: (u32, u32),
    /// Liste der Codes der Haltestellen, die auf der Route nacheinander
    /// angefahren werden, inklusive der Abfahrts- und der Endhaltestelle.
    pub stop_codes: Vec<u32>,
    /// Der Codes des Sonderzeichens, welcher automatisch ausgewählt werden soll,
    /// wenn diese Route eingestellt wird. Ob dieser Code überschrieben werden kann usw.
    /// ist abhängig vom Bordrechner.
    pub special_char_code: Option<u32>,
    /// Zusätzliches Textfeld, welches aber aktuell keine bestimmte Bedeutung hat.
    pub text: Option<String>,
    /// Im einfachsten Fall ist das Ziel (der Zielcode) einer Route einfach die letzte
    /// Haltestelle. Es kann aber sein, dass das angezeigte Ziel ab einer bestimmten
    /// Haltestelle wechseln soll, oder auch die Zielhaltestelle anders angezeigt werden soll.
    /// Für diese Fälle kann man Termini definieren.
    pub termini: Vec<PisRouteTerminus>,
    /// Linie/Code für die automatisch zu wählende folgende Route
    pub following_line_code: Option<(u32, u32)>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PisRouteTerminus {
    /// Index of the stop of the route, from which this terminus applies    
    pub stop_index: usize,
    /// New terminus code
    pub code: Option<u32>,
    /// New line
    pub line: Option<u32>,
    /// New special char code
    pub special_char_code: Option<u32>,
}
