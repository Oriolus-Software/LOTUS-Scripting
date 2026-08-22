/// Registers a LOTUS script type and exports the WASM entry points.
///
/// Expands to `init`, `register_actions`, `tick`, and `late_tick` functions that
/// delegate to the [`Script`] implementation on the given type. Incoming engine
/// messages are delivered in `late_tick` via [`Script::on_message`].
///
/// # Example
///
/// ```no_run
/// use lotus_script::{script, Script};
///
/// struct MyScript;
///
/// impl Script for MyScript {
///     fn tick(&mut self) {
///         // ...
///     }
/// }
///
/// script!(MyScript);
/// ```
#[macro_export]
macro_rules! script {
    ($t:ident) => {
        thread_local! {
            static SCRIPT: ::std::sync::LazyLock<::std::sync::Mutex<$t>> =
                ::std::sync::LazyLock::new(Default::default);
        }

        #[no_mangle]
        pub fn init() {
            SCRIPT.with(|s| s.lock().unwrap().init());
        }

        #[no_mangle]
        pub fn register_actions() {
            let actions = $t::actions();
            $crate::action::register_many(&actions);
        }

        #[no_mangle]
        pub fn tick() {
            SCRIPT.with(|s| s.lock().unwrap().tick());
        }

        #[no_mangle]
        pub fn late_tick() {
            for message in $crate::message::get() {
                SCRIPT.with(|s| s.lock().unwrap().on_message(message));
            }
        }
    };
}
