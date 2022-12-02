use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::ParseError::ParseRoundError;

#[derive(Debug)]
pub enum Selection {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
pub enum MatchResult {
    Win,
    Lose,
    Draw
}

#[derive(Debug)]
pub enum ParseError {
    ParseTournamentError,
    ParseRoundError,
    ParseChoiceError
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing input")
    }
}

impl Error for ParseError {}

pub fn parse_tournament(input: &[&str]) -> Result<Vec<(Selection, Selection)>, ParseError> {
    let mut rounds: Vec<(Selection, Selection)> = Vec::new();
    input.iter()
        .for_each(|raw_round| rounds.push(parse_round(raw_round).unwrap()));

    Ok(rounds)
}

fn parse_round(round: &str) -> Result<(Selection, Selection), ParseError> {
    let mut split_string = round.split_whitespace();
    let opp = split_string.next().ok_or(ParseRoundError)?;
    let user = split_string.next().ok_or(ParseRoundError)?;

    let opp = parse_opp_choice(opp)?;
    let user = decode_user_choice(user, &opp)?;

    Ok((opp, user))
}

/* Method for parsing part 1 inputs. Unused but keeping around for documentation of part1 work
fn parse_choice(choice: &str) -> Result<Selection, ParseError> {
    match choice {
        "A" => Ok(Selection::Rock),
        "X" => Ok(Selection::Rock),
        "B" => Ok(Selection::Paper),
        "Y" => Ok(Selection::Paper),
        "C" => Ok(Selection::Scissors),
        "Z" => Ok(Selection::Scissors),
        _ => Err(ParseError::ParseChoiceError)
    }
} */

fn parse_opp_choice(choice: &str) -> Result<Selection, ParseError> {
    match choice {
        "A" => Ok(Selection::Rock),
        "B" => Ok(Selection::Paper),
        "C" => Ok(Selection::Scissors),
        _ => Err(ParseError::ParseChoiceError)
    }
}

fn decode_user_choice(choice: &str, opp: &Selection) -> Result<Selection, ParseError> {
    let selection= match choice {
        "X" => {
            match opp {
                Selection::Rock => Selection::Scissors,
                Selection::Paper => Selection::Rock,
                Selection::Scissors => Selection::Paper,
            }
        },
        "Y" => {
            match opp {
                Selection::Rock => Selection::Rock,
                Selection::Paper => Selection::Paper,
                Selection::Scissors => Selection::Scissors,
            }
        },
        "Z" => {
            match opp {
                Selection::Rock => Selection::Paper,
                Selection::Paper => Selection::Scissors,
                Selection::Scissors => Selection::Rock,
            }
        },
        _ => panic!("UNPARSEABLE INPUTS: {}, {:?}", choice, opp)
    };
    Ok(selection)
}

pub fn eval_tournament(rounds: Vec<(Selection, Selection)>) -> (u32, u32) {
    let [mut opp, mut user] = [0; 2];

    for (opp_choice, user_choice) in rounds {
        let (opp_score, user_score) = eval_round(opp_choice, user_choice);
        opp += opp_score;
        user += user_score;
    }

    (opp, user)
}

fn eval_round(opp: Selection, user: Selection) -> (u32, u32) {
    let (opp_points, user_points);

    let (opp_result, user_result) = match (&opp, &user) {
        (Selection::Rock, Selection::Rock) => (MatchResult::Draw, MatchResult::Draw),
        (Selection::Rock, Selection::Paper) => (MatchResult::Lose, MatchResult::Win),
        (Selection::Rock, Selection::Scissors) => (MatchResult::Win, MatchResult::Lose),
        (Selection::Paper, Selection::Rock) => (MatchResult::Win, MatchResult::Lose),
        (Selection::Paper, Selection::Paper) => (MatchResult::Draw, MatchResult::Draw),
        (Selection::Paper, Selection::Scissors) => (MatchResult::Lose, MatchResult::Win),
        (Selection::Scissors, Selection::Rock) => (MatchResult::Lose, MatchResult::Win),
        (Selection::Scissors, Selection::Paper) => (MatchResult::Win, MatchResult::Lose),
        (Selection::Scissors, Selection::Scissors) => (MatchResult::Draw, MatchResult::Draw),
    };

    opp_points = selection_points(opp) + result_points(opp_result);
    user_points = selection_points(user) + result_points(user_result);

    (opp_points, user_points)
}

fn selection_points(selection: Selection) -> u32 {
    match selection {
        Selection::Rock => 1,
        Selection::Paper => 2,
        Selection::Scissors => 3,
    }
}

fn result_points(result: MatchResult) -> u32 {
    match result {
        MatchResult::Win => 6,
        MatchResult::Lose => 0,
        MatchResult::Draw => 3
    }
}