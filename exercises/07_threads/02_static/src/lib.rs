// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let half_index: usize = slice.len() / 2;
    let halved_vector: (&[i32], &[i32]) = slice.split_at(half_index);

    let first_half = halved_vector.0;
    let second_half = halved_vector.1;

    let sum_first_part = thread::spawn(move || {
        println!("summing first half");
        first_half.iter().sum::<i32>()
    });

    let sum_second_part = thread::spawn(move || {
        println!("summing second half");
        second_half.iter().sum::<i32>()
    });

    sum_first_part.join().unwrap() + sum_second_part.join().unwrap()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
