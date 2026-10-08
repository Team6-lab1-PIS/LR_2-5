package example;

public class TernaryConditionalExample {
    public static void run() {
        // Установка необходимых флагов
        boolean flag = true;
        boolean flag1 = false;
        boolean flag2 = false;

        // Пример 1. Автоматическая распаковка обёртки и приведение типов внутри тернарного оператора.
        // Видоизменен из-за невозможности использования deprecated конструктора в JDK 9 и выше

        // Number n = flag ? new Integer(1) : new Double(2.0);
        Integer valInteger = Integer.valueOf(1);
        double n = flag ? valInteger : 2.0;

        // Пример 2. Высокий риск получить NullPointerException.
        int n1 = flag1 ? 1 : flag2 ? 2 : null;

        // Переменная n1 не используется, поэтому анализатор так же подсветит эту ошибку.
        System.out.println(n);
    }

    // Пример 3. Высокий риск получить NullPointerException.
    double getVal(int idx) {
        double[] vals = new double[]{1.0, 2.0, 3.0};
        return (idx < 0 || idx >= vals.length) ? null : vals[idx];
    }
}