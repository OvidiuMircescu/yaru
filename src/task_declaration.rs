use crate::scheduler::TaskInfo;
pub trait TaskDeclaration{
    fn run(&mut self);

    fn dependencies(&self) -> &Vec<TaskInfo>;
}

type CallbackFunc = Box<dyn Fn()>;
pub struct SimpleTask
{
    deps : Vec<TaskInfo>,
    callback : CallbackFunc
}

impl SimpleTask
{
    pub fn new(deps: &[TaskInfo], callback : CallbackFunc) -> Self {
        SimpleTask {deps : deps.to_vec(), callback}
    }
}

impl TaskDeclaration for SimpleTask
{
    fn run(&mut self){
        (self.callback)();
    }

    fn dependencies(&self) -> &Vec<TaskInfo> {
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
