use std::rc::Rc;
use std::cell::RefCell;

fn newtask(message:&str, deps:&[usize], clown:&Rc<RefCell<String>>, sched:&mut newsched::Scheduler ) ->usize{
    let clown = clown.clone();
    let message = String::from(message);
    let func = move || clown.borrow_mut().push_str(&message);
    let task = newsched::SimpleTask::new(&deps, Box::new(func));
    sched.submit(Box::new(task))
}

#[test]
fn one_task(){
    let result = Rc::new(RefCell::new(String::new()));
    let mut sched = newsched::Scheduler::new();
    newtask("Done!\n", &[], &result, &mut sched);
    sched.start();

    // assert_eq!(*RefCell::borrow(&result), "Done!\n");
    assert_eq!(*result.borrow(), "Done!\n");
}

#[test]
fn many_independent_tasks(){
    let result = Rc::new(RefCell::new(String::new()));
    let mut sched = newsched::Scheduler::new();

    let mut expected = String::new();
    for i in 1 .. 20{
            let message = format!("task {}\n", i);
            expected.push_str(&message);
            newtask(&message, &[], &result, &mut sched);
        }

    sched.start();
    assert_eq!(*result.borrow(), expected);
}

#[test]
fn one_depends_on_many(){
    let result = Rc::new(RefCell::new(String::new()));
    let mut sched = newsched::Scheduler::new();

    let mut expected = String::new();
    let mut deps = Vec::new();
    for i in 1 .. 20{
            let message = format!("task {}\n", i);
            expected.push_str(&message);
            deps.push(newtask(&message, &[], &result, &mut sched));
        }
    let message = format!("final task\n");
    expected.push_str(&message);
    newtask(&message, &deps, &result, &mut sched);
    sched.start();
    assert_eq!(*result.borrow(), expected);
    // println!("{}", *result.borrow());
}

#[test]
fn chain(){
    let result = Rc::new(RefCell::new(String::new()));
    let mut sched = newsched::Scheduler::new();

    let mut expected = String::from("start task\n");
    let mut idx = newtask(&expected, &[], &result, &mut sched);
    let start_task_id = idx;
    let mut deps = Vec::new();

    // first set of tasks depending on start
    for _ in 1..=3{
        let message = format!("first set task {}\n", idx);
        expected.push_str(&message);
        idx = newtask(&message, &[start_task_id], &result, &mut sched);
        deps.push(idx);
    }

    // middle set of tasks depending on first set
    let mut middleset = String::new();
    for _ in 1..=3{
        let message = format!("middle set task {}\n", idx);
        middleset.push_str(&message);
        idx = newtask(&message, &deps, &result, &mut sched);
    }

    // second of tasks set depending on start
    for _ in 1..=3{
        let message = format!("second set task {}\n", idx);
        expected.push_str(&message);
        idx = newtask(&message, &[start_task_id], &result, &mut sched);
    }

    sched.start();

    expected.push_str(&middleset);
    assert_eq!(*result.borrow(), expected);
}

