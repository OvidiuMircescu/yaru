use super::scheduled_task::*;

pub struct DependencyObserver{
    observer : ScheduledTaskWeakRef,
}

impl DependencyObserver{
    pub fn new(observer: ScheduledTaskRef)->DependencyObserver{
        DependencyObserver {
            observer : std::rc::Rc::downgrade(&observer)
        }
    }
}

impl Observer for DependencyObserver{
    fn notify(&mut self, from:&TaskState) {
        if let Some(obs) = self.observer.upgrade(){
            obs.borrow_mut().notify_from_dependency(from);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_declaration::SimpleTask;
    #[test]
    fn test_build()
    {
        let simptask = SimpleTask::new(&[], Box::new(|| println!("hehe!")));
        let _schedtask = ScheduledTask::new(Box::new(simptask));
        // let depobs = EmptyDependencyObserver::new(std::rc::Rc::new(std::cell::RefCell::new(schedtask)));
    }
}