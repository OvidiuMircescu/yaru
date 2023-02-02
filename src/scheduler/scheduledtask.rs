use crate::generaltask;

pub enum TaskState{
    Waiting,
    Ready,
    Running,
    Done
}

pub trait Observer {
    fn notify(&mut self, from:&TaskState);
}


pub type ScheduledTaskRef = std::rc::Rc<std::cell::RefCell<ScheduledTask>>;
pub struct DependencyObserver{
    observer : ScheduledTaskRef,
    // subject : ScheduledTaskRef
}

impl DependencyObserver{
    pub fn new(observer: ScheduledTaskRef,
        //    subject : ScheduledTaskRef
          )->DependencyObserver{
        DependencyObserver {
            observer,
            //  subject
        }
    }
}

impl Observer for DependencyObserver{
    fn notify(&mut self, from:&TaskState) {
        self.observer.borrow_mut().notify_from_dependency(from)
    }
}

pub struct ScheduledTask{
    task : Box<dyn generaltask::GeneralTask>,
    // id : TaskId,
    number_of_dependencies : usize,
    observers : Vec<Box <dyn Observer>>,
    state : TaskState
}

impl ScheduledTask{
    pub fn new(task : Box<dyn generaltask::GeneralTask>)-> ScheduledTask{
        let number_of_dependencies = task.dependencies().len();
        ScheduledTask{
            task,
            // id,
            number_of_dependencies,
            observers : Vec::new(),
            state : if number_of_dependencies == 0 {TaskState::Ready}
                    else {TaskState::Waiting}
        }
    }

/*    pub fn is_ready(&self)->bool{
        matches!(self.state, TaskState::Ready)
    }

    pub fn is_done(&self)->bool{
        matches!(self.state, TaskState::Done)
    }

     pub fn get_state(&self)-> &TaskState{
        &self.state
    }
 */
    pub fn run(&mut self){
        self.state = TaskState::Running;
        self.task.run();
        self.state = TaskState::Done;
        self.notify_observers();
    }

    pub fn register(&mut self, mut obs: Box <dyn Observer>){
        obs.notify(&self.state);
        self.observers.push(obs);
    }

    fn notify_observers(&mut self){
        for obs in &mut self.observers{
            obs.notify(&self.state);
        }
    }

    fn notify_from_dependency(&mut self, state_from:&TaskState){
        match state_from{
            TaskState::Done => {
                self.number_of_dependencies -= 1;
                if self.number_of_dependencies == 0{
                    self.state = TaskState::Ready;
                    self.notify_observers();
                }
            },
            _ => ()
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