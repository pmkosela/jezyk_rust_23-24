1. Napisz funkcję o nagłówku `fn powtorki(t: ...) -> ...` która z danego
   wektora utworzy nowy z tymi samymi wartościami, ale tylko tymi, które się po
   sobie powtarzają. Na przykład:
   `powtorki(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]) == vec![3, 3, 3, 3, 1, 1]`
           

2. Napisz funkcję o nagłówku `fn unikalne(t: ...) -> ...` która utworzy i
   zwróci nowy wektor, ale tylko z wartościami, które w danym wektorze się nie
   powtarzają (w zachowanej kolejności). Na przykład:
`unikalne(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]) == vec![5, 6]`

3. Napisz funkcję o nagłówku `fn zlicz_wiele(s1: ..., s2: ...) -> ...` która
   zwróci liczbę elementów (z powtórzeniami) wektora s1 zawartych w s2 (lub
           odwrotnie — wynik będzie ten sam).

```
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2]) == 3
zlicz_wiele(&vec![1, 2, 1, 3], &vec![12]) == 0
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2, 2]) == 4
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2, 2, 1]) == 6
```

Wykorzystując iteratory, utwórz (i wyświetl) wektor zawierający:

4. małe litery alfabetu angielskiego;

5. kwadraty 10. kolejnych liczb całkowitych począwszy od 1;

6. 10 kolejnych potęg dwójki;

7. odwrotności wszystkich liczb od 1 do 20;

8. liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4.

Wykorzystując iteratory, napisz zestaw funkcji, które dla danego wektora
napisów zwrócą:

9. wektor zawierający napisy krótsze niż 4 znaki;

10. wektor napisów nie zawierających liter 'a' ani 'A';

11. wektor napisów zawierających cyfry;

12. wektor zawierający te same napisy ale odwrócone;

13. wektor napisów zawierających podwojoną literę (np.: inny, pizza, brutto, lekki, dzienny, itp).

14. Napisz funkcję `fn indeksy(tablica: ..., element: ...) -> ...` która zwróci
    wektor indeksów (licząc od zera), na których znajduje się w zadanej tablicy
    podany element. Zadanie wykonaj w dwóch wersjach — przy użyciu pętli oraz
    bez ich użycia (z iteratorami zamiast tego).
