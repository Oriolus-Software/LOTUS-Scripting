#[macro_export]
macro_rules! script {
    ($t:ident) => {
        static SCRIPT: ::std::sync::LazyLock<::std::sync::Mutex<$t>> =
            ::std::sync::LazyLock::new(Default::default);

        #[no_mangle]
        pub fn init() {
            SCRIPT.lock().unwrap().init();
        }

        #[no_mangle]
        pub fn register_actions() {
            let actions = $t::actions();
            $crate::action::register_many(&actions);
        }

        #[no_mangle]
        pub fn tick() {
            SCRIPT.lock().unwrap().tick();
        }

        #[no_mangle]
        pub fn late_tick() {
            for message in $crate::message::get() {
                SCRIPT.lock().unwrap().on_message(message);
            }
        }
    };
}
