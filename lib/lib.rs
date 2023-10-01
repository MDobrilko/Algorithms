mod sortings;
mod trees;

pub use crate::sortings::*;
pub use crate::trees::*;

pub fn range(
    from: usize,
    to: usize,
    step: i32,
) -> std::iter::FromFn<impl FnMut() -> Option<usize>> {
    let mut from = from as i32;
    let to = to as i32;

    std::iter::from_fn(move || {
        if (step < 0 && from < to) || (step > 0 && from > to) {
            None
        } else {
            let res = Some(from as usize);
            from += step;
            res
        }
    })
}
