pub trait SchedTask{
    fn run(&mut self);

    fn dependencies(&self) -> &Vec<usize>;
}

pub struct SimpleTask <T>
where
  T : Fn()
{
    deps : Vec<usize>,
    callback : T
}

impl<T> SimpleTask<T>
where
  T : Fn()
{
    pub fn new(deps: &[usize], callback : T) -> Self {
        SimpleTask {deps : deps.to_owned(), callback}
    }
}

impl <T> SchedTask for SimpleTask<T>
where
  T : Fn()
{
    fn run(&mut self){
        (self.callback)();
    }

    fn dependencies(&self) -> &Vec<usize> {
        &self.deps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut t1 = SimpleTask::new(&[], || println!("hehe!"));
        t1.run();
    }
}
