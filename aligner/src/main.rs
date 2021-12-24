pub fn increasing<'a>(slice: &'a [i32]) -> &'a [i32] {
    let mut ret: &'a [i32] = slice;
    for i in 0..slice.len() - 1 {
        if slice[i] >= slice[i + 1] {
            ret = &slice[..=i];
            break;
        }
    }
    ret
}

fn main() -> Result<(), String> {
    let vec = vec![2, 4, 7, 8, 6, 3, 5];
    let result;
    {
        let slice = &vec[..];
        result = increasing(slice);
    }
    assert_eq!(result, &[2, 3, 4, 5, 6, 7, 8]);
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    assert_eq!(increasing(&[8, 4, 2, 7]), &[2, 4, 7, 8]);
}