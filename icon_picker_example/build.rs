pub fn main() {
    // An empty slice means every family the enabled features provide. This
    // generates an index rather than a function per icon — there are close to
    // twenty thousand of them.
    iced_lucide::build_index(&[], "icon").expect("Build icon index");
}
