use lsystem::{lsystem::LSystem, rules::Rules};

/*
Rules:
0 -> 1[0]0
1 -> 11

Axiom: 0

Produces:
n = 0 : 1[0]0
n = 1 : 11[1[0]0]1[0]0
n = 2 : 1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0
n = 3 : 11111111[1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0]1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0
*/

fn main() {
    /* Setup */
    let mut rules = Rules::<char>::new();
    rules.set('1', vec!['1', '1']);
    rules.set('0', vec!['1', '[', '0', ']', '0']);

    let axiom = &['0'];
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

    let generations = system.take(4).collect::<Vec<_>>();

    for (n, s) in generations.iter().enumerate() {
        println!("n = {} : {}", n, s.iter().collect::<String>());
    }
}
