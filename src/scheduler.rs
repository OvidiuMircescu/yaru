
use crate::schedtask;

pub type TaskId = usize;
//pub type TaskRef = std::rc::Rc<Box<dyn schedtask::SchedTask>>;
struct WorkOnTask{
    task : Box<dyn schedtask::SchedTask>,
    id : TaskId,
    waiting_dependencies : Vec<WorkOnTaskRef>,
    state : TaskState
}

type WorkOnTaskRef = std::rc::Rc<std::cell::RefCell<WorkOnTask>>;
enum TaskState{
    queued,
    running,
    ready,
    error,
    undefined
}

impl WorkOnTask{
    pub fn filter_dependencies(&mut self){
        self.waiting_dependencies.retain(|x|
            if let TaskState::ready = x.borrow().state {
                false
            } else {
                true
            });
    }

    pub fn is_ready(&self)->bool{
        self.waiting_dependencies.is_empty()
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
        let new_task = WorkOnTask{
            task,
            id,
            waiting_dependencies,
            state : TaskState::queued
        };
        let new_task = std::rc::Rc::new(std::cell::RefCell::new(new_task));
        self.all_tasks.insert(id, new_task.clone());
        self.waiting_tasks.push(new_task.clone());
        id
    }

    pub fn start(&mut self){
        for e in self.waiting_tasks.iter(){
            e.borrow_mut().filter_dependencies();
        }
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
                task.borrow_mut().state = TaskState::ready;
            }
            self.waiting_tasks.retain(|x|
                if let TaskState::ready = x.borrow().state {
                    false
                } else {
                    true
                });
            for e in self.waiting_tasks.iter(){
                e.borrow_mut().filter_dependencies();
            }
        }
    }
}