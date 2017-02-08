use std::thread;
use std::ops::Range;

#[derive(Debug)]
struct Regs {
    r0: u16,
    r1: u16,
    r7: u16,
}

fn algorithm(regs: &mut Regs) {
    if regs.r0 == 0 {
        regs.r0 = regs.r1 + 1;
        return;
    } else if regs.r1 == 0 {
        regs.r0 = regs.r0 - 1;
        regs.r1 = regs.r7;

        algorithm(regs);
        return;
    } else {
        let tmp0 = regs.r0;
        regs.r1 = regs.r1 - 1;

        algorithm(regs);

        regs.r1 = regs.r0;
        regs.r0 = tmp0;
        regs.r0 = regs.r0 - 1;

        algorithm(regs);

        return;
    }
}

fn thread_body(range: Range<u16>) -> Vec<(u16, Regs)> {
    range.map(|r7| {
        let mut regs = Regs {
            r0: 4,
            r1: 1,
            r7: r7,
        };

        algorithm(&mut regs);

        (r7, regs)
    }).collect()
}

fn main() {
    let thread1 = thread::Builder::new()
        .name("bf-1".to_owned())
        .spawn(move || {
            thread_body(1..4682)
        }).unwrap();

    
    let thread2 = thread::Builder::new()
        .name("bf-2".to_owned())
        .spawn(move || {
            thread_body(4682..9363)
        }).unwrap();

    
    let thread3 = thread::Builder::new()
        .name("bf-3".to_owned())
        .spawn(move || {
            thread_body(9363..14044)
        }).unwrap();

    
    let thread4 = thread::Builder::new()
        .name("bf-4".to_owned())
        .spawn(move || {
            thread_body(14044..18725)
        }).unwrap();

    
    let thread5 = thread::Builder::new()
        .name("bf-5".to_owned())
        .spawn(move || {
            thread_body(18725..23406)
        }).unwrap();

    
    let thread6 = thread::Builder::new()
        .name("bf-6".to_owned())
        .spawn(move || {
            thread_body(23406..28087)
        }).unwrap();

    
    let thread7 = thread::Builder::new()
        .name("bf-7".to_owned())
        .spawn(move || {
            thread_body(28087..32767)
        }).unwrap();


    let vec1 = thread1.join().unwrap();
    let vec2 = thread2.join().unwrap();
    let vec3 = thread3.join().unwrap();
    let vec4 = thread4.join().unwrap();
    let vec5 = thread5.join().unwrap();
    let vec6 = thread6.join().unwrap();
    let vec7 = thread7.join().unwrap();


    for (r7, regs) in vec1 {
        println!("{: >5}: r0 = {: >5}", r7, regs.r0);
    }
    for (r7, regs) in vec2 {
        println!("{: >5}: r0 = {: >5}", r7, regs.r0);
    }
    for (r7, regs) in vec3 {
        println!("{: >5}: r0 = {: >5}", r7, regs.r0);
    }
    for (r7, regs) in vec4 {
        println!("{: >5}: r0 = {: >5}", r7, regs.r0);
    }
    for (r7, regs) in vec5 {
        println!("{: >5}: r0 = {: >5}", r7, regs.r0);
    }
    for (r7, regs) in vec6 {
        println!("{: >5}: r0 = {: >5}", r7, regs.r0);
    }
    for (r7, regs) in vec7 {
        println!("{: >5}: r0 = {: >5}", r7, regs.r0);
    }
    
}
