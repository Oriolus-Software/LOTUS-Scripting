//! Fahrgastinformationssystem (PIS): Daten und Abfragefunktionen.
//!
//! Passenger information system (PIS) data and query functions.

use serde::{Deserialize, Serialize};

use crate::content::ContentId;

/// Namespace zum Abfragen der aktiven PIS-Gruppe aus dem Simulator.
///
/// Scripts konstruieren diesen Typ nicht manuell; die zugehörigen Funktionen
/// (z. B. [`PisGroup::get_all_stations`]) lesen PIS-Daten zur Laufzeit.
///
/// Namespace for querying the active PIS group from the simulator.
///
/// Scripts do not construct this type manually; call the associated functions
/// (e.g. [`PisGroup::get_all_stations`]) to read PIS data at runtime.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PisGroup {
    /// Name der PIS-Gruppe.
    ///
    /// Name of the PIS group.
    pub name: String,
    /// In den Gruppendaten enthaltene Stationen.
    ///
    /// Stations contained in the group data set.
    pub stations: Vec<PisStation>,
    /// In der Gruppe definierte Sonderzeichen.
    ///
    /// Special characters defined in the group.
    pub special_chars: Vec<PisSpecialChar>,
    /// In der Gruppe definierte Routen.
    ///
    /// Routes defined in the group.
    pub routes: Vec<PisRoute>,
    /// Optionaler Name des Leitstellen-Servers.
    ///
    /// Optional dispatch server name.
    pub server_name: Option<String>,
}

impl PisGroup {
    /// Holt den Namen der aktiven PIS-Gruppe.
    ///
    /// Returns the name of the active PIS group.
    #[cfg(feature = "ffi")]
    pub fn get_name() -> String {
        let name =
            lotus_script_sys::FfiObject::from_packed(unsafe { lotus_script_sys::pis::get_name() });
        name.deserialize()
    }

    /// Holt die Station mit dem gegebenen Code.
    ///
    /// Returns the station with the given code.
    #[cfg(feature = "ffi")]
    pub fn get_station(code: u32) -> Option<PisStation> {
        let station = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_station(code)
        });
        station.deserialize()
    }

    /// Holt den aufgelösten Sonderzeichen-String für die gegebene Linie und den Code.
    ///
    /// Returns the resolved special-character string for the given line and code.
    #[cfg(feature = "ffi")]
    pub fn get_special_char_with_line(line: u32, special_char_code: u32) -> String {
        let route_codes = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_special_char_with_line(line, special_char_code)
        });
        route_codes.deserialize()
    }

    /// Holt die Route mit der gegebenen Linie und dem Code.
    ///
    /// Returns the route for the given line and route code.
    #[cfg(feature = "ffi")]
    pub fn get_route(line_code: (u32, u32)) -> Option<PisRoute> {
        let route = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_route(line_code.0, line_code.1)
        });
        route.deserialize()
    }

    /// Liefert eine sortierte, duplikatfreie Liste aller Routencodes für die gegebene Linie.
    ///
    /// Returns sorted unique route codes available for the given line.
    #[cfg(feature = "ffi")]
    pub fn get_route_codes_by_line(line: u32) -> Vec<u32> {
        let route_codes = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_route_codes_by_line(line)
        });
        route_codes.deserialize()
    }

    /// Liefert sämtliche Stationen der aktiven PIS-Gruppe.
    ///
    /// Returns all stations of the active PIS group.
    #[cfg(feature = "ffi")]
    pub fn get_all_stations() -> Vec<PisStation> {
        let stations = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_all_stations()
        });
        stations.deserialize()
    }

    /// Liefert sämtliche Routen der aktiven PIS-Gruppe.
    ///
    /// Returns all routes of the active PIS group.
    #[cfg(feature = "ffi")]
    pub fn get_all_routes() -> Vec<PisRoute> {
        let routes = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_all_routes()
        });
        routes.deserialize()
    }

    /// Liefert sämtliche Sonderzeichen der aktiven PIS-Gruppe, aufgelöst für die gegebene Linie.
    /// Jedes Tupel enthält den Sonderzeichen-Code und den aufgelösten Anzeigestring.
    ///
    /// Returns all special characters of the active PIS group resolved for the given line.
    #[cfg(feature = "ffi")]
    pub fn get_all_special_chars_with_line(line: u32) -> Vec<(u32, String)> {
        let special_chars = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_all_special_chars_with_line(line)
        });
        special_chars.deserialize()
    }

    /// Holt den Namen des Leitstellen-Servers.
    ///
    /// Returns the name of the dispatch server, if connected.
    #[cfg(feature = "ffi")]
    pub fn get_server_name() -> Option<String> {
        let server_name = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_server_name()
        });
        server_name.deserialize()
    }
}

/// Datensatz für eine Station im PIS.
///
/// PIS station record.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PisStation {
    /// Mit der ID wird die Station mit der Map verknüpft (auch dort gibt es eine ID für jede Station).
    /// Dies ist u. a. für die Fahrgäste und die öffentlichen KI-Fahrzeuge notwendig.
    ///
    /// Links the station to the map (each station has an id there as well).
    /// Required e.g. for passengers and public AI vehicles.
    pub id: String,
    /// Der Code ist die Zahl, mit der die Station innerhalb des PIS identifiziert wird.
    /// Dieser Code wird für die Anzeige auf dem PIS-Display verwendet.
    ///
    /// Numeric code identifying the station within the PIS; used on PIS displays.
    pub code: u32,
    /// Die Strings sind die Texte, die auf den Innenanzeigen angezeigt werden. Zwei Strings gibt es
    /// z. B. für Wechselanzeigen.
    ///
    /// Interior display texts; two strings e.g. for alternating displays.
    pub interieur_display: [String; 2],
    /// Optionale zweizeilige Front-Außenanzeige.
    ///
    /// Optional two-line front exterior destination display.
    pub terminus_front_option: Option<[String; 2]>,
    /// Optionale zweizeilige Seiten-Zielanzeige.
    ///
    /// Optional two-line side destination display strings.
    pub terminus_side_option: Option<[String; 2]>,
    /// Einzeiliger Außenzieltext.
    ///
    /// Single-line exterior destination text.
    pub terminus_oneline: String,
}

/// Platzierung eines einzeiligen Ziels beim Erweitern auf zwei Zeilen.
///
/// Placement of a one-line destination when expanding to two lines.
pub enum PisStationTerminusOneLineTo {
    /// Einzeiligen Text in die erste Zeile setzen.
    ///
    /// Put the one-line text on the first line.
    FirstLine,
    /// Einzeiligen Text in die zweite Zeile setzen.
    ///
    /// Put the one-line text on the second line.
    SecondLine,
}

impl PisStation {
    /// Liefert die (zweizeilige) Front-Außenanzeige für das Ziel.
    /// Verfügt die Ziel-Station über keine zweizeilige Front-Außenanzeige, so wird die einzeilige
    /// Außenanzeige um eine Leerzeile erweitert, wobei `one_to_two_line` die Reihenfolge dieser
    /// Erweiterung bestimmt:
    /// - `FirstLine`: Die einzeilige Zeile wird in die erste Zeile und die Leerzeile in die zweite Zeile eingefügt
    /// - `SecondLine`: Die einzeilige Zeile wird in die zweite Zeile und die Leerzeile in die erste Zeile eingefügt
    ///
    /// Returns the (two-line) front exterior destination display.
    /// If no two-line front display is defined, the one-line text is expanded with a blank line;
    /// `one_to_two_line` controls the order:
    /// - `FirstLine`: one-line text on the first line, blank on the second
    /// - `SecondLine`: blank on the first line, one-line text on the second
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

    /// Gibt die zweizeilige Seiten-Zielanzeige dieser Station zurück.
    ///
    /// Returns the two-line side destination display for this station.
    pub fn terminus_side(&self, one_to_two_line: PisStationTerminusOneLineTo) -> [String; 2] {
        if let Some(side) = &self.terminus_side_option {
            side.clone()
        } else {
            self.terminus_front(one_to_two_line)
        }
    }

    /// Gibt zurück, ob der Stationscode ein öffentlicher PIS-Code ist.
    ///
    /// Returns whether the station code is a public PIS code.
    pub fn code_is_public(&self) -> bool {
        self.code < 1_000_000
    }
}

/// Datensatz für ein Sonderzeichen im PIS.
///
/// PIS special-character record.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PisSpecialChar {
    /// Der Code ist die Zahl, mit der das Sonderzeichen innerhalb des PIS identifiziert wird.
    ///
    /// Numeric code identifying the special character within the PIS.
    pub code: u32,
    /// Der Sonderzeichen-String, wobei dieser auch über bestimmte Codes verfügen kann, mit denen
    /// die originale Liniennummer eingefügt werden kann.
    ///
    /// So bedeutet z. B. „M(R2-R1)“, dass auf dem Linienfeld ein M, gefolgt von
    /// den Ziffern 2 bis 1 von rechts gezählt, angezeigt wird. Wenn die Liniennummer
    /// z. B. 123 ist, wird M23 angezeigt.
    /// Soll nur M2 angezeigt werden, müsste man „M(R2-R2)“ als `chars` eintragen.
    ///
    /// Special-character string; may contain codes that insert the original line number.
    ///
    /// For example, `M(R2-R1)` displays M followed by digits 2 down to 1 counted from the right.
    /// For line number 123 this shows M23; for M2 only use `M(R2-R2)` as `chars`.
    pub chars: String, // STRN
}

/// Datensatz für eine Route im PIS. Jeder Datensatz wird über eine
/// Liniennummer und einen Code innerhalb der Linie identifiziert.
///
/// PIS route record identified by line number and in-line route code.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PisRoute {
    /// Liniennummer und Code innerhalb der Linie zur Zuordnung.
    ///
    /// Line number and in-line route code for identification.
    pub line_code: (u32, u32),
    /// Liste der Codes der Haltestellen, die auf der Route nacheinander
    /// angefahren werden, inklusive der Abfahrts- und der Endhaltestelle.
    ///
    /// Stop codes visited in order, including departure and destination stops.
    pub stop_codes: Vec<u32>,
    /// Code des Sonderzeichens, das automatisch ausgewählt werden soll,
    /// wenn diese Route eingestellt wird. Ob dieser Code überschrieben werden kann, ist abhängig vom Bordrechner.
    ///
    /// Special-character code selected automatically when this route is set; overridability depends on the onboard computer.
    pub special_char_code: Option<u32>,
    /// Sind für das Balisensystem/Anforderungen/Weichen individuelle Anmelde-Codes nötig?
    ///
    /// Optional routing/signalling code for balise systems, requests, or turnouts.
    pub routing_code: Option<u32>,
    /// Zusätzliches Textfeld ohne fest definierte Bedeutung.
    ///
    /// Additional text field with no fixed meaning currently.
    pub text: Option<String>,
    /// Im einfachsten Fall ist das Ziel einer Route die letzte Haltestelle.
    /// Ab einer bestimmten Haltestelle kann das angezeigte Ziel wechseln oder anders dargestellt werden.
    /// Dafür können Termini definiert werden.
    ///
    /// Usually the destination is the last stop; termini override the displayed destination from specific stops onward.
    pub termini: Vec<PisRouteTerminus>,
    /// Linie und Code der automatisch zu wählenden Folgeroute.
    ///
    /// Line and code of the route to select automatically next.
    pub following_line_code: Option<(u32, u32)>,
}

impl PisRoute {
    /// Erstellt einen neuen PIS-Routendatensatz.
    ///
    /// Creates a new PIS route record.
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
    /// Gibt das aktive Routenziel ab dem aktuellen Haltestellenindex zurück.
    ///
    /// Returns the active route terminus from the current stop index onward.
    pub fn get_current_direction(&self, stop_index: usize) -> Option<PisRouteTerminus> {
        self.termini
            .iter()
            .filter(|terminus| stop_index >= terminus.stop_index)
            .max_by_key(|terminus| terminus.stop_index)
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

/// Zielüberschreibung ab einer bestimmten Haltestelle auf einer Route.
///
/// Destination override applied from a specific stop on a route.
#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PisRouteTerminus {
    /// Index der Haltestelle auf der Route, ab der dieses Ziel gilt.
    ///
    /// Index of the stop of the route, from which this terminus applies
    pub stop_index: usize,
    /// Neuer Zielstationscode.
    ///
    /// New terminus code
    pub code: Option<u32>,
    /// Neue Linie.
    ///
    /// New line
    pub line: Option<u32>,
    /// Neuer Sonderzeichen-Code.
    ///
    /// New special char code
    pub special_char_code: Option<u32>,
    /// Neuer Routing-Code.
    ///
    /// New routing code
    pub routing_code: Option<u32>,
}

impl PisRouteTerminus {
    fn routing_code_hash(&self) -> u32 {
        0
    }

    /// Gibt den Routing-Code zurück oder einen generierten Hash, falls nicht gesetzt.
    ///
    /// Returns the routing code, falling back to a generated hash if unset.
    pub fn routing_code(&self) -> u32 {
        self.routing_code.unwrap_or(self.routing_code_hash())
    }
}

/// PISS-Content-Gruppe passend zur aktiven PIS-Gruppe und einer Fahrzeugklasse.
///
/// PISS content group matching the active PIS group and a vehicle class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PisSpGroup {
    /// Anzeigename der PISS-Gruppe.
    ///
    /// Display name of the PISS group.
    pub name: String,
    /// Content-ID der zugrunde liegenden PIS-Basisgruppe.
    ///
    /// Content id of the underlying basic PIS group.
    pub basic_pis_group: ContentId,
    /// Fahrzeugklasse, für die diese PISS-Gruppe gilt.
    ///
    /// Vehicle class this PISS group applies to.
    pub class: String,
    /// Zusätzlicher Linienstring aus der PISS-Gruppe.
    ///
    /// Additional line string from the PISS group.
    pub add_lines: String,
    /// Zusätzliche Linienstrings pro Station.
    ///
    /// Additional line strings per station.
    pub add_lines_stations: Vec<PisSpAddLines>,
    /// In der PISS-Gruppe definierte Routen.
    ///
    /// Routes defined in the PISS group.
    pub routes: Vec<PisSpRoute>,
}

/// Zusätzlicher Linien-Text für eine Station in einer PISS-Gruppe.
///
/// Additional line text for a station in a PISS group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PisSpAddLines {
    /// Stationscode, zu dem die Zusatzlinien gehören.
    ///
    /// Station code the additional lines belong to.
    pub code: i32,
    /// Zusätzlicher Linien-Anzeigestring.
    ///
    /// Additional line display string.
    pub lines: String,
}

/// Routendatensatz in einer PISS-Content-Gruppe.
///
/// Route record stored in a PISS content group.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PisSpRoute {
    /// Routencode innerhalb der PISS-Gruppe.
    ///
    /// Route code within the PISS group.
    pub code: i32,
    /// Linien-Anzeigestring für diese Route.
    ///
    /// Line display string for this route.
    pub lines: String,
    /// Haltestellen-Anzeigestrings entlang der Route.
    ///
    /// Stop display strings along the route.
    pub stop_lines: Vec<String>,
}
impl PisSpGroup {
    /// Liefert die ContentId der PISS-Gruppe, die zur aktiven PIS-Gruppe passt und die gegebene Klasse hat.
    ///
    /// Returns the PISS group content id for the active PIS group and the given class.
    #[cfg(feature = "ffi")]
    pub fn get_content_id(class: &str) -> Option<ContentId> {
        let class = lotus_script_sys::FfiObject::new(&class);
        let content_id = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_content_id(class.packed())
        });
        content_id.deserialize()
    }

    /// Liefert die zusätzlichen Linien aus der gegebenen PISS-Gruppe.
    ///
    /// Returns the additional line string from the given PISS group.
    #[cfg(feature = "ffi")]
    pub fn get_group_strings(content_id: ContentId) -> String {
        let content_id = lotus_script_sys::FfiObject::new(&content_id);
        let lines = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_group_strings(content_id.packed())
        });
        lines.deserialize()
    }

    /// Liefert die zusätzlichen Linien für eine Station aus der gegebenen PISS-Gruppe.
    ///
    /// Returns additional line text for a station from the given PISS group.
    #[cfg(feature = "ffi")]
    pub fn get_station_strings(content_id: ContentId, station_code: u32) -> Option<String> {
        let content_id = lotus_script_sys::FfiObject::new(&content_id);
        let lines = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_station_strings(content_id.packed(), station_code)
        });
        lines.deserialize()
    }

    /// Liefert die Route mit der gegebenen Linie und dem Code aus der gegebenen PISS-Gruppe.
    ///
    /// Returns the route for the given line and code from the given PISS group.
    #[cfg(feature = "ffi")]
    pub fn get_route(content_id: ContentId, line: u32, code: u32) -> Option<PisSpRoute> {
        let content_id = lotus_script_sys::FfiObject::new(&content_id);
        let route = lotus_script_sys::FfiObject::from_packed(unsafe {
            lotus_script_sys::pis::get_sp_route_data(content_id.packed(), line, code)
        });
        route.deserialize()
    }
}
