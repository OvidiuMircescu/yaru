use crate::generaltask;

mod scheduledtask;
use scheduledtask::*;

pub type TaskId = usize;

pub struct Scheduler{
    all_tasks : std::collections::HashMap<TaskId, ScheduledTaskRef>,
    last_id : TaskId,
}

impl Scheduler{
    pub fn new()-> Scheduler{
        Scheduler{
            all_tasks : std::collections::HashMap::new(),
            last_id : 0,
        }
    }

    pub fn submit(&mut self, task : Box<dyn generaltask::GeneralTask>) -> TaskId{
        let id = self.last_id;
        self.last_id += 1;
        let dependencies: Vec<ScheduledTaskRef> = task.dependencies()
            .iter()
            .filter_map(|x|self.all_tasks.get(x))
            .map(|x| x.clone())
            .collect();
        let new_task =ScheduledTask::new(task) ;
        let new_task = std::rc::Rc::new(std::cell::RefCell::new(new_task));
        self.all_tasks.insert(id, new_task.clone());
        for task in dependencies.iter(){
            task.borrow_mut().register(& new_task);
        }
        id
    }

    pub fn start(&mut self){
        let mut waiting_tasks: Vec<ScheduledTaskRef>  = self.all_tasks.values()
                    .filter(|x| !x.borrow().is_done())
                    .cloned()
                    .collect();
        while !waiting_tasks.is_empty() {
            let ready_tasks : Vec<ScheduledTaskRef> = waiting_tasks.iter()
                    .filter(|x| x.borrow().is_ready())
                    .cloned()
                    .collect();
            if ready_tasks.is_empty(){
                println!("Merde!");
                break
            }
            for task in ready_tasks{
                task.borrow_mut().run();
                waiting_tasks.retain(|x| !std::rc::Rc::ptr_eq(x,&task));
            }
        }
    }
}