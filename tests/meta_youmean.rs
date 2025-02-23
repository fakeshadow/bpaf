use bpaf::*;

#[test]
fn ambiguity() {
    set_override(false);
    #[derive(Debug, Clone)]
    enum A {
        V(Vec<bool>),
        W(String),
    }

    let a0 = short('a').switch().many().map(A::V);
    let a1 = short('a').argument::<String>("AAAAAA").map(A::W);
    let parser = construct!([a0, a1]).to_options();

    let r = parser
        .run_inner(Args::from(&["-aaaaaa"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Parser supports -a as both option and option-argument, try to split -aaaaaa into individual options (-a -a ..) or use -a=aaaaa syntax to disambiguate");

    let r = parser
        .run_inner(Args::from(&["-b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "No such flag: `-b`, did you mean `-a`?");
}

#[test]
fn short_cmd() {
    set_override(false);
    let parser = long("alpha")
        .req_flag(())
        .to_options()
        .command("beta")
        .short('b')
        .to_options();

    let r = parser
        .run_inner(Args::from(&["c"]))
        .unwrap_err()
        .unwrap_stderr();

    assert_eq!(r, "No such command: `c`, did you mean `b`?");
}
