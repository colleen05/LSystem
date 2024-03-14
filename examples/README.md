# L-System Examples

## Algae
This is based off of Lindenmayer's original L-System for modelling the growth of algae, seen [here](https://en.wikipedia.org/wiki/L-system#Example_1:_algae).

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
n = 7 : ABAABABAABAABABAABABAABAABABAABAAB
```