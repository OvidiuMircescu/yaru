use crate::scheduler::TaskId;
pub trait SchedTask{
    fn run(&mut self);

    fn dependencies(&self) -> &Vec<TaskId>;
}

type CallbackFunc = Box<dyn Fn()>;
pub struct SimpleTask
{
    deps : Vec<TaskId>,
    callback : CallbackFunc
}

impl SimpleTask
{
    pub fn new(deps: &[TaskId], callback : CallbackFunc) -> Self {
        SimpleTask {deps : deps.to_owned(), callback}
    }
}

impl SchedTask for SimpleTask
{
    fn run(&mut self){
        (self.callback)();
    }

    fn dependencies(&self) -> &Vec<TaskId> {
        &self.deps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut t1 = SimpleTask::new(&[], Box::new(|| println!("hehe!")));
        t1.run();
    }
}
