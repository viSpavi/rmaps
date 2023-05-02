pub(super) fn handle_enter() -> Box<dyn Fn()> {
    Box::new(|| {
        println!("enter");
    })
}
