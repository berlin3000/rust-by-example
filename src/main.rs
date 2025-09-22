
fn reverse_tuple(pair: (bool, i32)) -> (i32, bool) {
    let (bool_param, int_param) = pair;

    (int_param, bool_param)
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    length: u32
}

impl Rectangle {
    fn new(height: u32, length: u32) -> Rectangle {
        return Rectangle {height, length};
    }

    fn area(&self) -> u32 {
        self.height * self.length
    }
}

fn main() {
    // variable
    let an_integer = 100i32;
    println!("Integer: {}", an_integer);

    // default type
    let default_float = 3.5;
    println!("default float: {}", default_float);

    // mutable
    let mut mutable_integer: i32 = 35;
    println!("mutable integer: {}", mutable_integer);
    mutable_integer = 1000;
    println!("mutable integer after update: {}", mutable_integer);
    

    // overwrite variable with shadowing
    let mutable = 888;
    println!("mutable variable: {}", mutable);
    let mutable = true;
    println!("mutable variable after shadowing: {}", mutable);

    // array
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {}", my_array[0]);

    // tuple - collection of values of different types
    let a_tuple = (5, 5.1, true, [1,2,3], -1.234);
    println!("tuple: {:#?}", a_tuple.3);

    // bitwise operation
    println!("bitwise operation: 10001 AND 01110 is {:04b}", 0b0111u32 | 0b0001);

    // function
    println!("The reversed pair is {:?}", reverse_tuple((true, 34)));

    // implement first struct
    let rectangle: Rectangle = Rectangle::new(8, 7);

    println!("Rectangle values: {:?}", rectangle);
    println!("Rectangle area: {:?}", rectangle.area());

    // arrays and slices
    let array1: [u32; 10] = [8; 10];
    println!("arrays {:?}", array1);

    let array2: [u16; 6] = [2,4,8,16,32,64];
    let array_slice: &[u16] = &array2[1 .. 4];
    println!("slices from array {:?}", array_slice);

    // array access
    for i in 0 .. array_slice.len()+2 {
        match array_slice.get(i) {
            Some(i_value) => println!("{} : {}", i, i_value),
            None => println!("no value present")
        }
    }

}
