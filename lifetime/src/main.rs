
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i;

    {
        i = ImportantExcerpt {
            part: first_sentence,
        };

        // i = &j;

        // println!("i: {}", i.part);
    }

    // let i = ImportantExcerpt {
    //     part: first_sentence,
    // };

    println!("i: {}", i.part);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is: {}", result);



    let result = 0;
    {
        let y = 10;

        // This will not work because y is not valid outside the scope

        println!("y: {}", y);

        // result = &y;
    }

    println!("result: {}", result);

    let s1 = String::from("hello");

    // let result;

    // {
    //     let s2 = String::from("world");
    //     result = longest(&s1, &s2);
    //     println!("The longest string is: {}", result);
    // }
    // let s2 = String::from("hello world");

    // let result  = longest(&s1, &s2);

    println!("The longest string is: {}", result);



    // let s1 = String::from("hello");



//     let result;


    let s1 = "fsadf";
    let result;

    {
        let s2 = "asdfds";
        result = longest(s1, s2);
    }

    println!("The longest string is: {}", result);
    
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
