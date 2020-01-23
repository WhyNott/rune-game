const RUNES : [char; 26]  = [
	'ᚨ',
	'ᚠ',
	'ᚢ',
	'ᚦ',
	'ᚱ',
	'ᚲ',
	'ᚷ',
	'ᚹ',
	'ᚻ',
	'ᚾ',
	'ᛁ',
	'ᛃ',
	'ᛇ',
	'ᛈ',
	'ᛉ',
	'ᛊ',
	'ᛋ',
	'ᛏ',
	'ᛒ',
	'ᛖ',
	'ᛗ',
	'ᛚ',
	'ᛜ',
	'ᛝ',
	'ᛟ',
	'ᛞ'];


const DOT : char = '\u{030A}';

#[derive(PartialEq)]
enum RuleClause {
    This(char),
    Other(char)
}

// impl std::cmp::PartialEq<char> for RuleClause {
//     fn eq(&self, other: &char) -> bool {
// 	match self {
// 	    RuleClause::This(ch) => ch == other,
// 	    RuleClause::Other(ch) => ch == other,
// 	}
//     }
// }

use std::collections::BTreeMap;
use std::collections::BTreeSet;


extern crate dialoguer;
extern crate console;

use dialoguer::{theme::CustomPromptCharacterTheme, Input};
use console::Style;

extern crate rand;
extern crate rand_distr;

use rand::prelude::*;
use rand_distr::Pareto;


fn get_bound_index() -> usize {
    let distr = Pareto::new(1., 1.).unwrap();
    loop {
	let val: f64 = thread_rng().sample(distr);
	if val < 27. {
	    return (val - 1.) as usize
	}
    }
}


use rand::seq::SliceRandom;

fn main() {

    let positive = &RUNES[0..13];
    let positive_style = Style::new().green();
    
    let negative = &RUNES[13..26];
    let negative_style = Style::new().red();



    let alignment = |rune| {
	if positive.contains(&rune) {
	    return positive_style.apply_to(rune)
	} else if negative.contains(&rune) {
	    return negative_style.apply_to(rune)
	}
	unreachable!();
    };

    
    let mut rules : BTreeMap<char, Vec<RuleClause>>  =  BTreeMap::new();

    let mut box1 : BTreeSet<char> = BTreeSet::new();
    
    let mut box2 : BTreeSet<char> = BTreeSet::new();

    let mut rng = thread_rng();

    //generate randomized rules
    let mut shuffled_runes = RUNES;
    shuffled_runes.shuffle(&mut rng);

    for _i in 1..20{

	let is_other = rand::thread_rng().gen_range(0, 4) == 3;
	
	let pareto_rune = shuffled_runes[get_bound_index()];
	let mut chosen_clause;
	loop {
	    let chosen_rune = *RUNES.choose(&mut rng).unwrap();
	    if !is_other {
	    chosen_clause = RuleClause::This(chosen_rune);
	    } else {
		chosen_clause = RuleClause::Other(chosen_rune);
	    }
	    match rules.get(&pareto_rune) {
		None => {
		    if chosen_rune != pareto_rune {
			break;
		    }
		},
		Some(vector) => {
		    if  chosen_rune != pareto_rune && !vector.contains(&chosen_clause) {
			break;
		    }
		}
	    }
	    
	}
	
	match rules.get_mut(&pareto_rune) {
	    Some(ref mut vector) => vector.push(chosen_clause),
	    None => {
		let vector = vec![chosen_clause]; 
		rules.insert(pareto_rune, vector); }
	}
	
    }
    
    
    
    println!("Positive runes:");
	
    for rune in positive.iter() {
	print!("{} ", positive_style.apply_to(rune));
    }
    
    print!("\n");
    println!("Negative runes:");
    
    for rune in negative.iter() {
	print!("{} ", negative_style.apply_to(rune));
    }
    println!();
    println!();
    
    println!("Rules:");
    for (key, value) in rules {
	print!("{} → ", alignment(key));
	for clause in value {
	    match clause {
		RuleClause::This(rune) => {
		    print!("{} ", alignment(rune))},
		RuleClause::Other(rune) => print!("{}{} ", alignment(rune), DOT)
	    }
	}
	println!();
    }
    
}
