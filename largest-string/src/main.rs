// This program implements the largest string

fn largest<'a>(str1:&'a str, str2:&'a str) -> &'a str {

    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main()  {

    let string_1 = "Kiran";
    let string_2 = "Kannur";

    let result = largest(string_1,string_2);
    println!("Larger string of {} and {} is {}",string_1,string_2,result);

}
