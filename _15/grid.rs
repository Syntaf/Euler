

pub struct Grid {
    raw: Vec<Vec<bool>>,
    columns: usize,
    rows: usize
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Grid {
        let mut vec: Vec<Vec<bool>> = vec![vec![false; columns]; rows];
        for i in &vec {
            println!("{:?}", i); 
        }
        Grid { raw: vec, columns: columns, rows: rows }
    }
}
