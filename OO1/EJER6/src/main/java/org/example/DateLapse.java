package org.example;

import java.time.LocalDate;

public class DateLapse {

    private LocalDate from;
    private LocalDate to;

    public void setFrom(LocalDate from) {
        this.from = from;
    }

    public void setTo(LocalDate to) {
        this.to = to;
    }

    public DateLapse() {

    }

    public LocalDate getFrom(){
        return this.from;
    }
    public LocalDate getTo(){
        return this.to;
    }

    public int sizeInDays() {
        if (this.from.isBefore(this.to) || this.from.equals(this.to)) {
            long dias = this.to.toEpochDay() - this.from.toEpochDay();
            return Math.toIntExact(dias);
        }
        return -1;
    }

    public boolean includesData(LocalDate other){
        return this.from.isBefore(other) && this.to.isAfter(other);
    }
}
