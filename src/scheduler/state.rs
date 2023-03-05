use super::observers::Observer;
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskState{
    Waiting,
    Ready,
    Running,
    Done
}

impl std::fmt::Display for TaskState{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            TaskState::Waiting => write!(f, "Waiting"),
            TaskState::Ready   => write!(f, "Ready"),
            TaskState::Running => write!(f, "Running"),
            TaskState::Done    => write!(f, "Done"),
        }
    }
}

pub struct StateManager{
    state : TaskState,
    observers : Vec<Box <dyn Observer>>
}

impl StateManager{
    pub fn new() -> StateManager{
        StateManager{
            state : TaskState::Waiting,
            observers : Vec::new()
        }
    }

    pub fn register(&mut self, mut obs: Box <dyn Observer>){
        obs.notify(&self.state);
        self.observers.push(obs);
    }

    pub fn set_state(&mut self, state:TaskState){
        if self.state != state{
            self.state = state;
            self.notify_observers();
        }
    }

    fn notify_observers(&mut self){
        for obs in &mut self.observers{
            obs.notify(&self.state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state()
    {
        let t1 = TaskState::Ready;
        let t2 = TaskState::Done;
        let t3 = TaskState::Ready;
        assert!(t1 !=t2);
        assert!(t1 == t3);
        assert!(t1.to_string() == "Ready");
        assert!(t2.to_string() == "Done");
    }
}