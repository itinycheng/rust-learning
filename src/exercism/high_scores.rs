#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    /// clone &elem to elem
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    /// diff into_iter with iter
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut owned = self.scores.to_owned();
        owned.sort_unstable_by(|a, b| b.cmp(a));
        owned.into_iter().take(3).collect::<Vec<u32>>()
    }
}
