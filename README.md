# L-Systems in Rust

This is just a practise implementation of [L-Systems](https://en.wikipedia.org/wiki/L-system) in Rust.

Currently, Rust's type system isn't being used to its greatest potential here. That may change in the future.

## Example Usage

### Algae
This is based off of Lindenmayer's original L-System for modelling the growth of algae, described [here](https://en.wikipedia.org/wiki/L-system#Example_1:_algae).

**Rules:** \
**A** -> AB \
**B** -> A

**Axiom:** A

**Code:**
```rs
let mut rules = Rules::<char>::new();
rules.set('A', vec!['A', 'B']);
rules.set('B', vec!['A']);

let axiom = &['A'];
let system = LSystem::<char>::new(rules.clone(), axiom);

let generations = system.take(7).collect::<Vec<_>>();

for (n, s) in generations.iter().enumerate() {
    println!("n = {} : {}", n, s.iter().collect::<String>());
}
```

**Output:**
```
n = 0 : AB
n = 1 : ABA
n = 2 : ABAAB
n = 3 : ABAABABA
n = 4 : ABAABABAABAAB
n = 5 : ABAABABAABAABABAABABA
n = 6 : ABAABABAABAABABAABABAABAABABAABAAB
```

### Sierpinski triangle (examples/sierpinski_triangle.rs)
This will generate a sequence of characters that can be interpreted to draw a Sierpinski triangle fractal, described [here](https://en.wikipedia.org/wiki/L-system#Example_5:_Sierpinski_triangle).

**Rules:** \
**G** -> GG \
**F** -> F-G+F+G-F

**Axiom:** F-G-G

**Setup:**
```rs
let mut rules = Rules::<char>::new();
rules.set('F', vec!['F', '-', 'G', '+', 'F', '+', 'G', '-', 'F']);
rules.set('G', vec!['G', 'G']);

let axiom = &['F', '-', 'G', '-', 'G'];
let system = LSystem::<char>::new(rules.clone(), axiom);
```

Example rendering using the program's output:

![Result](examples/images/sierpinski_triangle_rendered.png)