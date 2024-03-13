use std::thread;

fn main() {
    let list1 = vec![1,2,3];
    // 不可变引用
    let only_borrows = || println!("From closure: {:?}", list1);
    
    let mut list2 = vec![1, 2, 3];
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    borrows_mutably();
    println!("before calling closure: {:?}", list2);

    let list3 = vec![1,2,3];
    println!("before defining closure: {:?}", list3);
    thread::spawn(move || println!("from thread: {:?}", list3))
    .join()
    .unwrap();

    let mut list4 = [
        Rectangle {width: 10, heigth: 1},
        Rectangle {width: 3, heigth: 5},
        Rectangle {width: 7, heigth: 12},
    ];

    let mut num_sort_operations = 0;
    list4.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations}", list4);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}
