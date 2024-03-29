pub struct ScoreKeeper {
    size: usize,
    data: Box<[Option<bool>]>
}

impl ScoreKeeper {
    pub fn new(size: usize) -> ScoreKeeper {
        let data = Box<[None; size.fac]>
        
        ScoreKeeper {
            size,
            data,
        };
    }
}