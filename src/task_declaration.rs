pub trait TaskDeclaration{
    fn run(& self);
}

type CallbackFunc = Box<dyn Fn()>;
pub struct SimpleTask
{
    callback : CallbackFunc
}

impl SimpleTask
{
    pub fn new(callback : CallbackFunc) -> Self {
        SimpleTask {callback}
    }
}

impl TaskDeclaration for SimpleTask
{
    fn run(& self){
        (self.callback)();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let t1 = SimpleTask::new(Box::new(|| println!("hehe!")));
        t1.run();
    }
}
