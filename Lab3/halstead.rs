// -------------------- halstead.rs --------------------
// Расчёты для задания №1 и №2.

/// Вычисление n2*
/// Принятое допущение:
/// n2* = (C * M * P) + (C * Q)
/// где:
///   C — число целей
///   M — число измерений на параметр
///   P — число отслеживаемых параметров
///   Q — число рассчитываемых параметров на цель
pub fn n2_star(c_targets: usize, m_meas: usize, p_tracked: usize, q_out: usize) -> f64 {
    let inputs = (c_targets * m_meas * p_tracked) as f64;
    let outputs = (c_targets * q_out) as f64;
    inputs + outputs
}

/// Задание №1:
/// V* = (n2* + 2) log2(n2* + 2)
/// B  = (V*^2) / (3000 * λ)
pub fn task1(n2s: f64, lambda: f64) -> (f64, f64) {
    let v_star = (n2s + 2.0) * (n2s + 2.0).log2();
    let b = (v_star * v_star) / (3000.0 * lambda);
    (v_star, b)
}

/// Результаты задания №2 складываем в структуру,
/// чтобы в main.rs не держать много отдельных переменных.
pub struct Task2Out {
    pub k: f64,          // предварительная оценка числа модулей
    pub i_levels: f64,   // уровни иерархии
    pub k_modules: f64,  // итоговое число модулей K
    pub n_len: f64,      // длина N
    pub v_vol: f64,      // объём V
    pub p_asm: f64,      // команды ассемблера P
    pub tk_days: f64,    // календарное время (дни)
    pub tk_hours: f64,   // календарное время (часы)
    pub b_err: f64,      // потенциальные ошибки (по V/3000)
    pub tn_hours: Option<f64>, // начальная надёжность (может не определиться при B<=1)
}

/// Задание №2:
/// k = n2*/8
/// (если k >> 8) i = log2(n2*)/3 + 1,  K ≈ n2*/8 + n2*/8^2
/// N = 220K + K log2 K
/// V ≈ K * 220 * log2(48)
/// P = 3N/8
/// Tk = 3N / (8 m ν)
/// B = V/3000
/// t_n = Tk / (2 ln B)   (Tk переведено в часы)
pub fn task2(n2s: f64, m_programmers: f64, nu_prod: f64, workday_hours: f64) -> Task2Out {
    // 1) k = n2*/8
    let k = n2s / 8.0;

    // 2) Проверка "k >> 8". Для практической реализации считаем k > 8 достаточным.
    let hierarchical = k > 8.0;

    // 3) Уровни иерархии
    let i_levels = if hierarchical {
        (n2s.log2() / 3.0) + 1.0
    } else {
        1.0
    };

    // 4) Итоговое число модулей K
    let k_modules = if hierarchical {
        (n2s / 8.0) + (n2s / (8.0 * 8.0))
    } else {
        k
    };

    // 5) Длина программы N
    let n_len = 220.0 * k_modules + k_modules * k_modules.log2();

    // 6) Объём программы V
    let v_vol = k_modules * 220.0 * 48.0_f64.log2();

    // 7) Команды ассемблера P
    let p_asm = 3.0 * n_len / 8.0;

    // 8) Календарное время разработки (в днях), затем в часах
    let tk_days = 3.0 * n_len / (8.0 * m_programmers * nu_prod);
    let tk_hours = tk_days * workday_hours;

    // 9) Ошибки по формуле задания №2
    let b_err = v_vol / 3000.0;

    // 10) Начальная надёжность имеет смысл только при B > 1 (иначе ln(B) <= 0)
    let tn_hours = if b_err > 1.0 {
        Some(tk_hours / (2.0 * b_err.ln()))
    } else {
        None
    };

    Task2Out {
        k,
        i_levels,
        k_modules,
        n_len,
        v_vol,
        p_asm,
        tk_days,
        tk_hours,
        b_err,
        tn_hours,
    }
}