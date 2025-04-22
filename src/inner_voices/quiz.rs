pub trait CategoryQuiz: Copy + Eq + std::hash::Hash + Ord {
    fn evaluate(answers: &[Self]) -> Option<Self> where Self:Sized {
        use itertools::Itertools;
        
        answers
            .iter()
            .copied() // Convert &Self to Self for grouping
            .sorted() // group_by requires sorted data
            .chunk_by(|&answer| answer)
            .into_iter()
            .map(|(variant, group)| (variant, group.count()))
            .max_by_key(|&(_, count)| count)
            .map(|(variant, _)| variant)
    }
}
