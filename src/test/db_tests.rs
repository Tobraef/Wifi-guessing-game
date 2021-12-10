#[cfg(test)]
pub mod tests {
    use crate::db::Context;
    use crate::db::opentdb::opentdb_ctx::OpentdbCtx;
    use crate::db::{Category, Difficulty};

    #[test]
    fn correct_url_parsing() {
        let ctx = OpentdbCtx::new();
        let questions = ctx.load_questions(Some(Difficulty::Easy), Some(Category::GeneralKnowledge), Some(3));
        assert!(questions.iter().all(|q| q.difficulty == Difficulty::Easy));
        assert!(questions.iter().all(|q| q.category == Category::GeneralKnowledge));
        assert_eq!(questions.len(), 3);
    }
}