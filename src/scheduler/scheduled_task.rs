use crate::task_declaration;
use super::state;
use super::observers;
use super::task_execution;

pub type ScheduledTaskRef = std::rc::Rc<std::cell::RefCell<ScheduledTask>>;
pub type ScheduledTaskWeakRef = std::rc::Weak<std::cell::RefCell<ScheduledTask>>;
pub struct ScheduledTask{
    task : Box<dyn task_execution::TaskExecution>,
    // id : TaskId,
    number_of_dependencies : usize,
    // state : state::StateManager
}

impl ScheduledTask{
    pub fn new(task_description : task_declaration::TaskDeclaration,
               dependencies : &[super::task_info::TaskInfo]
              )-> ScheduledTask{
        let number_of_dependencies = dependencies.len();
        let task:Box<dyn task_execution::TaskExecution> = match task_description{
            task_declaration::TaskDeclaration::Simple(t) => Box::new(task_execution::SimpleTaskExecution::new(t)),
            task_declaration::TaskDeclaration::Bloc(tasks) => Box::new(task_execution::BlocExecution::new(tasks))
        };
        let mut result = ScheduledTask{
            task,
            // id,
            number_of_dependencies,
        };
        if number_of_dependencies == 0 {
            result.task.set_state(state::TaskState::Ready)
        };
        result
    }

    pub fn run(&mut self, sched : &mut super::Scheduler){
        self.task.run(sched);
    }

    pub fn register(&mut self, obs: Box <dyn observers::Observer>){
        self.task.register(obs);
    }

    pub fn notify_from_dependency(&mut self, state_from:&state::TaskState){
        match state_from{
            state::TaskState::Done => {
                self.number_of_dependencies -= 1;
                if self.number_of_dependencies == 0{
                    self.task.set_state(state::TaskState::Ready);
                }
            },
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Scheduler, TaskInfo};

    use super::*;
    #[test]
    fn test_state()
    {
        let mut sched = Scheduler::new();
        let t1 = task_declaration::SimpleTask::new(Box::new(|| println!("T1!")));
        let t1 = task_declaration::TaskDeclaration::Simple(t1);
        let t2 = task_declaration::SimpleTask::new(Box::new(|| println!("T2!")));
        let t2 = task_declaration::TaskDeclaration::Simple(t2);
        let sk_t1 = ScheduledTask::new(t1, &[]);
        let sk_t1 = std::rc::Rc::new(std::cell::RefCell::new(sk_t1));
        let t_info = TaskInfo::new(sk_t1.clone());
        let sk_t2 = ScheduledTask::new(t2, &[t_info]);
        let sk_t2 = std::rc::Rc::new(std::cell::RefCell::new(sk_t2));
        let dep_obs = Box::new(observers::DependencyObserver::new(sk_t2.clone()));
        sk_t1.borrow_mut().register(dep_obs);
        println!("nb dep:{}", sk_t2.borrow().number_of_dependencies);
        sk_t1.borrow_mut().run(&mut sched);
        println!("nb dep:{}", sk_t2.borrow().number_of_dependencies);

    }
}