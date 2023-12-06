
// 具名结构体
struct Point {
    x: f32,
    y: f32,
}

// 元组
struct Pair(i32, f32);

// 单元结构体
struct Unit;

fn demo1 () {
    let point = Point{x: 1.0, y: 2.0};
    let pair = Pair(1, 2.0);

    assert_eq!(pair.0, 1);

    let unit1 = Unit;
    let unit2 = Unit;
}