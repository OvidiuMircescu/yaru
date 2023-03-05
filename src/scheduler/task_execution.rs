use super::state::TaskState;
pub trait ExecutionObserver{
    fn notify_from_execution(&mut self, state : TaskState);
}

pub trait TaskExecution{
  fn run(&mut self);
}

pub struct SimpleTaskExecution{
}

pub struct BlocExecution{

}

impl TaskExecution for SimpleTaskExecution{
    fn run(&mut self) {
        println!("Running simple task");
    }
}

impl TaskExecution for BlocExecution{
    fn run(&mut self) {
        println!("Running block");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::scheduler::TaskState;
    pub struct TestScheduledTask{
        task : Box<dyn TaskExecution>,
        state : isize
    }

    impl TestScheduledTask{
        fn run(&mut self){
            self.task.run();
        }

        pub fn another(&mut self){
            self.state += 1;
        }
    }
    impl ExecutionObserver for TestScheduledTask{
        fn notify_from_execution(&mut self, execution_state:TaskState){
            println!("Received {execution_state}");
        }
    }

    #[test]
    fn test_build(){
        let ts = SimpleTaskExecution{};
        let mut tst = TestScheduledTask{
            task : Box::new(ts),
            state : 42
        };
        tst.run();
        tst.another();
    }
}
