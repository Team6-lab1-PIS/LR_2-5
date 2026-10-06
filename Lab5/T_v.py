import random

# Генерируем 100 случайных значений T_в_i в интервале [0.7, 1.2]
N = 100
T_vi = [random.uniform(0.7, 1.2) for _ in range(N)]

# Считаем T_в формуле: (1/N) * сумму всех T_в_i
T_v = sum(T_vi) / N

print(f"Среднее время = {T_v:.3f} с")