// #![forbid(unsafe_code)]
use ::std::{
    collections::HashMap,
};

/// Typical example of lack-of-Polonius limitation: get_or_insert pattern.
/// See https://nikomatsakis.github.io/rust-belt-rust-2019/#72
// fn get_or_insert (
//     map: &'_ mut HashMap<u32, String>,
// ) -> &'_ String
// {
//     if let Some(v) = map.get(&22) {
//         return v;
//     }
//     map.insert(22, String::from("hi"));
//     &map[&22]
// }

// Unsafe method
// fn get_or_insert (
//     map: &'_ mut HashMap<u32, String>,
// ) -> &'_ String
// {   
//     if let Some(v) = unsafe { (*(map as *const HashMap<u32,String>)).get(&22) } {
//         return v;
//     }
//     map.insert(22, String::from("hi"));
//     &map[&22]
// }


fn get_or_insert (
    map: &'_ mut HashMap<u32, String>,
) -> &'_ String
{
    // written like this to show the "transition path" from previous code
    let should_insert =
        if let Some(_discarded) = map.get(&22) {
            false
        } else {
            true
        }
    ;
    // but `should_insert` can obviously be shortened down to `map.get(&22).is_none()`
    // or, in this very instance, to `map.contains_key(&22).not()`.
    if should_insert {
        map.insert(22, String::from("hi"));
    }
    map.get(&22).unwrap() // or `&map[&22]`
}