// Задание №3: рейтинг программиста и ожидаемые ошибки.

/// Три варианта коэффициента c(λ, R) по условию.
#[derive(Debug, Clone, Copy)]
pub enum CVariant {
    Sum,     // c = 1/(λ + R)
    Product, // c = 1/(λ * R)
    InvSum,  // c = 1/λ + 1/R
}

/// Вычисление коэффициента c(λ, R) в зависимости от выбранного варианта.
fn c_value(variant: CVariant, lambda: f64, r: f64) -> f64 {
    match variant {
        CVariant::Sum => 1.0 / (lambda + r),
        CVariant::Product => 1.0 / (lambda * r),
        CVariant::InvSum => (1.0 / lambda) + (1.0 / r),
    }
}

/// Расчёт рейтинга за один оцениваемый период.
/// Используется формула вида:
/// R = R0 * [ 1 + 10^-3 * ( ΣVj - Σ(Bk / c(λ, R0)) ) ].
///
/// Далее ожидаемые ошибки:
/// B_next = c(λ, R) * V_next.
pub fn task3_one_period(
    r0: f64,
    lambda: f64,
    volumes_kb: &[f64],
    bugs: &[f64],
    next_volume_kb: f64,
    variant: CVariant,
) -> (f64, f64, f64) {
    // 1) ΣVj
    let sum_v: f64 = volumes_kb.iter().sum();

    // 2) c(λ, R0) — используем рейтинг начала периода
    let c0 = c_value(variant, lambda, r0);

    // 3) Σ(Bk / c)
    let sum_penalty: f64 = bugs.iter().map(|b| b / c0).sum();

    // 4) Новый рейтинг по формуле
    let r1 = r0 * (1.0 + 1e-3 * (sum_v - sum_penalty));

    // 5) Ожидаемые ошибки в следующей программе
    let c_next = c_value(variant, lambda, r1);
    let b_next = c_next * next_volume_kb;

    (r1, c_next, b_next)
}