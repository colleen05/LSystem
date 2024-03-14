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

    let p = rules.productions.clone();

    let axiom = &['A'];
    let mut system = LSystem::<char>::new(rules, axiom);

    /* Printing context */
    println!("Rules:");
    for (k, v) in p {
        print!("{} -> ", k);

        for v in v {
            print!("{}", v);
        }

        println!(" ");
    }

    print!("\nAxiom: ");
    for c in axiom {
        print!("{} ", c);
    }

    /* Iterating */
    println!("\n\nProduces:");
    for n in 0..=7 {
        print!("n = {} : ", n);

        for c in system.state() {
            print!("{}", c);
        }

        println!(" ");

        system.next();
    }
}
