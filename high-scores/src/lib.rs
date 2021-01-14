#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {scores: scores.to_vec()}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        let l = self.scores.clone();
        l.into_iter().last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let b = self.scores.clone();
        b.into_iter().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut tb = self.scores.clone();
        tb.sort_unstable();
        tb.reverse();
        tb.truncate(3);
        tb

    }
}
