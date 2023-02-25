use super::scheduled_task::*;

pub struct ReadyTasksManager{
    ready_tasks : Vec<ScheduledTaskRef>
}

impl ReadyTasksManager{
    pub fn new() -> ReadyTasksManager{
        ReadyTasksManager{ ready_tasks : Vec::new()}
    }

    pub fn take(&mut self)->Vec<ScheduledTaskRef>{
        std::mem::take(&mut self.ready_tasks)
    }

    pub fn add(&mut self, task:ScheduledTaskRef){
        self.ready_tasks.push(task);
    }
}
pub type ReadyTasksManagerRef = std::rc::Rc<std::cell::RefCell<ReadyTasksManager>>;
pub struct ReadyTaskObserver{
    manager : ReadyTasksManagerRef,
    subject_task : ScheduledTaskWeakRef
}

impl ReadyTaskObserver{
    pub fn new(manager:ReadyTasksManagerRef, subject_task:ScheduledTaskRef)-> ReadyTaskObserver{
        ReadyTaskObserver{
            manager,
            subject_task: std::rc::Rc::downgrade(&subject_task)
        }
    }
}

impl Observer for ReadyTaskObserver{
    fn notify(&mut self, from_state:&TaskState) {
        match from_state{
            TaskState::Ready => if let Some(obs) = self.subject_task.upgrade(){
                                        self.manager.borrow_mut().add(obs)
                                },
            _ => ()
        }
    }
}
