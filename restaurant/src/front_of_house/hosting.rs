// 如果将 hosting.rs 放在 src 目录，编译器会认为 hosting 模块中的 hosting.rs 的代码
// 声明于 crate 根，而不是声明为 front_of_house 的子模块
pub fn add_to_waitlist() {}