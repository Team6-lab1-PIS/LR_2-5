import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.Scanner;

public class Main {
    // Интервалы времени между последовательными ошибками.
    private static double[] intervals;

    public static void main(String[] args) {
        Locale.setDefault(Locale.US);

        String fileName = args.length > 0 ? args[0] : "input.txt";

        try {
            intervals = readIntervals(fileName);
            int n = intervals.length;

            // Находим B численным методом и округляем до целого числа ошибок.
            double bExact = findB(n);
            int b = Math.max(n + 1, (int) Math.round(bExact));

            // Рассчитываем остальные параметры модели Джелинского-Моранды.
            double k = calculateK(b, n);
            double nextErrorTime = 1.0 / (k * (b - n));
            double testingTime = calculateRemainingTestingTime(b, n, k);

            System.out.println("Число обнаруженных ошибок n = " + n);
            System.out.printf("Решение уравнения B = %.6f%n", bExact);
            System.out.println("Оценка общего числа ошибок B = " + b);
            System.out.printf("Коэффициент K = %.6f%n", k);
            System.out.printf("Время до следующей ошибки = %.6f ч%n", nextErrorTime);
            System.out.printf("Время до окончания тестирования = %.6f ч%n", testingTime);
        } catch (FileNotFoundException e) {
            System.out.println("Файл не найден: " + fileName);
        } catch (IllegalArgumentException | IllegalStateException e) {
            System.out.println("Ошибка: " + e.getMessage());
        }
    }

    private static double[] readIntervals(String fileName) throws FileNotFoundException {
        List<Double> values = new ArrayList<>();

        // Считываем все числа из файла до его конца.
        try (Scanner scanner = new Scanner(new File(fileName))) {
            scanner.useLocale(Locale.US);
            while (scanner.hasNext()) {
                if (!scanner.hasNextDouble()) {
                    throw new IllegalArgumentException(
                            "в файле встретилось нечисловое значение: " + scanner.next());
                }

                double value = scanner.nextDouble();
                if (value <= 0) {
                    throw new IllegalArgumentException("все интервалы должны быть больше нуля");
                }
                values.add(value);
            }
        }

        if (values.isEmpty()) {
            throw new IllegalArgumentException("файл не содержит интервалов времени");
        }

        double[] result = new double[values.size()];
        for (int i = 0; i < values.size(); i++) {
            result[i] = values.get(i);
        }
        return result;
    }

    private static double findB(int n) {
        double left = n + 1e-7;
        double right = n + 1.0;

        // Расширяем границы, пока значения функции не получат разные знаки.
        while (equation(left, n) * equation(right, n) > 0 && right < 1_000_000) {
            right *= 2;
        }

        if (equation(left, n) * equation(right, n) > 0) {
            throw new IllegalStateException("для введенных данных не удалось найти B");
        }

        // Уточняем корень уравнения методом бисекции.
        for (int iteration = 0; iteration < 200; iteration++) {
            double middle = (left + right) / 2.0;

            if (equation(left, n) * equation(middle, n) <= 0) {
                right = middle;
            } else {
                left = middle;
            }
        }

        return (left + right) / 2.0;
    }

    private static double equation(double b, int n) {
        double sumX = 0;
        double sumIX = 0;
        double leftPart = 0;

        // Вычисляем обе части нелинейного уравнения для B.
        for (int i = 1; i <= n; i++) {
            sumX += intervals[i - 1];
            sumIX += i * intervals[i - 1];
            leftPart += 1.0 / (b - i + 1);
        }

        double rightPart = n * sumX / ((b + 1) * sumX - sumIX);
        return leftPart - rightPart;
    }

    private static double calculateK(int b, int n) {
        double denominator = 0;

        // Знаменатель оценки максимального правдоподобия для K.
        for (int i = 1; i <= n; i++) {
            denominator += (b - i + 1) * intervals[i - 1];
        }

        return n / denominator;
    }

    private static double calculateRemainingTestingTime(int b, int n, double k) {
        double time = 0;

        // Суммируем ожидаемые интервалы до обнаружения оставшихся ошибок.
        for (int i = n + 1; i <= b; i++) {
            time += 1.0 / (k * (b - i + 1));
        }

        return time;
    }
}
