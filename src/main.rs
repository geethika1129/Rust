fn funct(a:i32,b:i32,c:i32)->i32{
    return a*b*c;
}

struct Person{
    name:String,
    age:u8,
}

fn describe(person:&Person){
    println!("{} is {} years old",person.name,person.age);

}

mod mm{
    pub fn doo()
    {
        println!("module called");
    }
}

struct Point(i32,i32);

#[derive(Debug)]
enum Direction{
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport {x : u32, y: u32 }, // Struct variant
}


fn main() {
    println!("Hello, world!");
    let x=32;
    println!("x:{x}");

    let y=funct(10,10,30);
    println!("result:{}",y);

    let aa="ab";
    let bb="cd";
    let mut ss=String :: new();
    ss.push_str(aa);
    ss.push_str(bb);
    println!("ss is:{}",ss);
    println!("{:?}",&ss[0..3]);
    let x=10;

    // conditionals
    let s=if x<20 {"small"} else {"large"};
    println!("the size is {}",s);

    //loops
    //while loop
    let mut x=1000;
    while x>=10{
        x=x/2;

    }
    println!("value after while loop is {}",x);

    //for loop
    for x in 1..5{//5 not included
        println!("x is {x}");
    }

    //loop statement --used to loop forever
    let mut i=0;
    loop {
        i+=1;
        println!("{i}");
        if i>100{
            break;
        }
    }

    let mut a=[42,10,20,20,40];
    a[4]=0;
    println!("a:{a:?}");

    let t=(7,true);
    println!("t.0:{}",t.0);
    println!("t.1:{}",t.1);

    match a{
        [x,y,z,a,b]=>println!("y is {y}"),
        _=>println!("all elements are ignored"),
    }
    let k='A';
    let mut r=&k;
    println!("r:{}",*r);

    //values which cannot bee changed
    //exclusive references

    let mut pt=(1,2);
    let xx=&mut pt.0;
    *xx=20;
    println!("point is :{pt:?}");


    let mut peter=Person{
        name:String::from("Peter"),age:20
        
    };
    describe(&peter);
//  to copy few fields from the available structure just use the .... operator

//tuple structs

//todo!("implement the struct");

//enums
let m: PlayerMove = PlayerMove::Run(Direction::Left);
println!("On this turn: {:?}", m);
//{}


    let result: Result<i32, &str> = Ok(42);

    if let Ok(x) = result {
        println!("The result is: {}", x);
    } else {
        println!("Error occurred");
    }

    fn main() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        
        while let Some(number) = numbers.pop() {
            println!("Processing number: {}", number);
        }
    }

//creating methods
//we se impl and then they declare functions and use it like
//let mut rr=mrthod_name::new(" ");

//traits
//impl pet for dog || for a seperate dog struct

//unsafe rust
//dereferencing pointers is unsafe
//reading mutable variables


mm::doo();




    













}
