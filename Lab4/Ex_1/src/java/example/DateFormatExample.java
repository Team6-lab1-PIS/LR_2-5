package example;

import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.util.Date;

public class DateFormatExample {
    private static final DateFormat format = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");

    public static String getDate() {
        return format.format(new Date());
    }
}