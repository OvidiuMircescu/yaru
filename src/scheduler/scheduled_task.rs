use crate::task_declaration;

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
pub struct ScheduledTask{
    task : Box<dyn task_declaration::TaskDeclaration>,
    // id : TaskId,
    number_of_dependencies : usize,
    observers : Vec<Box <dyn Observer>>,
    state : TaskState
}

impl ScheduledTask{
    pub fn new(task : Box<dyn task_declaration::TaskDeclaration>)-> ScheduledTask{
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

    pub fn notify_from_dependency(&mut self, state_from:&TaskState){
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