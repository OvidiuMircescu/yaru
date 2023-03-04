use super::task_declaration;

mod scheduled_task;
use scheduled_task::*;

mod observers;
use observers::*;

mod ready_task;
use ready_task::*;

mod task_info;

pub type TaskId = usize;
pub type TaskInfo = task_info::TaskInfo;
pub struct Scheduler{
    all_tasks : std::collections::HashMap<TaskId, ScheduledTaskRef>,
    last_id : TaskId,
    ready_tasks : ReadyTasksManagerRef
}

// pub type SchedulerRef = std::rc::Rc<std::cell::RefCell<Scheduler>>;
// pub type SchedulerWeakRef = std::rc::Weak<std::cell::RefCell<Scheduler>>;

impl Scheduler{
    pub fn new()-> Scheduler{
        Scheduler{
            all_tasks : std::collections::HashMap::new(),
            last_id : 0,
            ready_tasks : std::rc::Rc::new(std::cell::RefCell::new(ReadyTasksManager::new())),
        }
    }

    pub fn submit(&mut self,
                  task : Box<dyn task_declaration::TaskDeclaration>,
                  dependencies : &[TaskInfo]
                 ) -> TaskInfo{
        let id = self.last_id;
        self.last_id += 1;
        // let dependencies = task.dependencies().clone();
        let new_task = ScheduledTask::new(task, dependencies);
        let new_task = std::rc::Rc::new(std::cell::RefCell::new(new_task));
        self.all_tasks.insert(id, new_task.clone());
        for dep_task in dependencies.into_iter(){
            let observer = Box::new(DependencyObserver::new(new_task.clone()));
            dep_task.register(observer);
        }
        let obs:Box <dyn Observer> = Box::new(
            ReadyTaskObserver::new(self.ready_tasks.clone(), new_task.clone()));
        new_task.borrow_mut().register(obs);
        TaskInfo::new(new_task)
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
    use super::*;
    use crate::SimpleTask;
    #[test]
    fn test_memory()
    {
        let simptask = SimpleTask::new(Box::new(|| println!("hehe!")));
        let mut sched = Scheduler::new();
        sched.submit(Box::new(simptask), &[]);
        assert_eq!(1, sched.all_tasks.len());
        sched.start();
        assert_eq!(1, sched.all_tasks.len());
        for t in sched.all_tasks.values(){
            assert_eq!(1, std::rc::Rc::strong_count(t));
        }
    }
}