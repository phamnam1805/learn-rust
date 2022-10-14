fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    // println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // // A reference to what is yielded is `&i32`. Destructure to `i32`.
    // println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    vec1.iter().find(|&&x| x == 1).ok_or_else(|| panic!("not good"));
}
