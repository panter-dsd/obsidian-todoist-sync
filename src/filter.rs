use clap::ValueEnum;

#[derive(Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum FilterState {
    Completed,
    Uncompleted,
    InProgress,
    Unknown,
}

pub struct Filter {
    pub states: Vec<FilterState>,
    pub today: bool,
}
