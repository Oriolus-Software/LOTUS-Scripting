use serde::{Deserialize, Serialize};

use crate::content::ContentId;

#[doc(hidden)]
#[derive(Clone, Debug)]
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
    pub fn get_special_char_with_line(line: u32, special_char_code: u32) -> String {
        let route_codes = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_special_char_with_line(line, special_char_code)
        });
        route_codes.deserialize()
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
    pub fn get_route_codes_by_line(line: u32) -> Vec<u32> {
        let route_codes = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_route_codes_by_line(line)
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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PisStation {
    /// Mit der ID wird die Station mit der Map verknüpft (auch dort gibt es eine ID für jede Station).
    /// Dies ist u.A. für die Fahrgäste und die öffentlichen KI-Fahrzeuge notwendig.
    pub id: String,
    /// Der Code ist die Zahl, mit der die Station innerhalb des PIS identifiziert wird.
    /// Dieser Code wird für die Anzeige auf dem PIS-Display verwendet.
    pub code: u32,
    /// Die Strings sind die Texte, die auf den Innenanzeigen angezeigt werden. Zwei Strings gibt es
    /// z.B. für Wechselanzeigen
    pub interieur_display: [String; 2],
    /// Die folgenden Strings sind für die Außenanzeigen.
    pub terminus_front_option: Option<[String; 2]>,
    pub terminus_side_option: Option<[String; 2]>,
    pub terminus_oneline: String,
}

pub enum PisStationTerminusOneLineTo {
    FirstLine,
    SecondLine,
}

impl PisStation {
    /// Liefert die (zweizeilige) Front-Außenanzeige für das Ziel.
    /// Verfügt die Ziel-Station über keine zweizeilige Front-Außenanzeige, so wird die einzeilige
    /// Außenanzeige um eine Leerzeile erweitert, wobei one_to_two_line die Reihenfolge dieser
    /// Erweiterung bestimmt:
    /// - FirstLine: Die einzeilige Zeile wird in die erste Zeile und die Leerzeile in die zweite Zeile eingefügt
    /// - SecondLine: Die einzeilige Zeile wird in die zweite Zeile und die Leerzeile in die erste Zeile eingefügt
    pub fn terminus_front(&self, one_to_two_line: PisStationTerminusOneLineTo) -> [String; 2] {
        if let Some(front) = &self.terminus_front_option {
            front.clone()
        } else {
            match one_to_two_line {
                PisStationTerminusOneLineTo::FirstLine => {
                    [self.terminus_oneline.clone(), String::new()]
                }
                PisStationTerminusOneLineTo::SecondLine => {
                    [String::new(), self.terminus_oneline.clone()]
                }
            }
        }
    }

    pub fn terminus_side(&self, one_to_two_line: PisStationTerminusOneLineTo) -> [String; 2] {
        if let Some(side) = &self.terminus_side_option {
            side.clone()
        } else {
            self.terminus_front(one_to_two_line)
        }
    }
}

/// Datensatz für ein Sonderzeichen im PIS.
#[derive(Clone, Serialize, Deserialize, Debug)]
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
#[derive(Clone, Serialize, Deserialize, Debug)]
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
    /// Sind für das Balisensystem/Anforderungen/Weichen individuelle Anmelde-Codes nötig?
    pub routing_code: Option<u32>,
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

impl PisRoute {
    pub fn new(
        line_code: (u32, u32),
        stop_codes: Vec<u32>,
        special_char_code: Option<u32>,
        routing_code: Option<u32>,
        text: Option<String>,
        termini: Vec<PisRouteTerminus>,
        following_line_code: Option<(u32, u32)>,
    ) -> Self {
        Self {
            line_code,
            stop_codes,
            special_char_code,
            routing_code,
            text,
            termini,
            following_line_code,
        }
    }
    pub fn get_current_direction(&self, stop_index: usize) -> Option<PisRouteTerminus> {
        self.termini
            .iter()
            .find(|terminus| stop_index >= terminus.stop_index)
            .cloned()
            .or_else(|| {
                self.stop_codes.last().map(|code| PisRouteTerminus {
                    stop_index: 0,
                    code: Some(*code),
                    line: Some(self.line_code.0),
                    special_char_code: self.special_char_code,
                    routing_code: self.routing_code,
                })
            })
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct PisRouteTerminus {
    /// Index of the stop of the route, from which this terminus applies    
    pub stop_index: usize,
    /// New terminus code
    pub code: Option<u32>,
    /// New line
    pub line: Option<u32>,
    /// New special char code
    pub special_char_code: Option<u32>,
    /// New routing code
    pub routing_code: Option<u32>,
}

impl PisRouteTerminus {
    fn routing_code_hash(&self) -> u32 {
        0
    }

    pub fn routing_code(&self) -> u32 {
        self.routing_code.unwrap_or(self.routing_code_hash())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PisSpGroup {
    pub name: String,
    pub basic_pis_group: ContentId,
    pub class: String,
    pub add_lines: String,
    pub add_lines_stations: Vec<PisSpAddLines>,
    pub routes: Vec<PisSpRoute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PisSpAddLines {
    pub code: i32,
    pub lines: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PisSpRoute {
    pub code: i32,
    pub lines: String,
    pub stop_lines: Vec<String>,
}

impl PisSpGroup {
    /// Liefert die ContentId der PISS-Gruppe, die zur aktiven PISG passt und die gegebene Klasse hat.
    #[cfg(feature = "ffi")]
    pub fn get_content_id(class: &str) -> Option<ContentId> {
        let class = lotus_script_sys::FfiObject::new(&class);
        let content_id = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_content_id(class.packed())
        });
        content_id.deserialize()
    }

    /// Liefert die zusätzlichen Linien aus der gegebenen PISS-Gruppe.
    #[cfg(feature = "ffi")]
    pub fn get_group_strings(content_id: ContentId) -> String {
        let content_id = lotus_script_sys::FfiObject::new(&content_id);
        let lines = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_group_strings(content_id.packed())
        });
        lines.deserialize()
    }

    /// Liefert die zusätzlichen Linien für eine Station aus der gegebenen PISS-Gruppe.
    #[cfg(feature = "ffi")]
    pub fn get_station_strings(content_id: ContentId, station_code: u32) -> Option<String> {
        let content_id = lotus_script_sys::FfiObject::new(&content_id);
        let lines = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_station_strings(content_id.packed(), station_code)
        });
        lines.deserialize()
    }

    /// Liefert die Route mit der gegebenen Linie und Code aus der gegebenen PISS-Gruppe.
    #[cfg(feature = "ffi")]
    pub fn get_route(content_id: ContentId, line: u32, code: u32) -> Option<PisSpRoute> {
        let content_id = lotus_script_sys::FfiObject::new(&content_id);
        let route = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_route_data(content_id.packed(), line, code)
        });
        route.deserialize()
    }
}
