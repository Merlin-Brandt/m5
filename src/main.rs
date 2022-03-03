
//////////////////////////////////////////////////////
//                                                  //
//  Yes, the code is in a very messy state,         //
//  I have never intended to publish the code,      //
//  and so far it was not worth cleaning it up.     //
//                                                  //
//////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(core_intrinsics)]
#![feature(try_blocks)]

extern crate meval;


use regex::Regex;
use std::collections::HashMap;

mod util;
use util::*;

mod error;
use error::*;

mod mmatch;
use mmatch::*;

mod example_input;

#[macro_use]
extern crate lazy_static;

pub static mut BRACES: Option<Braces> = None;

pub fn init_braces() {
    let mut braces = Braces::new();
    braces.push_braces(("'", "`")).unwrap();
    braces.push_braces(("«", "»")).unwrap();
    unsafe {
        BRACES = Some(braces);
    }
}

pub fn braces() -> Braces<'static> {
    unsafe {BRACES.clone().unwrap()}
}

pub const RULE_INVOCATION_CHAR: char = ':';
pub const RULE_DEFINITION_KEY: &str = "%:";

pub const SWIRL_WHITESPACE_RULES: [&str; 4] = ["whitespace", "whitespaces", "nwh", "1nwh"];
pub const SWIRL_WHITESPACE_HANDLER_PARAM_HEADER: &str = "swirl_whitespace_handler_header";
pub const SWIRL_WHITESPACE_HANDLER_PARAM_INPUT: &str = "swirl_whitespace_handler_body";
pub const SWIRL_WHITESPACE_HANDLER_HEADER: &str = "swirl_whitespace_handler_header";
pub const SWIRL_WHITESPACE_HANDLER_BODY: &str = "swirl_whitespace_handler_body";
pub const SWIRL_WHITESPACE_HANDLER_CATCH_BODY: &str = "swirl_whitespace_handler_body";


static mut INDENT: usize = 0;

fn push_indent() {unsafe {INDENT += 2;}}
fn pop_indent() {unsafe {INDENT -= 2;}}
fn get_indent() -> String {unsafe {"  ".repeat(INDENT)}}

// todo: remove unescaped whitespace

mod types;
use types::*;

// first string in returned pair is the skipped text
pub fn find_statement(input: &Input) -> Option<(&str, &Input)> {
    input.find(RULE_DEFINITION_KEY).map(|i| (&input[..i], &input[i..]))
}

use std::fs::File;

pub fn match_statement<'a>(input: &'a Input, rules: &Rules) -> MatchResult<(&'a Input, (String, Option<RuleVariant>))> {
    match match_rule_definition(input, rules) {
        Ok((input, (name, variant))) => Ok((input, (name, Some(variant)))),
        Err(def_err) => Err(def_err),
    }
}

fn init_rules() -> Rules {
    let mut rules = HashMap::new();
    rules.insert("swirl_default_call_explicit_syntax".to_string(), {
        Rule::new("swirl_default_call_explicit_syntax".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_feature_undefine_rule".to_string(), {
        Rule::new("swirl_feature_undefine_rule".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_new_quote_signs".to_string(), {
        Rule::new("swirl_new_quote_signs".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_version_0_0_1".to_string(), {
        Rule::new("swirl_version_0_0_1".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_version_0_0_2".to_string(), {
        Rule::new("swirl_version_0_0_2".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_version_0_0_3".to_string(), {
        Rule::new("swirl_version_0_0_3".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_version_0_1_0".to_string(), {
        Rule::new("swirl_version_0_1_0".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_version_0_2_0".to_string(), {
        Rule::new("swirl_version_0_2_0".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_load".to_string(), {
        Rule::new("swirl_load".to_string())
            .variant(RuleVariant::empty())
    });
    rules.insert("swirl_main".to_string(), {
        Rule::new("swirl_main".to_string())
    });
    rules
}

pub fn process(input: &str, rules: &mut Rules, mut appleft: MaybeInf<u32>, remove_defs: bool, mut receive_output: impl FnMut(&str) -> MatchResult<()>) -> MatchResult<()> {
    let mut input = input.to_string();

    while let Some((skipped_text, statement_begin)) = find_statement(&input) {
        // todo: use swirl to sweeten this up to 
        // break if appleft == MaybeInf::Finite(0);
        if appleft == MaybeInf::Finite(0) {
            break;
        }

        match match_statement(statement_begin, rules) {
            Ok((statement_end, (name, maybe_variant))) => {
                // all text until the current rule definition remains untouched (because it is between the beginning/a rule definition and a rule definition)
                // so just push it to the result string
                receive_output(skipped_text)?;
                if !remove_defs {
                    receive_output(&statement_begin[..(statement_begin.len() - statement_end.len())])?;
                }

                // add variant to definitions (or remove) (perhaps call it)
                if let Some(variant) = maybe_variant {
                    let name = || name.clone();
                    let rule_entry = rules.entry(name()).or_insert(Rule::new(name()));
                    let name = name();

                    // next portion to process is after the current rule definition
                    input = statement_end.to_string();

                    if variant.is_undefine() {
                        rules.remove(&name);
                    } else {
                        rule_entry.variants.push(variant.clone());

                        // empty name means invocation
                        if name.is_empty() {
                            // next portion to process is the output of application of the current rule definition (piped to all previous unnamed rule definitions)
                            let new_input = rules[&name].match_sequence(&input, rules, &mut appleft)?;
                            // if this rule was just to be applied once, remove from definitions
                            if variant.shallow_call() {
                                rules.get_mut(&name).unwrap().variants.pop().unwrap();
                            }
                            input = new_input;
                        }

                    }
                }
            },
            Err(def_err) => {
                // let user-defined ::swirl_main rule parse the string followed by %:
                match rules["swirl_main"].match_last(statement_begin, "", rules) {
                    Ok((new_input, result)) => {
                        input = result + new_input;
                    },
                    Err(main_err) => {
                        Err(MatchError::compose("Cannot parse %: statement", vec![def_err, main_err]))?
                    }
                }
            }
        }
    }

    // the rest of the input contains no more rule definitions, so output it
    receive_output(&input)?;
    Ok(())
}

fn process_file(target: &str, steps: MaybeInf<u32>, remove_defs: bool) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    File::open(&target)?.read_to_string(&mut buffer)?;
    
    let mut result = String::new();
    process(&buffer, &mut init_rules(), steps, remove_defs, |lines| result.push_str(lines).tap(Ok))?;

    File::create(format!("{}.out", target))?.write(result.as_bytes())?;

    Ok(())
}

use std::io::{self, Read, Write};
use std::error::Error;

static mut VERBOSE: bool = false; 

pub fn is_verbose() -> bool {
    return unsafe {VERBOSE};
}

//#[cfg(not(debug_assertions))]
fn main()  {
    let is_stepping = std::env::args().any(|s| s == "--step" || s == "-s");
    let remove_defs = !std::env::args().any(|s| s == "--print-defs" || s == "-d");
    let repl = std::env::args().any(|s| s == "--repl");
    unsafe { VERBOSE = std::env::args().any(|s| s == "--verbose" || s == "-v") };

    let steps = if is_stepping {
        MaybeInf::Finite(1)
    } else {
        MaybeInf::Infinite
    };

    init_braces();

    if cfg!(debug_assertions) {
        println!(" -- Debug mode --");
        unsafe { ::std::intrinsics::breakpoint() }
        process_file("src/test.swirl", MaybeInf::Infinite, true).map_err(|e| eprintln!("{}", e)).unwrap();
    }

    if !repl {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)
            .map_err(|e| eprintln!("{}", e)).unwrap();
        process(&buffer, &mut init_rules(), steps, remove_defs, |lines| {
            print!("{}", lines);
            io::stdout().flush().unwrap();
            Ok(())
        }).map_err(|e| {
            eprintln!("{}", e);
        }).unwrap();
    } else {
        let stdin = io::stdin();
        let mut userline = String::new();
        let mut rules = init_rules();
    
        print!(" $ ");
        io::stdout().flush().unwrap();
    
        while stdin.read_line(&mut userline).is_ok() {
            process(&userline, &mut rules, steps, remove_defs, |lines| {
                print!("{}", lines);
                Ok(())
            }).map_err(|e| {
                eprintln!("{}", e);
            }).unwrap();
            print!(" $ ");
            io::stdout().flush().unwrap();
            userline.clear();
        }
    }
}

