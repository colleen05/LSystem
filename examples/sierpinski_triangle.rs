use lsystem::{lsystem::LSystem, rules::Rules};

/*
Rules:
G -> GG
F -> F-G+F+G-F

Axiom: F-G-G

This will generate a sequence of characters that can be used to draw a sierpinski triangle fractal.

Feel free to try the following Python turtle code on https://pythonsandbox.com/turtle:
```py
import turtle

s = "" # Replace with generation 6 from program output.

t = turtle.Turtle()
t.hideturtle()
t.penup()
t.setx(-250)
t.sety(-200)
t.pendown()

for c in s:
if c == "F" or c == "G":
    t.forward(3.9) # Make this value bigger for smaller generations.
elif c == "-":
    t.left(120)
elif c == "+":
    t.right(120)
```
*/

fn main() {
    /* Setup */
    let mut rules = Rules::<char>::new();
    rules.set('F', vec!['F', '-', 'G', '+', 'F', '+', 'G', '-', 'F']);
    rules.set('G', vec!['G', 'G']);

    let axiom = &['F', '-', 'G', '-', 'G'];
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
