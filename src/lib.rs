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


/// Given two strings, will compute the hamming distance between them
/// 
/// Given two strings, using the hamming distance algorithm to work out the distance between them
/// Returns the distance as a javascript number.
#[wasm_bindgen]
pub fn hamming(string1: &str, string2: &str) -> i32 {
    let result = hamming::compute(string1, string2);

    result
}

/// Given two strings, computes the levenshtein difference between them
/// 
/// Given two strings, uses the levenshtein algorithm to work out the distance between them
/// This algorithm checks for the minimum number of insertions, deletions or subtitutions required to change one string into the other
#[wasm_bindgen]
pub fn levenshtein(string1: &str, string2: &str) -> i32 {

    let result = levenshtein::compute(string1, string2);

    result
}

// /// Given two strings, computes the levenshtein difference between them
/// 
/// Computes the levenshtein edit distance between two strings
/// implemented using teh wagner fischer dynamic programming algorithm
#[wasm_bindgen]
pub fn wagner_fischer(string1: &str, string2: &str) -> i32 {
    let result = wagner_fischer::compute(string1, string2);

    result
}

/// Given two strings, computes the damerau levenshtein difference between them
/// 
/// Given two strings, uses the damerau levenshtein algorithm to work out the distance between them
/// Checks same operations as levenshtein (isnertions, deletions or substitutions), but also includes transposition of two adjacent characters
/// Note this is the simpler "Optimal string alignment distance" algorithm, which is slightly simpler than the "true" demerau levenshtein distance
/// The difference between the two algorithms can be seen here <https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance>
#[wasm_bindgen]
pub fn damerau_levenshtein(string1: &str, string2: &str) -> i32 {
    let result = damerau_levenshtein::compute(string1, string2);

    result
}

/// Given two strings, computes the jaro winkler function for distance between them
/// 
/// Given two strings, uses the jaro winkler function to return a score between 0 and 1 between them
/// 0 identifies two identical strings, 1 means they have no similarities
/// Good function for shorter strings
#[wasm_bindgen]
pub fn jaro_winkler(string1: &str, string2: &str) -> f64 {
    // 1 - to match expected format
    let result = 1.0 - jaro_winkler::compute(string1, string2);

    result
}

/// Given two strings, computes the longest common subsequence (LCS) between them
/// 
/// Uses an implementation of a well known dynamic programming based LCS algorithm
/// to compute the longest common subsequence between two given strings
#[wasm_bindgen]
pub fn lcs(string1: &str, string2: &str) -> i32 {
    let result = lcs::compute(string1, string2);

    result
}

/// Given two strings, computes an ngram-based distance value between them
/// 
/// Given two strings, uses an ngram based distance algorithm to calculate a value between them.
/// Inspired by "Fast String Matching using an n-gram Algorithm" by jong yong kim and john shawe-taylor
#[wasm_bindgen]
pub fn ngram(string1: &str, string2: &str, n: i32) -> f64 {
    let result = ngram::compute(string1, string2, n);

    result
}

/// Given two strings, computes a vector space distance algorithm which returns a floating point distance value between them.
/// 
/// Given two strings, uses a vector space distance algorithm to calculate a value between them.
/// Adapted from Richard Connor et al's equivalent algorithm originally implemented in java,
/// described in the paper "Modelling String Structure in Vector Spaces" by Richard Connor, Alan Dearle, and Lucia Vadicamo.
#[wasm_bindgen]
pub fn jenson_shannon_vector(string1: &str, string2: &str) -> f64 {
    let result = jensen_shannon_vector::compute(string1, string2);

    result
}
