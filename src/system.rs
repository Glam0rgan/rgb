/// Configuration of the emulator.
pub struct Config {
    /// CPU frequency.
    pub(crate) freq: u64,
    /// Cycle sampling count in the CPU frequency controller.
    pub(crate) sample: u64,
    /// Delay unit in CPU frequency controller.
    pub(crate) delay_unit: u64,
    /// Don't adjust CPU frequency.
    pub(crate) native_speed: bool,
}

impl Config {
    /// Create the default configuration.
    pub fn new() -> Self {
        let freq = 4194300; // 4.1943 MHz
        Self {
            freq,
            sample: freq / 1000,
            delay_unit: 10,
            native_speed: false,
        }
    }

    /// Set the CPU frequency.
    pub fn freq(mut self, freq: u64) -> Self {
        self.freq = freq;
        self
    }

    /// Set the sampling count of the CPU frequency controller.
    pub fn sample(mut self, sample: u64) -> Self {
        self.sample = sample;
        self
    }

    /// Set the delay unit.
    pub fn delay_unit(mut self, delay: u64) -> Self {
        self.delay_unit = delay;
        self
    }

    /// Set the flag to run at native speed.
    pub fn native_speed(mut self, native: bool) -> Self {
        self.native_speed = native;
        self
    }
}

/// Represents the entire emulator context.
pub struct System<D> {
    cfg: Config,
    hw: HardwareHandle,
    fc: FreqControl,
    cpu: Cpu,
    mmu: Option<Mmu>,
    dbg: Device<D>,
    ic: Device<Ic>,
    timer: Device<Timer>,
    serial: Device<Serial>,
}

impl<D> System<D>
where
    D: Debugger + 'static,
{
    /// Create a new emulator context;
    pub fn new<T>(cfg: Config, rom: &[u8], hw: T, dbg: D) -> Self
    where
        T: Hardware + 'static,
    {
        info!("Init...");

        let hw = HardwareHandle::new(hw);
        
        let cpu = Cpu::new();
        let mut mmu = Mmu::new();
        let ic = Device::new(Ic::new());
        let irq = ic.borrow().irq().clone();
        let timer = Device::new(Timer::new(irq.clone()));

        mmu.add_handler((0xff0f, 0xff0f), ic.handler());
        mmu.add_handler((0xffff, 0xffff), ic.handler());

        mmu.add_handler((0xff04, 0xff07), timer.handler());

        info!("Starting...");

        fc.reset();

        let mmu = Some(mmu);

        Self {
            cfg,
            hw,
            fc,
            cpu,
            mmu,
            ic,
            timer,
        }
    }
    fn step(&mut self, mut mmu: Mmu) -> Mmu {

        let time = self.cpu.execute(&mut mmu);

        time += self.cpu.check_interrupt(&mut mmu, &self.ic);
        
        self.timer.borrow_mut().step(time);

        mmu
    }

    /// Run a single step of emulation.
    /// This function needs to be called repeatedly until it returns `false`.
    /// Returning `false` indicates the end of emulation, and the functions shouldn't be called again.
    pub fn poll(&mut self) -> bool {
        if !self.hw.get().borrow_mut().sched() {
            return false;
        }

        let mmu = self.mmu.take().unwrap();
        self.mmu = Some(self.step(mmu));

        true
    }
}

/// Run the emulator with the given configuration.
pub fn run<T: Hardware + 'static>(cfg: Config, rom: &[u8], hw: T) {
    run_inner(cfg, rom, hw, Debugger::empty())
}

/// Run the emulator with the given configuration and debugger.
pub fn run_debug<T: Hardware + 'static, D: Debugger + 'static>(
    cfg: Config,
    rom: &[u8],
    hw: T,
    dbg: D,
) {
    run_inner(cfg, rom, hw, dbg)
}

fn run_inner<T: Hardware + 'static, D: Debugger + 'static>(cfg: Config, rom: &[u8], hw: T, dbg: D) {
    let mut sys = System::new(cfg, rom, hw, dbg);
    while sys.poll() {}
}