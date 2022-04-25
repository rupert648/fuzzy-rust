#![warn(missing_docs)]

//! This crate contains "JS functions" for neon, allowing the user to perform several varying string comparison algorithms of varying complexity

// imports
use crate::string_matchers::{
    damerau_levenshtein,
    hamming,
    levenshtein,
    wagner_fischer,
    jaro_winkler,
    lcs,
    ngram,
    jensen_shannon_vector
};

mod string_matchers;

use wasm_bindgen::prelude::*;

/// Given two strings, will comppute the hamming distance between them
/// 
/// Given two strings, computes the distance between them using Jensen-Shannon Distance over strings.
/// Third argument is a boolean, where `true` means that the casing of the strings will be ignored
/// Function Example:
/// ```js
/// const fuzzy = require('fuzzy-neon');
/// let score = fuzzy.distance('nick', 'nice', true);
/// console.log(score);
/// ```
#[wasm_bindgen]
pub fn distance(string1: &str, string2: &str, ignore_case: bool) -> f64 {

    let result: f64 = if ignore_case {
        jensen_shannon_vector::compute(&string1.to_lowercase(), &string2.to_lowercase())
    } else {
        jensen_shannon_vector::compute(&string1, &string2)
    };

    result
}


// /// Given two strings, will compute the hamming distance between them
// /// 
// /// Given two strings, using the hamming distance algorithm to work out the distance between them
// /// Returns the distance as a javascript number.
// pub fn hamming(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);

//     let result = hamming::compute(&string1, &string2);

//     Ok(cx.number(result))
// }

// /// Given two strings, computes the levenshtein difference between them
// /// 
// /// Given two strings, uses the levenshtein algorithm to work out the distance between them
// /// This algorithm checks for the minimum number of insertions, deletions or subtitutions required to change one string into the other
// pub fn levenshtein(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);

//     let result = levenshtein::compute(&string1, &string2);

//     Ok(cx.number(result))
// }

// /// Given two strings, computes the levenshtein difference between them
// /// 
// /// Computes the levenshtein edit distance between two strings
// /// implemented using teh wagner fischer dynamic programming algorithm
// pub fn wagner_fischer(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);

//     let result = wagner_fischer::compute(&string1, &string2);

//     Ok(cx.number(result))
// }

// /// Given two strings, computes the damerau levenshtein difference between them
// /// 
// /// Given two strings, uses the damerau levenshtein algorithm to work out the distance between them
// /// Checks same operations as levenshtein (isnertions, deletions or substitutions), but also includes transposition of two adjacent characters
// /// Note this is the simpler "Optimal string alignment distance" algorithm, which is slightly simpler than the "true" demerau levenshtein distance
// /// The difference between the two algorithms can be seen here <https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance>
// pub fn damerau_levenshtein(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);

//     let result = damerau_levenshtein::compute(&string1, &string2);

//     Ok(cx.number(result))
// }

// /// Given two strings, computes the jaro winkler function for distance between them
// /// 
// /// Given two strings, uses the jaro winkler function to return a score between 0 and 1 between them
// /// 0 identifies two identical strings, 1 means they have no similarities
// /// Good function for shorter strings
// pub fn jaro_winkler(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);

//     // 1 - to match expected format
//     let result = 1.0 - jaro_winkler::compute(&string1, &string2);

//     Ok(cx.number(result))
// }

// /// Given two strings, computes the longest common subsequence (LCS) between them
// /// 
// /// Uses an implementation of a well known dynamic programming based LCS algorithm
// /// to compute the longest common subsequence between two given strings
// pub fn lcs(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);

//     let result = lcs::compute(&string1, & string2);

//     Ok(cx.number(result))
// }

// /// Given two strings, computes an ngram-based distance value between them
// /// 
// /// Given two strings, uses an ngram based distance algorithm to calculate a value between them.
// /// Inspired by "Fast String Matching using an n-gram Algorithm" by jong yong kim and john shawe-taylor
// pub fn ngram(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
//     let ngram_handle = cx.argument::<JsNumber>(2)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);
//     let n = ngram_handle.value(&mut cx) as i32;

//     let result = ngram::compute(&string1, &string2, n);

//     Ok(cx.number(result))
// }

// /// Given two strings, computes a vector space distance algorithm which returns a floating point distance value between them.
// /// 
// /// Given two strings, uses a vector space distance algorithm to calculate a value between them.
// /// Adapted from Richard Connor et al's equivalent algorithm originally implemented in java,
// /// described in the paper "Modelling String Structure in Vector Spaces" by Richard Connor, Alan Dearle, and Lucia Vadicamo.
// pub fn jenson_shannon_vector(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     let string1_handle = cx.argument::<JsString>(0)?;
//     let string2_handle = cx.argument::<JsString>(1)?;
    
//     let string1 = string1_handle.value(&mut cx);
//     let string2 = string2_handle.value(&mut cx);

//     let result = jensen_shannon_vector::compute(&string1, &string2);

//     Ok(cx.number(result))
// }
