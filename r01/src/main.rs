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
    // COPY support
    let stack_a = 42;
    let stack_b = stack_a;
    println!("stack_a: {}", stack_a);
    println!("stack_b: {}", stack_b);

    // COPY not support
    let heap_a = "hello".to_string();
    let heap_b = heap_a;
    // for this expr. heap_a was moved to heap_b; so heap_a is null
    println!("heap_a: {}", heap_a);
    println!("heap_b: {}", heap_b);
}

fn main() {
    // println!("Hello, world!");

    // demo1();
    demo2();
}
