fn increasing<'a>(slice: &'a [i32]) -> &'a [i32] {
    let mut ret: &'a [i32] = slice;
    for i in 0..slice.len() - 1 {
        if slice[i] >= slice[i + 1] {
            ret = &slice[..=i];
            break;
        }
    }
    ret
}

fn main() {
    let vec = vec![2, 4, 7, 8, 6, 3, 5];
    let result = increasing(&vec);
    assert_eq!(result, &[2, 4, 7, 8]);
}
