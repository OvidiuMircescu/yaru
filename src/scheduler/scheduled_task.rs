use crate::task_declaration;
use super::state;
use super::observers;

pub type ScheduledTaskRef = std::rc::Rc<std::cell::RefCell<ScheduledTask>>;
pub type ScheduledTaskWeakRef = std::rc::Weak<std::cell::RefCell<ScheduledTask>>;
pub struct ScheduledTask{
    task : Box<dyn task_declaration::TaskDeclaration>,
    // id : TaskId,
    number_of_dependencies : usize,
    state : state::StateManager
}

impl ScheduledTask{
    pub fn new(task : Box<dyn task_declaration::TaskDeclaration>,
               dependencies : &[super::task_info::TaskInfo]
              )-> ScheduledTask{
        let number_of_dependencies = dependencies.len();
        let mut result = ScheduledTask{
            task,
            // id,
            number_of_dependencies,
            state : state::StateManager::new()
        };
        if number_of_dependencies == 0 {
            result.state.set_state(state::TaskState::Ready)
        };
        result
    }

    pub fn run(&mut self, sched : &mut super::Scheduler){
        self.state.set_state( state::TaskState::Running);
        self.task.run();
        self.state.set_state(state::TaskState::Done);
    }

    pub fn register(&mut self, obs: Box <dyn observers::Observer>){
        self.state.register(obs);
    }

    pub fn notify_from_dependency(&mut self, state_from:&state::TaskState){
        match state_from{
            state::TaskState::Done => {
                self.number_of_dependencies -= 1;
                if self.number_of_dependencies == 0{
                    self.state.set_state(state::TaskState::Ready);
                }
            },
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_state()
    {
    }
}