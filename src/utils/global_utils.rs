use super::*;

pub fn identity_function<T>(object: T) -> T {
    object
}

pub fn print_uci_info<T: fmt::Display>(message: &str, info: impl Into<Option<T>>) {
    if !GLOBAL_TIMECAT_STATE.is_in_debug_mode() {
        return;
    }
    let mut to_print = if let Some(info_message) = info.into() {
        format!(
            "{} {}",
            message.colorize(SUCCESS_MESSAGE_STYLE),
            info_message.colorize(INFO_MESSAGE_STYLE),
        )
    } else {
        message.colorize(SUCCESS_MESSAGE_STYLE)
    };
    if GLOBAL_TIMECAT_STATE.is_in_uci_mode() {
        to_print = format!("{} {to_print}", "info string".colorize(INFO_MESSAGE_STYLE))
    }
    println_wasm!("{to_print}");
}

pub struct TimecatDefaults {
    #[cfg(feature = "colored")]
    pub colored: bool,
    pub console_mode: bool,
    pub t_table_size: CacheTableSize,
    pub long_algebraic_notation: bool,
    pub num_threads: NonZeroUsize,
    pub move_overhead: Duration,
    pub use_own_book: bool,
    pub debug_mode: bool,
    pub chess960_mode: bool,
}

pub const TIMECAT_DEFAULTS: TimecatDefaults = TimecatDefaults {
    #[cfg(feature = "colored")]
    colored: true,
    console_mode: true,
    t_table_size: CacheTableSize::Exact(16),
    long_algebraic_notation: false,
    num_threads: const { unsafe { NonZeroUsize::new_unchecked(1) } },
    move_overhead: Duration::from_millis(200),
    use_own_book: false,
    debug_mode: true,
    chess960_mode: false,
};

#[derive(Debug)]
pub struct GlobalTimecatState {
    #[cfg(feature = "colored")]
    _colored: AtomicBool,
    _console_mode: AtomicBool,
    _t_table_size: RwLock<CacheTableSize>,
    _long_algebraic_notation: AtomicBool,
    _use_own_book: AtomicBool,
    _debug_mode: AtomicBool,
    _chess960_mode: AtomicBool,
}

impl Default for GlobalTimecatState {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalTimecatState {
    pub const fn new() -> Self {
        GlobalTimecatState {
            #[cfg(feature = "colored")]
            _colored: AtomicBool::new(TIMECAT_DEFAULTS.colored),
            _console_mode: AtomicBool::new(TIMECAT_DEFAULTS.console_mode),
            _t_table_size: RwLock::new(TIMECAT_DEFAULTS.t_table_size),
            _long_algebraic_notation: AtomicBool::new(TIMECAT_DEFAULTS.long_algebraic_notation),
            _use_own_book: AtomicBool::new(TIMECAT_DEFAULTS.use_own_book),
            _debug_mode: AtomicBool::new(TIMECAT_DEFAULTS.debug_mode),
            _chess960_mode: AtomicBool::new(TIMECAT_DEFAULTS.chess960_mode),
        }
    }

    #[cfg(feature = "colored")]
    #[inline]
    pub fn is_colored(&self) -> bool {
        self._colored.load(MEMORY_ORDERING)
    }

    #[cfg(not(feature = "colored"))]
    #[inline]
    pub fn is_colored(&self) -> bool {
        false
    }

    #[cfg(feature = "colored")]
    pub fn set_colored(&self, b: bool, verbose: bool) {
        self._colored.store(b, MEMORY_ORDERING);
        if verbose {
            print_uci_info("Colored output is set to", b);
        }
    }

    #[inline]
    pub fn is_in_console_mode(&self) -> bool {
        self._console_mode.load(MEMORY_ORDERING)
    }

    #[inline]
    pub fn is_in_uci_mode(&self) -> bool {
        !self.is_in_console_mode()
    }

    pub fn set_console_mode(&self, b: bool, verbose: bool) {
        self._console_mode.store(b, MEMORY_ORDERING);
        self._debug_mode.store(b, MEMORY_ORDERING);
        if verbose {
            force_println_info("Console mode is set to", b);
        }
    }

    pub fn set_uci_mode(&self, b: bool, verbose: bool) {
        self.set_console_mode(!b, false);
        if verbose {
            force_println_info("UCI mode is set to", b);
        }
    }

    #[inline]
    pub fn set_to_uci_mode(&self) {
        self.set_uci_mode(true, false);
    }

    #[inline]
    pub fn set_to_console_mode(&self) {
        self.set_console_mode(true, false);
    }

    #[inline]
    pub fn get_t_table_size(&self) -> CacheTableSize {
        self._t_table_size.read().unwrap().to_owned()
    }

    pub fn set_t_table_size(&self, transposition_table: &TranspositionTable, size: CacheTableSize) {
        //TODO: modify such that T Table and evaluation function takes same amount of space
        *self._t_table_size.write().unwrap() = size;
        transposition_table.reset_size();
        if GLOBAL_TIMECAT_STATE.is_in_debug_mode() {
            transposition_table.print_info();
        }
        print_uci_info(
            "Transposition table is set to size to",
            size.to_cache_table_memory_size::<TranspositionTableEntry>(),
        );
    }

    #[inline]
    pub fn use_long_algebraic_notation(&self) -> bool {
        self._long_algebraic_notation.load(MEMORY_ORDERING)
    }

    pub fn set_long_algebraic_notation(&self, b: bool) {
        self._long_algebraic_notation.store(b, MEMORY_ORDERING);
        print_uci_info("Long algebraic notation is set to", b);
    }

    #[inline]
    pub fn use_own_book(&self) -> bool {
        self._use_own_book.load(MEMORY_ORDERING)
    }

    pub fn set_using_own_book(&self, b: bool) {
        self._use_own_book.store(b, MEMORY_ORDERING);
        print_uci_info("Own Book Usage is set to", b);
    }

    #[inline]
    pub fn is_in_debug_mode(&self) -> bool {
        self._debug_mode.load(MEMORY_ORDERING)
    }

    pub fn set_debug_mode(&self, b: bool) {
        self._debug_mode.store(b, MEMORY_ORDERING);
        print_uci_info("Debug Mode is set to", b);
    }

    #[inline]
    pub fn is_in_console_and_debug_mode(&self) -> bool {
        self.is_in_console_mode() && self.is_in_debug_mode()
    }

    #[inline]
    pub fn is_in_chess960_mode(&self) -> bool {
        self._chess960_mode.load(MEMORY_ORDERING)
    }

    pub fn set_chess960_mode(&self, b: bool) {
        self._chess960_mode.store(b, MEMORY_ORDERING);
        print_uci_info("Chess 960 mode is set to", b);
    }
}
