// fn main() {
//     let v = vec![1, 2, 3];
//     let v_ref: &Vec<i32> = &v;
//     let v2 = *v_ref;
//     drop(v2);
// }

// fn main() {
//     // Doesn't work since cannot move out of a reference
//     let s = String::from("hello");
//     let s_ref = &s;
//     let s2 = *s_ref;
// }

fn build_string_format(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s += &format!("{}", i);
    }
    s
}
fn build_string_pushstr(n: usize) -> String {
    let mut s = String::with_capacity(n * 2);
    for i in 0..n {
        s.push_str(&i.to_string());
    }
    s
}

fn build_strings(n: usize) {
    println!("{}", build_string_format(n));
    println!("{}", build_string_pushstr(n));
}

fn main() {
    build_strings(15);
}
