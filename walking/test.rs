fn main() {
    let x = 3u32;

    let ref_a = &x;
    let ref_b = ref_a;

    println!("{}", *ref_b);
}