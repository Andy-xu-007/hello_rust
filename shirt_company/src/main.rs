use shirt_company::Inventory;
use shirt_company::ShirtColor;

fn main() {
    let store = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_1 = Some(ShirtColor::Blue);
    let user_2 = None;
    let giveaway_1 = store.giveaway(user_1);
    let giveaway_2 = store.giveaway(user_2);
    println!(
        // 使用 {:?} 格式化宏来打印一个类型（如 ShirtColor），你需要确保该类型实现了 std::fmt::Debug 特征。
        // 这个特征提供了将结构体或枚举等自定义类型转换为可读字符串的机制，这对于调试和输出日志非常有用
        "this user with preference {:?} gets {:?}",
        user_1, giveaway_1
    );

    println!(
        "this user with preference {:?} gets {:?}",
        user_2, giveaway_2
    );

}

