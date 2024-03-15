# L-System Examples

## Algae (algae.rs)
This is based off of Lindenmayer's original L-System for modelling the growth of algae, described [here](https://en.wikipedia.org/wiki/L-system#Example_1:_algae).

When run, it should output:
```
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
```

## Binary Tree (bin_tree.rs)
This is another classic example used for generating binary trees, described [here](https://en.wikipedia.org/wiki/L-system#Example_2:_fractal_(binary)_tree).

When run, it should output:
```
Rules:
0 -> 1[0]0
1 -> 11

Axiom: 0

Produces:
n = 0 : 1[0]0
n = 1 : 11[1[0]0]1[0]0
n = 2 : 1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0
n = 3 : 11111111[1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0]1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0
```