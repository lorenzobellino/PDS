// fn make_box(a: i32) -> Box<(i32, i32)> {
//     let b = Box::new((a, 1));
//     return b;
// }
#[derive(Debug)]
struct Test(i32);
impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping {} at address {:p}", self.0, self);
    }
}

fn main() {
    // let b = make_box(5);
    // let c = b.0 + b.1;
    // println!("{}", c);
    // let k = (12, 13, true, 12.1);
    // print!("{:?}", k);

    // let mut v : Vec<i16> = Vec::new();
    // v.push(1);
    // v.push(20);

    // let t = Test(42);
    // println!("created test : {} at address {:p}", t.0, &t);
    // println!("finishing main");

    // for i in 0..4 {
    //     let t = Test(i);
    //     println!("created test : {} at address {:p}", t.0, &t);
    //     println!("finishing block {}", i);
    // }

    // let mut v: Vec<i32> = Vec::<i32>::new();
    // println!(
    //     "ptr : {:p}, capacity : {}, size : {}",
    //     v.as_ptr(),
    //     v.capacity(),
    //     v.len()
    // );
    // v.push(1);
    // v.push(2);
    // println!(
    //     "ptr : {:p}, capacity : {}, size : {}",
    //     v.as_ptr(),
    //     v.capacity(),
    //     v.len()
    // );
    // println!("& v[0] : {:p}", &v[0]);
    // v.push(3);
    // v.push(4);
    // v.push(5);
    // println!(
    //     "ptr : {:p}, capacity : {}, size : {}",
    //     v.as_ptr(),
    //     v.capacity(),
    //     v.len()
    // );
    // println!("& v[0] : {:p}", &v[0]);
    // v.pop();
    // v.pop();
    // println!(
    //     "ptr : {:p}, capacity : {}, size : {}",
    //     v.as_ptr(),
    //     v.capacity(),
    //     v.len()
    // );
    // println!("& v[0] : {:p}", &v[0]);

    // v.shrink_to_fit();
    // println!(
    //     "ptr : {:p}, capacity : {}, size : {}",
    //     v.as_ptr(),
    //     v.capacity(),
    //     v.len()
    // );
    // let mut k = Vec::<i32>::with_capacity(10);
    // k.push(1);
    // println!(
    //     "ptr : {:p}, capacity : {}, size : {}",
    //     k.as_ptr(),
    //     k.capacity(),
    //     k.len()
    // );
    // k.shrink_to(2);
    // println!(
    //     "ptr : {:p}, capacity : {}, size : {}",
    //     k.as_ptr(),
    //     k.capacity(),
    //     k.len()
    // );

    let mut v = Vec::<Test>::new();
    println!(
        "ptr : {:p}, capacity : {}, size : {}",
        v.as_ptr(),
        v.capacity(),
        v.len()
    );
    v.push(Test(1));
    println!(
        "ptr : {:p}, capacity : {}, size : {}",
        v.as_ptr(),
        v.capacity(),
        v.len()
    );
    v.push(Test(2));
    v.push(Test(3));
    v.push(Test(4));
    v.push(Test(5));
    println!(
        "ptr : {:p}, capacity : {}, size : {}",
        v.as_ptr(),
        v.capacity(),
        v.len()
    );
    v.pop();
    v.pop();
    println!(
        "ptr : {:p}, capacity : {}, size : {}",
        v.as_ptr(),
        v.capacity(),
        v.len()
    );
    v.shrink_to_fit();
    println!(
        "ptr : {:p}, capacity : {}, size : {}",
        v.as_ptr(),
        v.capacity(),
        v.len()
    );
    let a = v.pop();
    println!("a : {:?}", a);
}
