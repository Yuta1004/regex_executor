extern crate regex_executor;
use regex_executor::automaton::nfa::{ NFA, NFAError };

fn main() -> Result<(), NFAError> {
    // NFA構成
    let chains = vec![
        (0, 0, 'a'), (0, 0, 'b'), (0, 1, 'a'), (1, 1, 'a'), (1, 1, 'b'),
        (1, 2, 'a'), (2, 2, 'a'), (2, 2, 'b'), (2, 3, 'b'), (2, 0, '@')
    ];
    let mut nfa = NFA::new(0, 3);
    for chain in &chains {
        nfa.set_chain(chain.0, chain.1, chain.2)?;
    }

    // 実行
    let target = String::from("abababbbabababbbaabbaab");
    println!("pattern: (a|b)* aab");
    println!("target str: \"{}\"", target);
    println!("result: {}", nfa.simulate(target));
    Ok(())
}
