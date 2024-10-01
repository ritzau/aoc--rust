use std::result::Result;

use crate::PuzzleCache;

pub fn not_quite_lisp() -> Result<(), ureq::Error> {
    println!("Hello, AOC15!");

    let cache = PuzzleCache::new("cache".into());
    let body: String = cache.fetch_input(2015, 1)?;
    println!("Body: {}", body.len());

    Ok(())
}
