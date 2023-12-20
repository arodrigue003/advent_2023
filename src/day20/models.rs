use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Untyped,
}

impl Display for ModuleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModuleType::Broadcaster => "",
                ModuleType::FlipFlop => "%",
                ModuleType::Conjunction => "&",
                ModuleType::Untyped => "",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub name: String,

    // For conjunction
    pub value: u16,
    pub mask: u16,

    // For flipflop
    pub memory: bool,

    pub module_type: ModuleType,
    // Each element correspond to the module position with the position inside the target module.
    pub output: Vec<(usize, u16)>,
}

impl Module {
    pub fn new(name: String, module_type: ModuleType, output: Vec<(usize, u16)>, input_count: u16) -> Self {
        Self {
            name: if module_type == ModuleType::Broadcaster {
                "broadcaster".to_string()
            } else {
                name
            },
            value: 0,
            mask: (1 << input_count) - 1,
            memory: false,
            module_type,
            output,
        }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.module_type {
            ModuleType::Broadcaster | ModuleType::FlipFlop | ModuleType::Conjunction => write!(
                f,
                "{}{} -> {:?} (memory: {}, value: {:#010b}, mask: {:#010b})",
                self.module_type, self.name, self.output, self.memory, self.value, self.mask
            ),
            ModuleType::Untyped => write!(f, "{}", self.name),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CableManagement {
    pub modules: Vec<Module>,
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
