use std::rc::Rc;
use std::cell::RefCell;

fn newtask(message:&str, deps:&[newsched::TaskInfo], clown:&Rc<RefCell<String>>, sched:&mut newsched::Scheduler ) ->newsched::TaskInfo{
    let clown = clown.clone();
    let message = String::from(message);
    let func = move || clown.borrow_mut().push_str(&message);
    let task = newsched::SimpleTask::new(Box::new(func));
    sched.submit(newsched::TaskDeclaration::Simple(task), deps)
}

fn newtask_vec(message:&str, deps:&[newsched::TaskInfo], clown:&Rc<RefCell<Vec<String>>>, sched:&mut newsched::Scheduler ) ->newsched::TaskInfo{
    let clown = clown.clone();
    let message = String::from(message);
    let func = move || clown.borrow_mut().push(String::from(&message));
    let task = newsched::SimpleTask::new(Box::new(func));
    sched.submit(newsched::TaskDeclaration::Simple(task), deps)
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
    let result = Rc::new(RefCell::new(Vec::new()));
    let mut sched = newsched::Scheduler::new();

    let start = String::from("start task\n");
    let start_task_id = newtask_vec(&start, &[], &result, &mut sched);
    let mut deps = Vec::new();

    // first set of tasks depending on start
    let mut exp_set1 = Vec::new();
    for it in 1..=3{
        let message = format!("first set task {}\n", it);
        exp_set1.push(message.clone());
        let info = newtask_vec(&message, &[start_task_id.clone()], &result, &mut sched);
        deps.push(info);
    }

    // middle set of tasks depending on first set
    let mut middleset = Vec::new();
    for it in 4..=6{
        let message = format!("depends on first set task {}\n", it);
        middleset.push(message.clone());
        newtask_vec(&message, &deps, &result, &mut sched);
    }

    // second of tasks set depending on start
    let mut exp_set2 = Vec::new();
    for it in 7..=9{
        let message = format!("second set task {}\n", it);
        exp_set2.push(message.clone());
        newtask_vec(&message, &[start_task_id.clone()], &result, &mut sched);
    }

    sched.start();

    assert_eq!(result.borrow().get(0), Some(&start));
    let mut maxset1 = 0usize;
    for t in &exp_set1{
        assert!(result.borrow().contains(t));
        let idx = result.borrow().iter().position(|x| x == t).unwrap();
        if idx > maxset1 {
            maxset1 = idx;
        }
    }
    for t in &exp_set2{
        assert!(result.borrow().contains(t));
    }
    for t in &middleset{
        assert!(result.borrow().contains(t));
        let idx = result.borrow().iter().position(|x| x == t).unwrap();
        assert!(idx > maxset1);
    }
}

#[test]
fn bloc_after_bloc(){
    let result = Rc::new(RefCell::new(Vec::new()));
    let mut sched = newsched::Scheduler::new();
    let mut bloc1: Vec<newsched::TaskDeclaration> = Vec::new();
    let mut res1 = Vec::new();

    for it in 1..=5{
        let message = format!("B1 T{}", it);
        res1.push(String::from(&message));
        let clown = result.clone();
        let func = move || clown.borrow_mut().push(String::from(&message));
        let task = newsched::SimpleTask::new(Box::new(func));
        bloc1.push(newsched::TaskDeclaration::Simple(task));
    }

    let mut bloc2: Vec<newsched::TaskDeclaration> = Vec::new();
    let mut res2 = Vec::new();
    for it in 6..=10{
        let message = format!("B2 T{}", it);
        res2.push(String::from(&message));
        let clown = result.clone();
        let func = move || clown.borrow_mut().push(String::from(&message));
        let task = newsched::SimpleTask::new(Box::new(func));
        bloc2.push(newsched::TaskDeclaration::Simple(task));
    }

    let info = sched.submit(newsched::TaskDeclaration::Bloc(bloc1), &[]);
    sched.submit(newsched::TaskDeclaration::Bloc(bloc2), &[info]);
    sched.start();

    for t in &res1{
        assert!(result.borrow().contains(t));
        let idx = result.borrow().iter().position(|x| x == t).unwrap();
        assert!(idx < 5);
    }

    for t in &res2{
        assert!(result.borrow().contains(t));
        let idx = result.borrow().iter().position(|x| x == t).unwrap();
        assert!(idx >= 5);
    }

}