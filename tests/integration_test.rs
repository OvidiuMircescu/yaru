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

    let mut expected = Vec::new();
    for i in 1 .. 20{
            let message = format!("task {}\n", i);
            expected.push(message.clone());
            newtask(&message, &[], &result, &mut sched);
        }

    sched.start();
    for val in expected{
        assert!(result.borrow().contains(&val));
    }
}

#[test]
fn one_depends_on_many(){
    let result = Rc::new(RefCell::new(String::new()));
    let mut sched = newsched::Scheduler::new();

    let mut expected = Vec::new();
    let mut deps = Vec::new();
    for i in 1 .. 20{
            let message = format!("task {}\n", i);
            expected.push(message.clone());
            deps.push(newtask(&message, &[], &result, &mut sched));
        }
    let final_message = format!("final task\n");
    newtask(&final_message, &deps, &result, &mut sched);
    sched.start();
    for val in expected{
        assert!(result.borrow().contains(&val));
    }
    assert!(result.borrow().ends_with(&final_message));
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
        let message = format!("depends on first set task {}\n", idx);
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
    println!("{}", *result.borrow());
    // TODO test
    // assert_eq!(*result.borrow(), expected);
}

