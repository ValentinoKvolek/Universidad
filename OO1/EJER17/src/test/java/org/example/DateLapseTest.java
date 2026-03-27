package org.example;

import org.junit.jupiter.api.Test;
import java.time.LocalDate;
import static org.junit.jupiter.api.Assertions.*;

class DateLapseTest {

    @Test
    void testSizeInDaysNormal() {
        DateLapse d = new DateLapse();
        d.setFrom(LocalDate.of(2024, 1, 1));
        d.setTo(LocalDate.of(2024, 1, 2));
        assertEquals(1, d.sizeInDays());
    }

    @Test
    void testSizeInDaysSameDate() {
        DateLapse d = new DateLapse();
        d.setFrom(LocalDate.of(2024, 1, 1));
        d.setTo(LocalDate.of(2024, 1, 1));
        assertEquals(0, d.sizeInDays());
    }

    @Test
    void testSizeInDaysInvalidRange() {
        DateLapse d = new DateLapse();
        d.setFrom( LocalDate.of(2024, 1, 10));
        d.setTo(LocalDate.of(2024, 1, 5));
        assertEquals(-1, d.sizeInDays());
    }

    @Test
    void testIncludesDateInside() {
        DateLapse d = new DateLapse();
        d.setFrom(LocalDate.of(2024, 1, 1));
        d.setTo(LocalDate.of(2024, 1, 10));
        assertTrue(d.includesDate(LocalDate.of(2024, 1, 5)));
    }

    @Test
    void testIncludesDateOutside() {
        DateLapse d = new DateLapse();
        d.setFrom(LocalDate.of(2024, 1, 1));
        d.setTo(LocalDate.of(2024, 1, 10));
        assertFalse(d.includesDate(LocalDate.of(2024, 1, 11)));
    }
}
