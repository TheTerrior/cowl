use smartlist::SmartList;

mod memory;
mod smartlist;

fn test_smartlist() {
    let mut sl: SmartList<i32> = SmartList::new();
    println!("{:?}", sl.add(4));
    println!("{:?}", sl.add(7));
    println!("{:?}", sl.add(8));
    let mut status = sl.remove(1);
    println!("{:?}", status);
    println!("{:?}", &sl.data);
    println!("{:?}", sl.add(10));
    println!("{:?}", sl.add(17));
    println!("{:?}", &sl.data);
    status = sl.remove(1);
    println!("{:?}", status);
    status = sl.remove(0);
    println!("{:?}", status);
    status = sl.remove(2);
    println!("{:?}", status);
    status = sl.remove(3);
    println!("{:?}", status);
    status = sl.remove(3);
    println!("{:?}", status);
    status = sl.remove(4);
    println!("{:?}", status);
    println!("{:?}", &sl.data);
}

fn main() {
    println!("Hello, world!");
    test_smartlist();
}
