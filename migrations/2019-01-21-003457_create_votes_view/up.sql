CREATE VIEW users_votes AS
SELECT 
  username, 
  questions.survey_id,
  choices.question_id, 
  choice_id, 
  score 
FROM votes 
JOIN choices ON votes.choice_id = choices.id 
JOIN questions ON questions.id = choices.question_id;
