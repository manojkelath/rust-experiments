pub struct Score(pub String, pub i32);

pub fn get_total_score(scores: &mut [Score]) -> i32 {
    let mut total_score = 0;
    for score in scores {
        total_score += score.1;
    }
    total_score
}
