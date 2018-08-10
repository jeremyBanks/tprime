#[derive(Default)]
pub struct History<State: Clone> {
    initial: State,
    commands: Vec<Box<dyn Fn(&mut State)>>,
    current: State,
}

impl<State: Clone> History<State> {
    pub fn execute(&mut self, command: impl Fn(&mut State) + 'static) {
        command(&mut self.current);
        self.commands.push(Box::new(command));
    }

    pub fn get(&self) -> State {
        self.current.clone()
    }

    fn at_t(&self, t: usize) -> Option<State> {
        let max_t = self.commands.len();
        if t > max_t {
            None
        } else if t == max_t {
            return Some(self.current.clone());
        } else {
            let mut state = self.initial.clone();
            for command in self.commands.iter().take(t) {
                command(&mut state);
            }
            Some(state)
        }
    }

    pub fn undo(&mut self) -> Option<Box<dyn Fn(&mut State)>> {
        if self.commands.len() == 0 {
            return None;
        }

        let new_current = self.at_t(self.commands.len() - 1).unwrap();
        let removed_command = self.commands.pop().unwrap();
        self.current = new_current;

        Some(removed_command)
    }
}
