use dyn_clone::DynClone;
use std::fmt::{Debug, Display, Formatter};

pub trait Module: Display + Debug + DynClone {
    fn get_pulses(&mut self, input_offset: u16, is_high: bool) -> Option<(&Vec<(usize, u16)>, bool)>;

    fn get_name(&self) -> &str;

    fn is_broadcaster(&self) -> bool {
        false
    }

    fn get_type(&self) -> ModuleType;
}

dyn_clone::clone_trait_object!(Module);

#[derive(Debug, Clone)]
pub struct Broadcaster {
    pub output: Vec<(usize, u16)>,
}

impl Broadcaster {
    pub fn new(output: Vec<(usize, u16)>) -> Self {
        Self { output }
    }
}

impl Display for Broadcaster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "broadcaster -> {:?}", self.output)
    }
}

#[derive(Debug, Clone)]
pub struct FlipFlop {
    pub name: String,
    pub memory: bool,
    pub output: Vec<(usize, u16)>,
}

impl FlipFlop {
    pub fn new(name: String, output: Vec<(usize, u16)>) -> Self {
        Self {
            name,
            memory: false,
            output,
        }
    }
}

impl Display for FlipFlop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{} -> {:?} (memory: {})", self.name, self.output, self.memory)
    }
}

#[derive(Debug, Clone)]
pub struct Conjunction {
    pub name: String,
    pub value: u16,
    pub mask: u16,
    pub output: Vec<(usize, u16)>,
}

impl Conjunction {
    pub fn new(name: String, output: Vec<(usize, u16)>, input_count: u16) -> Self {
        Self {
            name,
            value: 0,
            mask: (1 << input_count) - 1,
            output,
        }
    }
}

impl Display for Conjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "&{} -> {:?} (value: {:#010b}, mask: {:#010b})",
            self.name, self.output, self.value, self.mask
        )
    }
}

#[derive(Debug, Clone)]
pub struct Untyped {
    pub name: String,
}

impl Untyped {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for Untyped {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Untyped,
}

#[derive(Debug, Clone)]
pub struct CableManagement {
    pub modules: Vec<Box<dyn Module>>,
}

impl Display for CableManagement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Modules:")?;
        for module in &self.modules {
            writeln!(f, " * {}", module)?;
        }
        Ok(())
    }
}
