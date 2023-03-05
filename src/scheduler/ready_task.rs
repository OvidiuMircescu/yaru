use super::scheduled_task;
use super::state;
use super::observers;
pub struct ReadyTasksManager{
    ready_tasks : Vec<scheduled_task::ScheduledTaskRef>
}

impl ReadyTasksManager{
    pub fn new() -> ReadyTasksManager{
        ReadyTasksManager{ ready_tasks : Vec::new()}
    }

    pub fn take(&mut self)->Vec<scheduled_task::ScheduledTaskRef>{
        std::mem::take(&mut self.ready_tasks)
    }

    pub fn add(&mut self, task:scheduled_task::ScheduledTaskRef){
        self.ready_tasks.push(task);
    }
}
pub type ReadyTasksManagerRef = std::rc::Rc<std::cell::RefCell<ReadyTasksManager>>;
pub struct ReadyTaskObserver{
    manager : ReadyTasksManagerRef,
    subject_task : scheduled_task::ScheduledTaskWeakRef
}

impl ReadyTaskObserver{
    pub fn new(manager:ReadyTasksManagerRef, subject_task:scheduled_task::ScheduledTaskRef)-> ReadyTaskObserver{
        ReadyTaskObserver{
            manager,
            subject_task: std::rc::Rc::downgrade(&subject_task)
        }
    }
}

impl observers::Observer for ReadyTaskObserver{
    fn notify(&mut self, from_state:&state::TaskState) {
        match from_state{
            state::TaskState::Ready => if let Some(obs) = self.subject_task.upgrade(){
                                        self.manager.borrow_mut().add(obs)
                                },
            _ => ()
        }
    }
}
