use crate::generaltask;

mod scheduledtask;
use scheduledtask::*;

pub type TaskId = usize;
pub type ScheduledTaskRef = std::rc::Rc<std::cell::RefCell<ScheduledTask>>;

struct ReadyTasksManager{
    ready_tasks : Vec<ScheduledTaskRef>
}

impl ReadyTasksManager{
    fn new() -> ReadyTasksManager{
        ReadyTasksManager{ ready_tasks : Vec::new()}
    }

    fn take(&mut self)->Vec<ScheduledTaskRef>{
        std::mem::take(&mut self.ready_tasks)
    }

    fn add(&mut self, task:ScheduledTaskRef){
        self.ready_tasks.push(task);
    }
}
type ReadyTasksManagerRef = std::rc::Rc<std::cell::RefCell<ReadyTasksManager>>;
struct ReadyTaskObserver{
    manager : ReadyTasksManagerRef,
    subject_task : ScheduledTaskRef
}

impl ReadyTaskObserver{
    fn new(manager:ReadyTasksManagerRef, subject_task:ScheduledTaskRef)-> ReadyTaskObserver{
        ReadyTaskObserver{manager,subject_task}
    }
}

impl Observer for ReadyTaskObserver{
    fn notify(&mut self, from_state:&TaskState) {
        match from_state{
            TaskState::Ready => self.manager.borrow_mut().add(self.subject_task.clone()),
            _ => ()
        }
    }
}
pub struct Scheduler{
    all_tasks : std::collections::HashMap<TaskId, ScheduledTaskRef>,
    last_id : TaskId,
    ready_tasks : ReadyTasksManagerRef
}

impl Scheduler{
    pub fn new()-> Scheduler{
        Scheduler{
            all_tasks : std::collections::HashMap::new(),
            last_id : 0,
            ready_tasks : std::rc::Rc::new(std::cell::RefCell::new(ReadyTasksManager::new())),
        }
    }

    pub fn submit(&mut self, task : Box<dyn generaltask::GeneralTask>) -> TaskId{
        let id = self.last_id;
        self.last_id += 1;
        let dependencies: Vec<ScheduledTaskRef> = task.dependencies()
            .iter()
            .filter_map(|x|self.all_tasks.get(x))
            .map(|x| x.clone())
            .collect();
        let new_task =ScheduledTask::new(task) ;
        let new_task = std::rc::Rc::new(std::cell::RefCell::new(new_task));
        self.all_tasks.insert(id, new_task.clone());
        for task in dependencies.iter(){
            let observer = Box::new(DependencyObserver::new(new_task.clone()));
            task.borrow_mut().register(observer);
        }
        let obs:Box <dyn Observer> = Box::new(
            ReadyTaskObserver::new(self.ready_tasks.clone(), new_task.clone()));
        new_task.borrow_mut().register(obs);
        id
    }

    pub fn start(&mut self){
        let mut ready_tasks = self.ready_tasks.borrow_mut().take();
        while !ready_tasks.is_empty() {
            for task in ready_tasks{
                task.borrow_mut().run();
            }
            ready_tasks = self.ready_tasks.borrow_mut().take();
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_build()
    {
    }
}