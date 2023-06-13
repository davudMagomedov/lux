use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Change {
    from: String,
    to: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpecialFunction {
    changes: Vec<Change>,
    usage: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    main_changes: Vec<Change>,
    special_funcs: Vec<SpecialFunction>
}

impl Config {
    pub fn main_changes(&self) -> &Vec<Change> {
        &self.main_changes
    }

    pub fn special_funcs(&self) -> &Vec<SpecialFunction> {
        &self.special_funcs
    }
}

impl SpecialFunction {
    pub fn changes(&self) -> &Vec<Change> {
        &self.changes
    }

    pub fn usage(&self) -> &String {
        &self.usage
    }
}

impl Change {
    pub fn get_from(&self) -> &String {
        &self.from
    }

    pub fn get_to(&self) -> &String {
        &self.to
    }
}
