mod io_utils;
mod halstead;
mod rating;

use halstead::{n2_star, task1, task2};
use io_utils::{read_f64_default, read_f64_list_default, read_usize_default};
use rating::{task3_one_period, CVariant};

/// Небольшая функция для единообразного вывода чисел.
fn print_num(name: &str, x: f64) {
    // Формат: имя слева, значение справа, 6 знаков после запятой
    println!("{:<22} = {:.6}", name, x);
}

fn main() {
    println!("ЛР3: Метрики Холстеда (Rust)\n");

    // ------------------------------------------------------------
    // 1) Значения по умолчанию (вариант 1) — удобно для проверки.
    // ------------------------------------------------------------
    // Задания №1 и №2 (ТЗ на БКС)
    let def_targets = 20usize; // C — целей
    let def_meas = 30usize;    // M — измерений параметра на цель
    let def_tracked = 10usize; // P — отслеживаемых параметров
    let def_out = 3usize;      // Q — рассчитываемых параметров на цель
    let def_lambda = 1.53f64;  // λ — уровень языка

    // Задание №2: организационные параметры (задаются самостоятельно)
    let def_m = 12f64;         // m — число программистов
    let def_nu = 24f64;        // ν — производительность (команд/день)
    let def_workday = 8f64;    // длительность рабочего дня (ч)

    // Задание №3: рейтинг программиста
    let def_r0 = 1000f64;
    let def_volumes = vec![5.0, 7.0, 9.0, 11.0]; // объёмы программ, Кбайт
    let def_bugs = vec![0.0, 2.0, 5.0, 4.0];     // ошибки по программам
    let def_next_vol = 15f64;                    // объём будущей программы, Кбайт

    // ------------------------------------------------------------
    // 2) Ввод исходных данных.
    // Enter -> берём значение по умолчанию (вариант 1).
    // ------------------------------------------------------------
    println!("--- Ввод данных (Enter = по умолчанию) ---");
    let targets = read_usize_default("Число целей C", def_targets);
    let meas = read_usize_default("Измерений на параметр M", def_meas);
    let tracked = read_usize_default("Отслеживаемых параметров P", def_tracked);
    let outp = read_usize_default("Рассчитываемых параметров на цель Q", def_out);
    let lambda = read_f64_default("Уровень языка λ", def_lambda);

    let m_prog = read_f64_default("Число программистов m (задание 2)", def_m);
    let nu = read_f64_default("Производительность ν (команд/день)", def_nu);
    let workday = read_f64_default("Длительность рабочего дня (часы)", def_workday);

    let r0 = read_f64_default("Начальный рейтинг R0 (задание 3)", def_r0);
    let volumes = read_f64_list_default("Объёмы программ Vj (Кбайт)", &def_volumes);
    let bugs = read_f64_list_default("Ошибки Bk по программам", &def_bugs);
    let next_vol = read_f64_default("Объём следующей программы (Кбайт)", def_next_vol);

    // Простая проверка корректности входных массивов в задании №3
    if volumes.len() != bugs.len() {
        eprintln!(
            "Ошибка: количество Vj ({}) не равно количеству Bk ({}).",
            volumes.len(),
            bugs.len()
        );
        return;
    }

    // ------------------------------------------------------------
    // 3) Общая подготовка: n2* используется в заданиях №1 и №2.
    // ------------------------------------------------------------
    let n2s = n2_star(targets, meas, tracked, outp);

    // ------------------------------------------------------------
    // 4) Задание №1: V* и B.
    // ------------------------------------------------------------
    println!("\n=== Задание 1 ===");
    print_num("n2*", n2s);

    let (v_star, b1) = task1(n2s, lambda);
    print_num("V*", v_star);
    print_num("B (по V*)", b1);

    // ------------------------------------------------------------
    // 5) Задание №2: структурные параметры, объём, время, надёжность.
    // ------------------------------------------------------------
    println!("\n=== Задание 2 ===");
    let t2 = task2(n2s, m_prog, nu, workday);

    print_num("k", t2.k);
    print_num("i (уровни)", t2.i_levels);
    print_num("K (модули)", t2.k_modules);
    print_num("N", t2.n_len);
    print_num("V", t2.v_vol);
    print_num("P", t2.p_asm);
    print_num("Tk (дней)", t2.tk_days);
    print_num("Tk (часов)", t2.tk_hours);
    print_num("B (по V)", t2.b_err);

    match t2.tn_hours {
        Some(x) => print_num("t_n (часов)", x),
        None => println!("{:<22} = {}", "t_n (часов)", "не определено (B <= 1)"),
    }

    // ------------------------------------------------------------
    // 6) Задание №3: рейтинг и ожидаемые ошибки для 3 вариантов c(λ,R).
    // ------------------------------------------------------------
    println!("\n=== Задание 3 (3 варианта c) ===");

    let variants = [
        (CVariant::Sum, "c = 1/(λ+R)"),
        (CVariant::Product, "c = 1/(λ·R)"),
        (CVariant::InvSum, "c = 1/λ + 1/R"),
    ];

    for (var, title) in variants {
        println!("\n--- {title} ---");
        let (r1, c_next, b_next) = task3_one_period(r0, lambda, &volumes, &bugs, next_vol, var);
        print_num("R после периода", r1);
        print_num("c(λ,R)", c_next);
        print_num("B_next", b_next);
    }
}