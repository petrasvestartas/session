e not curve picks.
    pub fn others(&self) -> impl Iterator<Item = &'static Step> {
        self.steps.iter().filter(|step| !step.curves())
    }
}

/// Read typed words: options anywhere, the last one winning, and answers in step order.
pub fn read(recipe: &Recipe, words: &[&str]) -> Result<(usize, Vec<Answer>), String> {
    let others: Vec<&Step> = recipe.others().collect();
    let mut choice = 0;
    let mut answers = Vec::new();
    let mut rest = words;

    while !rest.is_empty() {
        if let Some((index, count)) = recipe.option_in(rest) {
            choice = index;
            rest = &rest[count..];
            continue;
        }

        let Some(step) = others.get(answers.len()) else {
            let options: Vec<_> = recipe.choices().collect();
            let listed = if options.is_empty() {
                String::new()
            } else {
                format!("; its options are {}", options.join(", "))
            };
            return Err(format!(
                "`{}` is not a value of {}{listed}",
                rest[0], recipe.name
            ));
        };
        answers.push(step.read(rest[0])?);
        rest = &rest[1..];
    }

    // typed values take the defaults of the numbers left, e.g. a full turn
    if !answers.is_empty()
        && others[answers.len()..]
            .iter()
            .all(|step| matches!(step, Step::Number { .. }))
    {
        for step in &others[answers.len()..] {
            if let Step::Number { default, .. } = step {
                answers.push(Answer::Number(*default));
            }
        }
    }

    Ok((choice, answers))
}

