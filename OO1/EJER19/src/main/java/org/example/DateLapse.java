package org.example;

import java.time.LocalDate;

public class DateLapse {

    private LocalDate from;
    private int sizeInDays;

    public DateLapse() {
    }

    public DateLapse(LocalDate desde, LocalDate hasta) {
        if (hasta.isBefore(desde)) {
            throw new IllegalArgumentException("La fecha final no puede ser anterior a la inicial");
        }
        this.from = desde;
        this.sizeInDays = Math.toIntExact(hasta.toEpochDay() - desde.toEpochDay());
    }


    public int sizeInDays() {
        return this.sizeInDays;
    }

    public LocalDate getFrom() {
        return this.from;
    }

    public LocalDate getTo() {
        return this.from.plusDays(this.sizeInDays);
    }

    public boolean includesDate(LocalDate other) {
        return (!other.isBefore(this.from) && !other.isAfter(this.getTo()));
    }

    public boolean overlaps(DateLapse otroPeriodo){
        return otroPeriodo.includesDate(this.getTo()) || otroPeriodo.includesDate(this.getFrom())
                || this.includesDate(otroPeriodo.getTo()) || this.includesDate(otroPeriodo.getFrom());
    }
}
