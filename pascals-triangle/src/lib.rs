pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

// pub struct Row(Vec<u32>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut res = PascalsTriangle {
            rows: Vec::default(),
        };

        for i in 1..row_count + 1 {
            let mut v = vec![1];

            let mut n = 1;
            for k in 1..i {
                n *= i - k;
                n /= k;
                v.push(n);
            }

            res.rows.push(v);

        }

        res
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.rows
    }
}
