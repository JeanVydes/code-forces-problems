fn solve<R: BufRead, W: Write>(mut input: FastInput<R>, mut w: W) {
    let n: usize = input.token();
    let mut nums: Vec<usize> = Vec::new();
    for _ in 0..n*2 {
        nums.push(input.token());
    }

    let mut count = 0;
    while count < n*2 {
        let pair1 = nums[count];
        let pair2 = nums[count+1];

        if pair1 > pair2 {
            write!(w, "{} {}\n", pair2, pair1);
        } else {
            write!(w, "{} {}\n", pair1, pair2);
        }
        
        count += 2;
    }
}