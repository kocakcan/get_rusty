/// Returns the n-th largest element in a slice
fn find_nth<T: Ord + Clone>(elems: &mut [T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}

/// Normally if you try to compile this function, the compiler returns the following error:
///
///     error [E0596]: cannot borrow `*elems` as mutable, as it is behind a `&` reference

struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>,
}

impl TestResult {
    pub fn get_curve(&self) -> &Option<usize> {
        &self.curve
    }

    /// If there is a curve, then increments all
    /// scores by the curve
    // pub fn apply_curve(&mut self) {
    //     if let Some(curve) = self.get_curve() {
    //         for score in self.scores.iter_mut() {
    //             *score += *curve;
    //         }
    //     }
    // }

    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.curve {
            for score in self.scores.iter_mut() {
                *score += curve;
            }
        }
    }
}
/// error[E0502]: cannot borrow `self.scores` as mutable because it is also borrowed as immutable

fn main() {
    let mut v: Vec<i32> = vec![5, 4, 3, 2, 1];
    println!("2nd largest: {}", find_nth(&mut v, 1));
    let mut test_result = TestResult {
        scores: vec![10, 20, 30, 40, 50],
        curve: Some(15),
    };

    test_result.apply_curve();
    println!("Final scores: {:?}", test_result.scores);
}
