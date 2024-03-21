1. Napisz funkcję `d2((x, y), (x, y)) -> f32`, która obliczy dystans
   pomiędzy punktami w przestrzeni *R^2*.
2. Napisz funkcję `d3((x, y, z), (x, y, z)) -> f32`, która obliczy dystans
   pomiędzy punktami w przestrzeni *R^3*.
3. Stwórz tablicę *N* elementów, którą wypełnisz resztami z dzielenia liczby
   `100` przez kolejne liczby naturalne. Następnie wyświetl kolejne wartości
   tablicy od końca.
4. Napisz funkcję `avg(&[u32]) -> f32`, która obliczy średnią arytmetyczną
   liczb z tablicy.
5. Napisz funkcję `sort(... u32, ... u32, ... u32)`, która rosnąco posortuje
   przekazane jej argumenty.
6. Napisz funkcję `swap_range(... [u32], (a1, a2), (b1, b2))`, która zamieni
   miejscami elementy, leżące w podanych przedziałach; jeśli przedziały mają
   różną długość, ogranicz się do długości krótszego z nich.
7. Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie
   na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i
   trzecim, podane będą minimalne i maksymalne wartości losowanych liczb.
   Funkcja powinna mieć nagłówek: `fn rand(seed: ..., min: ..., max: ...)`.
   Skorzystaj z
   [LCG](https://en.wikipedia.org/wiki/Linear_congruential_generator).
