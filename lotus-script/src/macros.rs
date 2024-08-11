#[macro_export]
macro_rules! script {
    ($t:ident) => {
        static SCRIPT: ::std::sync::OnceLock<::std::sync::RwLock<$t>> =
            ::std::sync::OnceLock::new();

        #[no_mangle]
        pub fn init() {
            SCRIPT.get_or_init(Default::default);

            SCRIPT.get().unwrap().write().unwrap().init();
        }

        #[no_mangle]
        pub fn register_actions() {
            let actions = $t::actions();
            $crate::action::register_many(&actions);
        }

        #[no_mangle]
        pub fn tick() {
            SCRIPT
                .get()
                .expect("expected script to be initialized")
                .write()
                .expect("expected script write lock")
                .tick();
        }

        #[no_mangle]
        pub fn late_tick() {
            let mut script = SCRIPT
                .get()
                .expect("expected script to be initialized")
                .write()
                .expect("expected script write lock");

            for message in $crate::message::get() {
                script.on_message(message);
            }
        }
    };
}
