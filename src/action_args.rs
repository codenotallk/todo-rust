#[derive(Debug)]
pub struct ActionArgs {
    pub command: Option<String>,
    pub first: Option<String>,
    pub second: Option<String>,
    pub third: Option<String>,
}

impl ActionArgs {
    pub fn new(command: &str) -> Self {
        Self {
            command: Some(command.to_owned()),
            ..Default::default()
        }
    }
}

impl Default for ActionArgs {
    fn default() -> Self {
        Self {
            command: Default::default(),
            first: Default::default(),
            second: Default::default(),
            third: Default::default(),
        }
    }
}

pub struct ActionArgsBuilder(ActionArgs);

impl ActionArgsBuilder {
    pub fn new() -> Self {
        Self(ActionArgs {
            ..Default::default()
        })
    }

    pub fn with_command(mut self, command: &str) -> Self {
        self.0.command = Some(command.to_owned());
        self
    }

    pub fn with_first(mut self, field: String) -> Self {
        self.0.first = Some(field);
        self
    }

    pub fn with_second(mut self, field: String) -> Self {
        self.0.second = Some(field);
        self
    }

    pub fn with_third(mut self, field: String) -> Self {
        self.0.third = Some(field);
        self
    }

    pub fn build(self) -> ActionArgs {
        self.0
    }
}
