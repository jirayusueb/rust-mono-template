// Triggers ts-rs export on `cargo test -p api`.
// ts-rs generates TypeScript types in tests that touch the TS trait.
#[test]
fn export_types() {
    assert_eq!(0, 0);
}
