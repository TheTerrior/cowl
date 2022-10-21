#[derive(Clone, Copy, Debug)]
pub struct TestStruct {
    pub a: i32,
    pub b: usize,
}

pub enum TestEnum {
    One(i32),
    Two(f64),
    Three(i64),
}

/*
abstract class Variable {
    // -- Snip --
}

class Num extends Variable {
    int n;

    int get_num(){
        return this.n;
    }
    // -- Snip --

}

class NumFloat extends Variable {
    float n;

    int get_num(){
        return this.n;
    }
    // -- Snip --

}

class String2 extends Variable {
    String s;

    int get_string(){
        return this.n.round();
    }
}


class Main {
    ... main(..){
        // List<GetNum> nums = new ArrayList<> ();


        // nums.add(new Num(10));
        // nums.add(new NumFloat(PI));

        // nums.get(0).get(); // 10
        // nums.get(1).get(); // 3 (rounded PI)

        // nums.add

        // let (ptr,vtable) = nums.get(0);
        // let f = vtable.get("get"); // fn pointer
        // f(ptr)

        List<Variable> vars = new ArrayList<> ();
        vars.add(new Num(10));
        vars.add(new String2("abc"));

        let num = (Num)vars.get(0);
        let string2 = (String2)vars.get(1);
    }
}
*/

pub struct TestVTable {
    // ...

}

trait TestTrait { 
    fn get(&self) -> i64;
}

struct TestStruct1 {
    x: i32,
}

impl TestTrait for TestStruct1 {
    fn get(&self) -> i64 {
        self.x as _
    }
}

struct TestStruct2 {
    x: i64,
}

impl TestTrait for TestStruct2 {
    fn get(&self) -> i64 {
        self.x as _
    }
}




fn run_experiments() {
    let mut z = TestStruct{
        a: 3,
        b: 3,
    };

    let a = Rc::new(RefCell::new(vec![1, 2, 3, 4]));
    let b = Rc::clone(&a);
    let c = Rc::clone(&b);

    a.borrow_mut()[0] = 10;
    let x = a.borrow_mut()[2] = 12;

    println!("{:?}", a.borrow());

    println!("{}", core::mem::size_of::<TestEnum>());

    print!("\n");

    println!("box {}", core::mem::size_of::<Box<i32>>());
    println!("usize {}", core::mem::size_of::<usize>());
    println!("rc refcell variable {}", core::mem::size_of::<Rc<RefCell<Variable>>>());
    println!("variable {}", core::mem::size_of::<Variable>());
    println!("vec of rcs {}", core::mem::size_of::<Vec<Rc<RefCell<Variable>>>>());
    println!("vec of u8 {}", core::mem::size_of::<Vec<u8>>());
    println!("hm {}", core::mem::size_of::<HashMap<usize, usize>>());
    println!("hs {}", core::mem::size_of::<HashSet<usize>>());
    println!("e i32 {}", core::mem::size_of::<TestStruct1>());
    println!("e i64 {}", core::mem::size_of::<TestStruct2>());

    let n = TestStruct1 {
        x: 2,
    };
    let m = TestStruct2 {
        x: 2,
    };
    

    let list_traits: Vec<Box<dyn TestTrait>> = vec![
        Box::new(n),
        Box::new(m),
    ];

    let o: & Box<dyn TestTrait> = & list_traits[0];

    let gotten = o.get();

    // let list_traits: Vec<Box<dyn TestTrait>> = vec![];

}



