mod generaltask;
mod scheduler;

pub use scheduler::Scheduler;
pub use generaltask::SimpleTask;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let t1 = SimpleTask::new(&[], Box::new(|| println!("hehe!")));
        let mut sched = Scheduler::new();
        sched.submit(Box::new(t1));
        sched.start();

    }
}
