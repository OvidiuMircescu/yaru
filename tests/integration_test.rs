
#[test]
fn t1(){
    let t1 = newsched::SimpleTask::new(&[], Box::new(|| println!("hehe!")));
    let mut sched = newsched::Scheduler::new();
    sched.submit(Box::new(t1));
    sched.start();
}