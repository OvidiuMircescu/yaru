mod task_declaration;
mod scheduler;

pub use scheduler::Scheduler;
pub use task_declaration::TaskDeclaration;
pub use task_declaration::SimpleTask;
pub use scheduler::TaskInfo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let t1 = SimpleTask::new(Box::new(|| println!("hehe!")));
        let mut sched = Scheduler::new();
        sched.submit(TaskDeclaration::Simple(t1), &[]);
        sched.start();

    }
}
