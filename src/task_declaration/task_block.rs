use super::TaskDeclaration;
use super::TaskInfo;
use crate::scheduler::Scheduler;

pub struct TaskBlock{
    deps : Vec<TaskInfo>,
    contents : Vec<Box<dyn TaskDeclaration>>,
    manager : SchedulerWeakRef,
}

impl TaskBlock{
    pub fn new(deps : Vec<TaskInfo>, manager : SchedulerRef) -> TaskBlock{
        TaskBlock{
            deps,
            contents : vec![],
            manager : std::rc::Rc::downgrade(manager)}
    }

    pub fn add(&mut self, task:Box<dyn TaskDeclaration>){
        self.contents.push(task);
    }
}

impl TaskDeclaration for TaskBlock{
    fn run(&mut self) {
        todo!()
    }

    fn dependencies(&self) -> &Vec<TaskInfo> {
        &self.deps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_declaration::SimpleTask;

    #[test]
    fn it_works() {
        let mut man = Scheduler::new();
        let mut tb = TaskBlock::new(vec![], &man);
        let t1 = SimpleTask::new(&[], Box::new(|| println!("hehe!")));
        tb.add(Box::new(t1));
        // man.submit(Box::new(tb));
    }
}
