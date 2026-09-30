import random

# Генерируем 200 случайных значений T_n_i в интервале [9, 14]
N = 200
T_ni = [random.uniform(9, 14) for _ in range(N)]

# Считаем T_в  формуле: (1/N) * сумму всех T_в_i
T_n = sum(T_ni) / N

print(f"Среднее время = {T_n:.3f} с")