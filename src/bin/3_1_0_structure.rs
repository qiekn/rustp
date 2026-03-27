struct QIEKN(i32, i32, f64);

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

fn area(rect: Rectangle) -> f32 {
    // let Point { x: point_a_x, y: point_a_y } = rect.top_left;
    // let Point { x: point_b_x, y: point_b_y } = rect.bottom_right;

    let Rectangle {
        top_left: Point{ x: point_a_x, y: point_a_y },
        bottom_right: Point{ x: point_b_x, y: point_b_y },
    } = rect;

    let w = point_b_x - point_a_x;
    let h = point_a_y - point_b_y;
    (w * h)
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Qiekn");
    let age = 18;
    let qiekn = Person { name, age };

    // 以 Debug 方式打印结构体
    println!("{:?}", qiekn);

    // 实例化结构体 `Point`
    let point: Point = Point { x: 10.0, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point，
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point {
        y: 114514.0,
        ..point
    };

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y); // 5.2 0.4

    // 使用 `let` 绑定来解构 point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let pa = Point{x:1.0, y:2.0};
    let pb = Point{x:2.0, y:0.0};
    let rect = Rectangle{top_left: pa, bottom_right: pb};
    println!("area is {}", area(rect));
}
