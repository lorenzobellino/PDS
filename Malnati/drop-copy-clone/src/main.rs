struct Test(i32);
impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping Test({}) @ {:p}", self.0, self);
    }
}

struct Test2(i32);
impl Clone for Test2 {
    fn clone(&self) -> Self {
        println!("Cloning Test2({}) @ {:p}", self.0, self);
        Test2(self.0)
    }
}
impl Copy for Test2 {}

fn alfa(t: Test) {
    println!("Chiamata la funzione alfa() con Test({}) @ {:p}", t.0, &t);
}

fn alfa2(t: &Test) {
    println!("Chiamata la funzione alfa2() con Test({}) @ {:p}", t.0, &t);
}

fn main() {
    let t = Test(6);
    println!("Creando Test({} @ {:p}", t.0, &t);
    alfa(t);
    // println!("Accedendo a Test({} @ {:p}", t.0, &t);
    // non posso accedere a t perchè la sua ownership e stata
    // spostata al momento in cui è stata chiamata la funzione alfa
    let t2 = Test(10);
    println!("Creando Test({} @ {:p}", t2.0, &t2);
    alfa2(&t2);
    println!("Accedendo a Test({} @ {:p}", t2.0, &t2);

    println!("Ending ...");

    let t3 = Test2(20);
    let mut t4 = t3.clone();
    println!("Test2({}) @ {:p}", t3.0, &t3);
    println!("Test2({}) @ {:p}", t4.0, &t4);

    t4.0 += 1;
    println!("Test2({}) @ {:p}", t3.0, &t3);
    println!("Test2({}) @ {:p}", t4.0, &t4);

    let t5 = Test2(30);
    let mut t6 = t5;
    println!("Test2({}) @ {:p}", t5.0, &t5);
    println!("Test2({}) @ {:p}", t6.0, &t6);
    t6.0 += 2;
    println!("Test2({}) @ {:p}", t5.0, &t5);
    println!("Test2({}) @ {:p}", t6.0, &t6);
}
