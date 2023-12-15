use super::answer::Answer;

pub trait Solution {
    fn part_a(&self, input: &[String]) -> Answer;
    fn part_b(&self, input: &[String]) -> Answer;
}
