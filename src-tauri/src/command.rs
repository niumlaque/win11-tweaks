use crate::win;

#[derive(Debug)]
pub struct Value {
    pub value: String,
    pub description: String,
}

impl Value {
    pub fn new(value: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub id: u64,
    pub label: String,
    pub def: win::reg::RegDef,
    pub values: Vec<Value>,
}

#[derive(Default)]
pub struct CommandManager {
    id_counter: u64,
}

impl CommandManager {
    pub fn gen(
        &mut self,
        label: impl Into<String>,
        def: win::reg::RegDef,
        values: Vec<Value>,
    ) -> Command {
        if self.id_counter < u64::MAX {
            self.id_counter += 1;
        } else {
            panic!();
        }
        Command {
            id: self.id_counter,
            label: label.into(),
            def,
            values,
        }
    }
}
