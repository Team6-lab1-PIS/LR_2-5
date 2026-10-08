package example;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;

import java.math.BigDecimal;

public class BigDecimalExample {
    @SuppressFBWarnings("SPP_USE_BIGDECIMAL_STRING_CTOR") // Аннотация для скрытия ошибки SPP_USE_BIGDECIMAL_STRING_CTOR расшерения fb-contrib
    public static void run() {
        // Пример 1. Возникновение ошибки из-за формата типа double.
        // Невозможно точно представить 1.1 в двоичной системе счисления.
        // В конструктор передается значение 1.100000000000000088817841970012523233890533447265625
        System.out.println(new BigDecimal(1.1));

        // Пример 2. Метод .equals() сравнивает не только числовое значение, но и масштаб (scale) — количество знаков после запятой.
        // У d1 scale равен 1. У d2 scale равен 2.
        // Результатом будет false
        BigDecimal d1 = new BigDecimal("1.1");
        BigDecimal d2 = new BigDecimal("1.10");
        System.out.println(d1.equals(d2));
    }
}