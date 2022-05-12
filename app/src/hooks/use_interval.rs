use yew::prelude::*;
use gloo::timers::callback::Interval;

use super::use_mut_latest;

pub fn use_interval<Callback>(callback: Callback, millis: u32)
where
    Callback: FnMut() + 'static,
{
    let callback_ref = use_mut_latest(callback);
    let interval_ref = use_mut_ref(|| None);

    use_effect_with_deps(
        move |millis| {
            if *millis > 0 {
                *interval_ref.borrow_mut() = Some(Interval::new(*millis, move || {
                    let callback_ref = callback_ref.current();
                    let callback = &mut *callback_ref.borrow_mut();
                    callback();
                }));
            } else {
                *interval_ref.borrow_mut() = None;
            }

            move || *interval_ref.borrow_mut() = None
        },
        millis,
    );
}
