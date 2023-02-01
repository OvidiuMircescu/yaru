use crate::generaltask;

pub enum TaskState{
    Waiting,
    Ready,
    Running,
    Done
}

pub struct ScheduledTask{
    task : Box<dyn generaltask::GeneralTask>,
    // id : TaskId,
    number_of_dependencies : usize,
    observers : Vec<ScheduledTaskRef>,
    state : TaskState
}

pub type ScheduledTaskRef = std::rc::Rc<std::cell::RefCell<ScheduledTask>>;
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

    pub fn is_ready(&self)->bool{
        matches!(self.state, TaskState::Ready)
    }

    pub fn is_done(&self)->bool{
        matches!(self.state, TaskState::Done)
    }

    pub fn run(&mut self){
        self.state = TaskState::Running;
        self.task.run();
        self.state = TaskState::Done;
        for obs in &self.observers{
            obs.borrow_mut().notify(&self);
        }
    }

    pub fn register(&mut self, task:&ScheduledTaskRef){
        self.observers.push(task.clone());
        task.borrow_mut().notify(&self);
    }

    pub fn notify(&mut self, from:&ScheduledTask){
        if from.is_done(){
            self.number_of_dependencies -= 1;
            if self.number_of_dependencies == 0{
                self.state = TaskState::Ready;
            }
        }
    }
}
