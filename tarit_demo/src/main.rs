fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("{result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("{result}");

    let p1 = Point {x: 5, y: 10.1};
    let p2 = Point{x: 'a', y: "hello"};

    let p3 = p1.mixup(p2);

    let newArticle = NewArticle {
        headline: String::from("horse_ebooks"),
        location: String::from("suzhou"),
        author: String::from("andy"),
        content: String::from("test"),
    };

    println!("1 new newArticle: {}", newArticle.summarize());



}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest;
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
