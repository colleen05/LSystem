use lsystem::{lsystem::LSystem, rules::Rules};

/*
Rules:
A -> AB
B -> A

Axiom: A

Produces:
n = 0 : A
n = 1 : AB
n = 2 : ABA
n = 3 : ABAAB
n = 4 : ABAABABA
n = 5 : ABAABABAABAAB
n = 6 : ABAABABAABAABABAABABA
n = 7 : ABAABABAABAABABAABABAABAABABAABAAB
*/

fn main() {
    /* Setup */
    let mut rules = Rules::<char>::new();
    rules.set('A', vec!['A', 'B']);
    rules.set('B', vec!['A']);

    let axiom = &['A'];
    let system = LSystem::<char>::new(rules.clone(), axiom);

    /* Printing context */
    println!("Rules:\n{}\n", rules);
    println!(
        "Axiom: {}\n",
        axiom
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("")
    );

    /* Iterating */
    println!("Produces:");

    let generations = system.take(7).collect::<Vec<_>>();

    for (n, s) in generations.iter().enumerate() {
        println!("n = {} : {}", n, s.iter().collect::<String>());
    }
}
