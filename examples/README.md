# L-System Examples

## Algae (algae.rs)
This is based off of Lindenmayer's original L-System for modelling the growth of algae, described [here](https://en.wikipedia.org/wiki/L-system#Example_1:_algae).

**Rules:** \
**A** -> AB \
**B** -> A

**Axiom:** A

**Produces:**\
**n = 0:** A \
**n = 1:** AB \
**n = 2:** ABA \
**n = 3:** ABAAB \
**n = 4:** ABAABABA \
**n = 5:** ABAABABAABAAB \
**n = 6:** ABAABABAABAABABAABABA

When run, the program should output the above information.

## Binary Tree (bin_tree.rs)
This will generate a sequence of characters that can be interpreted to draw binary trees, described [here](https://en.wikipedia.org/wiki/L-system#Example_2:_fractal_(binary)_tree).

**Rules:** \
0 -> 1[0]0 \
1 -> 11

**Axiom:** 0

**Produces:**\
**n = 0:** 1[0]0 \
**n = 1:** 11[1[0]0]1[0]0 \
**n = 2:** 1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0 \
**n = 3:** 11111111[1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0]1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0

When run, the program should output the above information.

## Sierpinski Triangle (sierpinski_triangle.rs)
This will generate a sequence of characters that can be interpreted to draw a Sierpinski triangle fractal, described [here](Example_5:_Sierpinski_triangle).

**Rules:** \
**G** -> GG \
**F** -> F-G+F+G-F

**Axiom:** F-G-G

**Produces:** \
**n = 0:** F-G+F+G-F-GG-GG \
**n = 1:** F-G+F+G-F-GG+F-G+F+G-F+GG-F-G+F+G-F-GGGG-GGGG \
**n = 2:** F-G+F+G-F-GG+F-G+F+G-F+GG-F-G+F+G-F-GGGG+F-G+F+G-F-GG+F-G+F+G-F+GG-F-G+F+G-F+GGGG-F-G+F+G-F-GG+F-G+F+G-F+GG-F-G+F+G-F-GGGGGGGG-GGGGGGGG \
***(...)***

When run, the program should generate the above information.

Feel free to try the following Python code on [pythonsandbox.com](https://pythonsandbox.com/turtle), which will interpret output from this example to draw a Sierpinski triangle:
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