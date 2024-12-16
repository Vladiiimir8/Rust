## Задание
Для заданной строки, sсостоящей из слов и пробелов, вернуть длину последнего слова в строке.

Слово — это максимум подстрока
состоящий только из символов, не являющихся пробелами.

## Код
``` rust
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed_str = s.trim();
        let mut length = 0;
        for ch in trimmed_str.chars().rev() {
            if ch == ' ' {
                if length > 0 {
                    break;
                }
            } else {
                length += 1;
            }
        }
        length
    }
}
```
## Скриншот
https://github.com/Vladiiimir8/plyaskin_20421/blob/main/rust/zadanie%201/zadanie%201.1/screen.png?raw=true