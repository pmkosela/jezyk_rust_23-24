1. Napisz funkcję, zliczającą liczbę wystąpień pewnego znaku w napisie.

2. Napisz funkcję wizytowka, która otrzymuje w dwóch parametrach napisowych
   imię i nazwisko, a zwraca napis powstały z pierwszej litery imienia, kropki,
   spacji i nazwiska, przy czym w wyniku pierwsza litera imienia i nazwiska
   mają być duże, pozostałe małe. Na przykład, dla danych `jan` oraz `KOWALSKI`
   funkcja ma zwracać napis `J. Kowalski`.

3. Napisz funkcję, która na podstawie napisu tworzy napis, zawierający co drugi
   znak napisu podanego w argumencie.

4. Napisz funkcję, przyjmującą w argumencie napis i zwracającą ten sam napis,
   czytany od tyłu.

5. Zdefiniuj funkcję o nagłówku `fn szyfruj(napis: ..., klucz: ...) -> ...`
   która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem
   odwracającym — klucz określa długość odwracanych fragmentów. Przykłady:
   ```
   szyfruj("Aladyn", 2) == "lAdany"
   szyfruj("Aladyn", 3) == "alAnyd"
   szyfruj("Aladyn", 4) == "dalAny"
   szyfruj("Aladyn", 5) = "ydalAn"
   szyfruj("koza", 3) == "zoka"
   szyfruj("kaszanka", 3) == "saknazak"
   szyfruj("kot Mruczek", 9) == "zcurM tokke"
   szyfruj("kot Mruczek", 1) == "kot Mruczek"
   szyfruj("kot Mruczek", 2) == "ok trMcuezk"
   ```

6. Napisz funkcję o nagłówku `fn dodaj_pisemnie(a: ..., b: ...) -> ...` która
   doda dwie (zakładamy, że poprawne) liczby całkowite podane w argumentach
   jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis.
   Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być
   dowolnie duże. Przykłady:
   ```
   dodaj_pismnie("1", "3") == "4"
   dodaj_pismnie("1", "3") == "4"
   dodaj_pismnie("8", "3") == "11"
   dodaj_pismnie("10", "23") == "33"
   dodaj_pismnie("1", "0") == "1"
   dodaj_pismnie("11", "00") == "11"
   dodaj_pismnie("131", "9900") == "10031"
   dodaj_pismnie("998", "7") == "1005"
   dodaj_pismnie("24872947", "294729478") == "319602425"
   dodaj_pismnie("5924729874298749827418582", "6782893629472094209740298") == "12707623503770844037158880"
   ```
