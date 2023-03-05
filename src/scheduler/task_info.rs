use super::scheduled_task;
use super::observers;

/// Public interface to a scheduled task.
#[derive(Clone)]
pub struct TaskInfo{
    followed_task : scheduled_task::ScheduledTaskRef
}

impl TaskInfo{
    pub fn new(followed_task : scheduled_task::ScheduledTaskRef) -> TaskInfo{
        TaskInfo{followed_task}
    }
    pub fn register(&self, obs: Box <dyn observers::Observer>){
        self.followed_task.borrow_mut().register(obs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::state;
    use crate::task_declaration;
    use crate::scheduler;
    struct TestObserver{
        pub state : std::rc::Rc<std::cell::RefCell<String>>
    }

    impl observers::Observer for TestObserver{
        fn notify(&mut self, from:&state::TaskState) {
            self.state.borrow_mut().clear();
            self.state.borrow_mut().push_str(&from.to_string());
        }
    }
    
    #[test]
    fn test_build()
    {
        let simptask = task_declaration::SimpleTask::new(Box::new(|| println!("hehe!")));
        let schedtask = scheduler::ScheduledTask::new(Box::new(simptask), &[]);
        let info = TaskInfo::new(std::rc::Rc::new(std::cell::RefCell::new(schedtask)));
        let state = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let obs = Box::new(TestObserver{state :state.clone()});
        info.register(obs);
        assert_eq!("Ready", state.borrow().as_str());
        let _cln = info.clone();
    }
}