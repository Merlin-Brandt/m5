use std::collections::{BTreeMap, HashMap};
use crate::*;

// for clarification: matches/applies a rule, not its definition (it has already been defined and read by m5) 

impl Rule {
    /// start trying to apply rule variants from the bottom up, skipping a number of variants
    pub fn match_last_skip<'a>(&self, input: &'a str, param: &str, rules: &Rules, skip: usize, candidate_errors: Vec<MatchError>) -> MatchResult<(&'a str, String)> {
        //let variants = &rules.get(name).ok_or_else(|| MatchError::new(format!("Rule '{}' does not exist.", name), &mut vec![]))?.variants;
        let mut candidate_errors = candidate_errors;
        for (i, v) in self.variants.iter().rev().enumerate().skip(skip) {

            v.on_enter(&self.name, input);

            match v.try_match(input, param, rules, &self.name, i) {
                // call on_fail, on_success
                Ok((input, result)) => {
                    v.on_success(&self.name, input, &result);
                    return Ok((input, result))
                },
                Err(err) => {
                    v.on_failure(&self.name, input, err.clone());
                    if !v.suppress_error() {
                        candidate_errors.push(err);
                    }
                },
            }
        }
        if candidate_errors.is_empty() {
            candidate_errors.push(MatchError::location(input));
        }
        return MatchError::compose(format!("No variant of '{}' matched.", self.name), candidate_errors).tap(Err);
    }

    pub fn is_macro_(name: &str) -> bool {
        [
            "m5cl",
            "m5_ident",
            "m5_rule_invoc",
            "m5_var",
            "m5_quote",
            "m5_quote_value",
            "m5_header",
            "m5_body",
            "m5_inner_rule_def",
            "m5_rule_def",
            "m5_file_invoc",
            "m5_print",
            "m5_rule_exists",
            "m5_verbose",
            "m5_load",
            "m5_import",
            "m5_print_defs"
        ].contains(&name)
    }

    /// try applying rule variants from the bottom up
    pub fn match_last<'a>(&self, input: &'a str, param: &str, rules: &Rules) -> MatchResult<(&'a str, String)> {
        if self.is_macro() {
            if self.name == "m5_print_defs" {
                unsafe { remove_defs = param == "false"; }
                Ok((input, "".to_string()))
            }
            else if self.name == "m5_load" {
                let path = param;
                Ok((input, dump_file(path)
                    .or_else(|err| {
                        if let Some(contents) = get_m5_lib(path) {
                            Ok(contents)
                        } else {
                            MatchError::new(format!("Error loading '{}': {}", param, err)).tap(Err)                         
                        }
                    })?))
            }
            /*else if self.name == "m5_import" {
                let filepath = param;
                let split = filepath.rfind('/');
                let (dir, file) = match split {
                    Some(split) => (&filepath[0..=split], &filepath[split..]),
                    None => ("./", filepath)
                };
                unimplemented!();
                Ok((input, dump_file(filepath)
                    .map_err(|err| MatchError::new(format!("Error loading '{}': {}", param, err)))?))
            }*/
            else if self.name == "m5_verbose" {
                unsafe {
                    if true || param == "y" {
                        VERBOSE = true;
                    } else {
                        VERBOSE = false;
                    }
                }
                Ok((input, "".to_string()))
            }
            else if self.name == "m5_rule_exists" {
                Ok((input, {
                    if rules.get(param).is_some() {
                        "t"
                    } else {
                        "f"
                    }
                }.to_string()))
            }
            else if self.name == "m5_print" {
                print!("{}", param);
                Ok((input, "".to_string()))
            }
            else if self.name == "m5cl" {
                meval::eval_str(param)
                    .map(|result| (input, result.to_string().trim_end().to_string()))
                    .map_err(|e| MatchError::new(format!("{}", e)))
            }  else if self.name == "m5_rule_invoc" {
                let (after_input, _) = match_rule_invoc(input, rules)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_var" {
                let (after_input, _) = match_var(input)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_quote" {
                let (_, after_input) = match_quote(input)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_quote_value" {
                let (contained_text, after_input) = match_quote(input)?;
                Ok((after_input, contained_text.to_string()))
            } else if self.name == "m5_header" {
                let usage_err = MatchError::new("Correct usage: m5_header({}) to match a header between curly braces.");
                let wrap_begin = param.chars().nth(0)
                    .ok_or(usage_err.clone())?;
                let wrap_end = param.chars().nth(1)
                    .ok_or(usage_err)?;
                let (after_input, maybe_invoc) = match_invocation_string_def(input, rules, wrap_begin, wrap_end, M5_WHITESPACE_HANDLER_HEADER)?;
                maybe_invoc.ok_or(MatchError::expected("Rule Header", input))?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_body" {
                let usage_err = MatchError::new("Correct usage: m5_body({}) to match a body between curly braces.");
                let wrap_begin = param.chars().nth(0)
                    .ok_or(usage_err.clone())?;
                let wrap_end = param.chars().nth(1)
                    .ok_or(usage_err)?;
                let (after_input, _) = match_invocation_string_def(input, rules, wrap_begin, wrap_end, M5_WHITESPACE_HANDLER_BODY)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_inner_rule_def" {
                let (after_input, _) = match_inner_rule_definition(input, rules)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_rule_def" {
                let (after_input, _) = match_rule_definition(input, rules)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_file_invoc" {
                let (after_input, _) = match_file_invocation(input, rules)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            } else if self.name == "m5_ident" {
                let (after_input, _) = match_ident(input)?;
                let len = input.len() - after_input.len();
                Ok((after_input, input[..len].to_string()))
            }
            else {
                unreachable!()
            }
        } else {
            self.match_last_skip(input, param, rules, 0, vec![])
        }
    }

    pub fn match_all<'a>(&self, input: &'a str, param: &str, rules: &Rules) -> MatchResult<String> {
        let (input_after, result) = self.match_last(input, param, rules)?;
        if input_after == "" {
            Ok(result)
        } else {
            MatchError::new(format!("::{}({}) did not consume all of its input, '{}' was left.", self.name, param, input_after))
                .tap(Err)
                .trace(self.backtraceline(input))
        }
    }

    // if one variant in sequence fails, the whole sequence fails.
    pub fn match_sequence(&self, input: &str, rules: &Rules, appleft: &mut MaybeInf<u32>) -> Result<String, MatchError> {
        if !self.name.is_empty() {
            return MatchError::new("Can only apply the unnamed rule in sequence.").tap(Err);
        }

        let mut input = input.to_string();
        for (i, variant) in self.variants.iter().rev().enumerate() {
            //backtrace.push(format!("%: {{{}}}", variant.header.as_ref().unwrap_or(&"".to_string())));
            //let _f =  finally(|| {backtrace.pop();});

            if *appleft == MaybeInf::Finite(0u32) {
                break;
            }

            *appleft-= 1;

            variant.on_enter(&self.name, &input);

            //let (unconsumed, replace) = variant.try_match(&input, "", rules, "", i)?;
            //input = replace + unconsumed;

            input = match variant.try_match(&input, "", rules, "", i) {
                Ok((unconsumed, replace)) => {
                    variant.on_success(&self.name, &input, &replace);
                    let input = replace + unconsumed;
                    input
                },
                Err(err) => {
                    variant.on_failure(&self.name, &input, err.clone());
                    return Err(err);
                }
            };
        }
        Ok(input)
    }
}
