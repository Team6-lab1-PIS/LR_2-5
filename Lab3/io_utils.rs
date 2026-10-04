// -------------------- io_utils.rs --------------------
// Вспомогательные функции ввода/разбора данных.

use std::io::{self, Write};

/// Читает строку из консоли.
/// Используется как базовая функция для всех остальных вводов.
fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Ошибка чтения stdin");

    s.trim().to_string()
}

/// Ввод целого числа (usize) с возможностью оставить значение по умолчанию.
/// Если пользователь вводит пустую строку -> возвращаем default.
pub fn read_usize_default(prompt: &str, default: usize) -> usize {
    let s = read_line(&format!("{prompt} [{default}]: "));
    if s.is_empty() {
        default
    } else {
        s.parse::<usize>().expect("Ожидалось целое число")
    }
}

/// Ввод числа с плавающей точкой (f64) с default.
/// Здесь я заменяю запятую на точку, чтобы ввод “1,53” работал корректно.
pub fn read_f64_default(prompt: &str, default: f64) -> f64 {
    let s = read_line(&format!("{prompt} [{default}]: "));
    if s.is_empty() {
        default
    } else {
        s.replace(',', ".")
            .parse::<f64>()
            .expect("Ожидалось вещественное число")
    }
}

/// Разбор списка чисел вида "5; 7; 9; 11".
/// Разделители: ';' и пробелы.
/// (Запятая не используется как разделитель списка, чтобы не мешать дробным числам.)
fn parse_f64_list(s: &str) -> Vec<f64> {
    s.split(|ch: char| ch == ';' || ch.is_whitespace())
        .filter(|p| !p.is_empty())
        .map(|p| {
            p.replace(',', ".")
                .parse::<f64>()
                .expect("Ошибка в списке чисел")
        })
        .collect()
}

/// Ввод списка чисел с возможностью оставить значения по умолчанию.
pub fn read_f64_list_default(prompt: &str, default: &[f64]) -> Vec<f64> {
    let default_str = default
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("; ");

    let s = read_line(&format!("{prompt} [{default_str}]: "));
    if s.is_empty() {
        default.to_vec()
    } else {
        parse_f64_list(&s)
    }
}