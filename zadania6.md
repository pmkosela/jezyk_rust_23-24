1. Napisz funkcję `fraction(numerator: i32, denominator: i32) -> Option<f32>`,
   która wykona odpowiednie dzielenie lub zwróci `None`, jeżeli to niemożliwe.

2. Napisz funkcję `position(element: i32, array: &[i32] -> Option<usize>)`.
   Funkcja powinna zwrócić indeks elementu w tablicy lub `None`, jeżeli element
   nie jest w tablicy.

3. Napisz funkcję `divisors(number: Option<u32>) -> usize`, która zwróci liczbę
   dzielników parametru *number* lub zakończy działanie programu, jeśli
   *number* ma wartość `None`.

4. Napisz funkcję `wizytowka(imie: Option<String>, nazwisko: Option<String>) ->
   String`, która stworzy wizytówkę (zadanie 5.3), w której w przypadku braku
   imienia zostanie użyte imię *Jan*, a w przypadku braku nazwiska --
   *Kowalski*.

5. Napisz funkcję `miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>,
   Option<f32>)` która oblicza rzeczywiste miejsca zerowe funkcji kwadratowej.

6. Napisz funkcję `oceny(punkty: &[u32], &mut[Result<u8, u32>])`, która
   przyporządkuje oceny studentom według pewnego klucza. Jeśli ktoś ma więcej
   niż 100 punktów, należy na tej pozycji umieścić wartość, informującą o
   błędzie i podać liczbę punktów ponad progiem.

7. Napisz funkcję `rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>])`,
   która przyjmie tablicę liczb zapisanych w postaci napisów w systemach
   dziesiętnym i szesnastkowym. Funkcja powinna rozpoznać system, w którym
   zapisana jest liczba i przekonwertować ją do zmiennej typu *u32*. Przyjmij,
   że liczby szesnastkowe oznaczone są prefiksem `0x`. Nie wszystkie napisy
   muszą być poprawne; zadbaj o obsługę błędów.

8. Napisz funkcję `dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>`,
   która pisemnie doda dwie, dowolnie duże liczby naturalne. Jeśli któryś z
   napisów nie jest liczbą, funkcja powinna zwrócić odpowiedni błąd.
