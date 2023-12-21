fn main() {
    let x = String::from("andy");
    {
        let y = "cammie";

        let result = longest(x.as_str(), y);
        println!("{}", result);
    }
    

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

