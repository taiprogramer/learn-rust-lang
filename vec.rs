fn main() {
    let mut a = vec![1, 2, 3];

    let b: &i32 = &a[0];
    let c: Option<i32> = a.pop();
    let d: Option<&i32> = a.get(0);
}

