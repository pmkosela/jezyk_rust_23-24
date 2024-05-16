1. Stwórz strukturę `Date` oraz zaimplementuj dla niej następujące metody:
- `fn to_string(&self) -> String` 
- `fn from_3(day: u8, month: Month, year: u16) -> Date` 
- `fn from_string(string: &str, delim: char) -> Date`

2. Stwórz strukturę `Time` oraz zaimplementuj dla niej metody
   analogiczne jak dla struktury `Date`.

3. Zmodyfikuj strukturę `Date` tak, aby mogła opcjonalnie przechowywać również
   godziny. Zastanów się nad cechami, które w tym wypadku może implementować
   struktura `Time`. Dodaj metody: `fn has_time(&self) -> bool`,
   `fn set_time(&mut self, time: _)`, `fn clear_time(&mut self, time: _)`.

5. W zgodzie ze zdroworozsądkowym poczuciem czasu oraz dokumentacją
   zaimplementuj cechy: `PartialOrd`, `Ord` , `PartialEq`, `Eq` dla struktury
   `Date`. Zastanów się nad pięknem ludzkiego spostrzegania czasu oraz
   czasem-samym-w-sobie ;)

6. Stwórz strukturę `Task`. Powinna zawierać następujące pola:
- name: String
- description: String
- priority (Low, Medium, High)
- due: Date

6. Zaimplementuj cechy `PartialOrd`, `Ord` , `PartialEq`, `Eq` dla struktury
   `Date`.
