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
    use crate::scheduler::scheduled_task::ScheduledTask;
    use crate::task_declaration::SimpleTask;
    #[test]
    fn test_build()
    {
        let simptask = SimpleTask::new(Box::new(|| println!("hehe!")));
        let _schedtask = ScheduledTask::new(Box::new(simptask), &[]);
        // let depobs = EmptyDependencyObserver::new(std::rc::Rc::new(std::cell::RefCell::new(schedtask)));
    }
}