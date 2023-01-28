use crate::schedtask;
use std::rc::Rc;
use std::cell::RefCell;
enum TaskState{
    Waiting,
    Ready,
    Done
}

pub type TaskId = usize;
//pub type TaskRef = std::rc::Rc<Box<dyn schedtask::SchedTask>>;
struct WorkOnTask{
    task : Box<dyn schedtask::SchedTask>,
    // id : TaskId,
    waiting_dependencies : Vec<WorkOnTaskRef>,
    observers : Vec<WorkOnTaskRef>,
    state : TaskState
}

type WorkOnTaskRef = std::rc::Rc<std::cell::RefCell<WorkOnTask>>;
impl WorkOnTask{
    pub fn new(task : Box<dyn schedtask::SchedTask>,
        waiting_dependencies: Vec<Rc<RefCell<WorkOnTask>>>)-> WorkOnTask{
            WorkOnTask{
                task,
                // id,
                waiting_dependencies,
                observers : Vec::new(),
                state : TaskState::Waiting
            }
    }

    pub fn filter_dependencies(&mut self){
        self.waiting_dependencies.retain(|x| !x.borrow().is_done());
        if self.waiting_dependencies.is_empty(){
            self.state = TaskState::Ready;
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
    }

    pub fn register(&mut self, task:&WorkOnTaskRef){
        self.observers.push(task.clone());
    }

    pub fn notify_from(&mut self, from:&WorkOnTaskRef){
        if from.borrow().is_done(){
            // self.waiting_dependencies.retain(|x| x.borrow().id != from.id);
            self.waiting_dependencies.retain(|x| !std::rc::Rc::ptr_eq(x, from));
            if self.waiting_dependencies.is_empty(){
                self.state = TaskState::Ready;
            }
        }
    }
}


pub struct Scheduler{
    all_tasks : std::collections::HashMap<TaskId, WorkOnTaskRef>,
    last_id : TaskId,
    waiting_tasks : Vec<WorkOnTaskRef>,
}

impl Scheduler{
    pub fn new()-> Scheduler{
        Scheduler{
            all_tasks : std::collections::HashMap::new(),
            last_id : 0,
            waiting_tasks : Vec::new(),
        }
    }

    pub fn submit(&mut self, task : Box<dyn schedtask::SchedTask>) -> TaskId{
        let id = self.last_id;
        self.last_id += 1;
        let waiting_dependencies = task.dependencies()
            .iter()
            .filter_map(|x|self.all_tasks.get(x))
            .map(|x| x.clone())
            .collect();
/*         let new_task = WorkOnTask{
            task,
            // id,
            waiting_dependencies,
            observers : Vec::new(),
            state : TaskState::Waiting
        };
 */
        let new_task =WorkOnTask::new(task, waiting_dependencies) ;
        let new_task = std::rc::Rc::new(std::cell::RefCell::new(new_task));
        self.all_tasks.insert(id, new_task.clone());
        self.waiting_tasks.push(new_task.clone());
        new_task.borrow_mut().filter_dependencies();
        for task in new_task.borrow().waiting_dependencies.iter(){
            task.borrow_mut().register(&new_task);
        }
        id
    }

    pub fn start(&mut self){
        while !self.waiting_tasks.is_empty() {
            let ready_tasks : Vec<WorkOnTaskRef> = self.waiting_tasks
                    .iter()
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
                self.waiting_tasks.retain(|x| !std::rc::Rc::ptr_eq(x,&task));
                for obs in task.borrow().observers.iter(){
                    obs.borrow_mut().notify_from(&task);
                }
            }
        }
    }
}