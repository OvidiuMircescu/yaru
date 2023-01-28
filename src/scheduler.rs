use crate::schedtask;
enum TaskState{
    Waiting,
    Ready,
    Done
}

pub type TaskId = usize;
struct WorkOnTask{
    task : Box<dyn schedtask::SchedTask>,
    // id : TaskId,
    number_of_dependencies : usize,
    observers : Vec<WorkOnTaskRef>,
    state : TaskState
}

type WorkOnTaskRef = std::rc::Rc<std::cell::RefCell<WorkOnTask>>;
impl WorkOnTask{
    pub fn new(task : Box<dyn schedtask::SchedTask>)-> WorkOnTask{
        let number_of_dependencies = task.dependencies().len();
        WorkOnTask{
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

    pub fn set_done(&mut self){
        self.state = TaskState::Done;
        for obs in &self.observers{
            obs.borrow_mut().notify_from(&self);
        }
    }

    pub fn register(&mut self, task:&WorkOnTaskRef){
        self.observers.push(task.clone());
        task.borrow_mut().notify_from(&self);
    }

    pub fn notify_from(&mut self, from:&WorkOnTask){
        if from.is_done(){
            self.number_of_dependencies -= 1;
            if self.number_of_dependencies == 0{
                self.state = TaskState::Ready;
            }
        }
    }
}


pub struct Scheduler{
    all_tasks : std::collections::HashMap<TaskId, WorkOnTaskRef>,
    last_id : TaskId,
}

impl Scheduler{
    pub fn new()-> Scheduler{
        Scheduler{
            all_tasks : std::collections::HashMap::new(),
            last_id : 0,
        }
    }

    pub fn submit(&mut self, task : Box<dyn schedtask::SchedTask>) -> TaskId{
        let id = self.last_id;
        self.last_id += 1;
        let dependencies: Vec<WorkOnTaskRef> = task.dependencies()
            .iter()
            .filter_map(|x|self.all_tasks.get(x))
            .map(|x| x.clone())
            .collect();
        let new_task =WorkOnTask::new(task) ;
        let new_task = std::rc::Rc::new(std::cell::RefCell::new(new_task));
        self.all_tasks.insert(id, new_task.clone());
        for task in dependencies.iter(){
            task.borrow_mut().register(& new_task);
        }
        id
    }

    pub fn start(&mut self){
        let mut waiting_tasks: Vec<WorkOnTaskRef>  = self.all_tasks.values()
                    .filter(|x| !x.borrow().is_done())
                    .cloned()
                    .collect();
        while !waiting_tasks.is_empty() {
            let ready_tasks : Vec<WorkOnTaskRef> = waiting_tasks.iter()
                    .filter(|x| x.borrow().is_ready())
                    .cloned()
                    .collect();
            if ready_tasks.is_empty(){
                println!("Merde!");
                break
            }
            for task in ready_tasks{
                task.borrow_mut().task.run();
                task.borrow_mut().set_done();
                waiting_tasks.retain(|x| !std::rc::Rc::ptr_eq(x,&task));
             }
        }
    }
}