// use super::scheduled_task::*;
use super::state;

pub trait Observer {
    fn notify(&mut self, from:&state::TaskState);
}

pub struct DependencyObserver{
    observer : super::scheduled_task::ScheduledTaskWeakRef,
}

impl DependencyObserver{
    pub fn new(observer: super::scheduled_task::ScheduledTaskRef)->DependencyObserver{
        DependencyObserver {
            observer : std::rc::Rc::downgrade(&observer)
        }
    }
}

impl Observer for DependencyObserver{
    fn notify(&mut self, from:&state::TaskState) {
        if let Some(obs) = self.observer.upgrade(){
            obs.borrow_mut().notify_from_dependency(from);
        }
    }
}


#[cfg(test)]
mod tests {
    // use super::*;
    use crate::scheduler::scheduled_task;
    use crate::task_declaration;
    #[test]
    fn test_build()
    {
        let simptask = task_declaration::SimpleTask::new(Box::new(|| println!("hehe!")));
        let _schedtask = scheduled_task::ScheduledTask::new(task_declaration::TaskDeclaration::Simple(simptask), &[]);
        // let depobs = EmptyDependencyObserver::new(std::rc::Rc::new(std::cell::RefCell::new(schedtask)));
    }
}