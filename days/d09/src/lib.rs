pub fn predict(sequence: Vec<i32>) -> i32 {
    if sequence.iter().all(|&x| x==0) {
        return 0
    } else {
        return sequence.last().unwrap() + predict(difference(&sequence))
    }
}

fn difference(sequence: &Vec<i32>) -> Vec<i32> {
    sequence[0..sequence.len()]
        .iter()
        .zip(sequence[1..].iter())
        .map(|(x0, x1)| x1 - x0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_difference() {
        let expected = vec![2, 3, 4, 5, 6];

        let vec = vec![1, 3, 6, 10, 15, 21];
        let result = difference(&vec);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_1() {
        let expected = 18;

        let vec = vec![0, 3, 6, 9, 12, 15];
        let result = predict(vec);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_2() {
        let expected = 28;

        let vec = vec![1, 3, 6, 10, 15, 21];
        let result = predict(vec);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_3() {
        let expected = 68;

        let vec = vec![10, 13, 16, 21, 30, 45];
        let result = predict(vec);

        assert_eq!(expected, result);
    }
}
