use super::state;
use super::observers;
use crate::task_declaration;

pub trait TaskExecution{
  fn run(&mut self, sched : &mut super::Scheduler);
  fn set_state(&mut self, new_state : state::TaskState);
  fn register(&mut self, obs: Box <dyn observers::Observer>);
}

pub struct SimpleTaskExecution{
    state: state::StateManager,
    definition : task_declaration::SimpleTask
}

impl SimpleTaskExecution{
    pub fn new(definition : task_declaration::SimpleTask) -> SimpleTaskExecution{
        SimpleTaskExecution{
            state: state::StateManager::new(),
            definition
        }
    }
}

impl TaskExecution for SimpleTaskExecution{
    fn run(&mut self, _ : &mut super::Scheduler) {
        self.state.set_state(state::TaskState::Running);
        self.definition.run();
        self.state.set_state(state::TaskState::Done);
    }

    fn set_state(&mut self, new_state : state::TaskState) {
        self.state.set_state(new_state);
    }

    fn register(&mut self, obs: Box<dyn observers::Observer>) {
        self.state.register(obs);
    }
}

pub struct BlocExecution{
    state: BlocStateManagerRef,
    tasks: Option<Vec<task_declaration::TaskDeclaration>>
}

impl BlocExecution{
    pub fn new(tasks: Vec<task_declaration::TaskDeclaration>) -> BlocExecution{
        BlocExecution{
            state : std::rc::Rc::new(std::cell::RefCell::new(BlocStateManager::new(tasks.len()))),
            tasks : Some(tasks)
        }
    }
}

impl TaskExecution for BlocExecution{
    fn run(&mut self, sched : &mut super::Scheduler) {
        let tasks = self.tasks.take();
        if let Some(tasks) = tasks{
            if tasks.len() > 0 {
                self.set_state(state::TaskState::Running);
                for declaration in tasks{
                    // The bloc is ready, thus there is no dependecy to add.
                    let new_task_exe = sched.submit(declaration, &[]);
                    let obs:Box <dyn observers::Observer> = Box::new(BlocStateObserver::new(self.state.clone()));
                    new_task_exe.register(obs);
                }
            }else{
                // empty bloc
                self.set_state(state::TaskState::Done);
            }
        }
    }

    fn set_state(&mut self, new_state : state::TaskState) {
        self.state.borrow_mut().set_state(new_state);
    }

    fn register(&mut self, obs: Box<dyn observers::Observer>) {
        self.state.borrow_mut().register(obs);
    }
}

type BlocStateManagerRef = std::rc::Rc<std::cell::RefCell<BlocStateManager>>;
struct BlocStateManager{
    state: state::StateManager,
    content_size : usize
}

impl BlocStateManager{
    fn new(content_size : usize) -> BlocStateManager{
        let mut result =BlocStateManager{
            state: state::StateManager::new(),
            content_size
        };
        if content_size == 0{
            result.state.set_state(state::TaskState::Done);
        }
        result
    }

    fn set_state(&mut self, new_state : state::TaskState) {
        self.state.set_state(new_state);
    }

    fn register(&mut self, obs: Box<dyn observers::Observer>) {
        self.state.register(obs);
    }

    fn notify_content_done(&mut self){
        self.content_size -= 1;
        if self.content_size == 0{
            self.state.set_state(state::TaskState::Done);
        }
    }
}

struct BlocStateObserver{
    manager : BlocStateManagerRef
}

impl BlocStateObserver{
    fn new(manager : BlocStateManagerRef)->BlocStateObserver{
        BlocStateObserver{manager}
    }
}

impl observers::Observer for BlocStateObserver{
    fn notify(&mut self, from:&state::TaskState) {
        match from{
            state::TaskState::Done => self.manager.borrow_mut().notify_content_done(),
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_build(){
    }
}
