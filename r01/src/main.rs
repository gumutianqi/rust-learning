mod demo2;
mod demo3;

fn demo1() {
    // 语法：面向表达式
    for i in 1..10 {
        if i > 0 && i <= 3 {
            println!("A");
        } else if i > 3 && i <= 6 {
            println!("B");
        } else {
            println!("C");
        }
    }
}

fn demo2() {
    // 值类型实现了 COPY 语义，存储在栈内存的指针上
    let stack_a = 42;
    let stack_b = stack_a;
    println!("stack_a: {}", stack_a);
    println!("stack_b: {}", stack_b);
}

fn demo3() {
    // string 类型为实现 COPY 语义，存储在堆内存中
    let heap_a = "hello".to_string();
    // for this expr. heap_a was moved to heap_b; so heap_a is null.
    // we can use heap_a.clone() to fix that.
    let heap_b = heap_a.clone();

    println!("heap_a: {}", heap_a);
    println!("heap_b: {}", heap_b);
}

// 使用 mut 修饰符，标记变量为可变绑定
fn demo4() {
    let mut answer = 42;
    answer = 41;

    println!("answer: {}", answer);


}


// Trait
// Trait
struct A;

impl A {
    fn hello(&self) {
        println!(" in A.")
    }
}

trait Hello {
    fn hello(&self);
    fn hi();
}

impl Hello for A {
    fn hello(&self) {
        print!("from Hello trait.");
    }

    fn hi() {
        println!("from Hello hi");
    }
}

fn main() {
    // println!("Hello, world!");

    // demo1();    // print AAABBBCCC
    // demo2();    // 42 42
    // demo3();    // error
    // demo4();

    let a = A;
    a.hello();

}
