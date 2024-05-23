1. Tradycyjne karty do gry (brydż, poker itd.) podzielone są na kolory o
   nazwach:
    - pik,
    - kier,
    - karo,
    - trefl.

    Zaprojektuj typ wyliczeniowy `Color`, który będzie mógł reprezentować dane
    o kolorze z dodatkowym warunkiem, że są one uporządkowane jak w brydżu
    (zgodnie z wypunktowaniem powyżej, gdzie są podane malejąco).

2. Zaprojektuj typ wyliczeniowy `Value`, reprezentujący tradycyjną kartę do
   gry, tzn. będącą numerem od 2 do 10 lub jedną z figur A as, K król, Q dama,
   J walet.

3. Stwórz strukturę `Card`, zawierającą pola o typach `Color` i `Value`.
   Zaimplementuj dla niej cechę `fmt::Display`.

4. Stwórz strukturę `Deck`, będącą kontenerem, początkowo przechowującym
   wszystkie 54 karty (4 figury + 9 blotek = 13, każda w jednym z czterech
           kolorów). Zaimplementuj metody:
   - `draw() -> Option<Card>`, losującą bez zwracania,
   - `len() -> usize`, zwracającą liczbę pozostałych kart.

5. Z typów `Color`, `Value`, `Card` utwórz moduł `playing_cards`.

6. W module nadrzędnym stwórz strukturę `Player`, przechowującą 13 kart. W
   konstruktorze `fn from_deck(&mut Deck)` pobierz 13 kart z talii przekazanej
   w parametrze.

7. Zaimplementuj metodę `Deck::shuffle(&mut self)`.
