package org.example;

import java.time.LocalDate;

public class DateLapse {

    private LocalDate from;
    private int sizeInDays;

    public DateLapse() {
    }

    public void setFrom(LocalDate from) {
        this.from = from;
    }

    public void setTo(LocalDate date){
        if (date.isBefore(this.from)) {
            this.sizeInDays = -1;
        } else {
            this.sizeInDays = Math.toIntExact(date.toEpochDay() - this.from.toEpochDay());
        }
    }


    public int sizeInDays() {
        if (this.getTo().isBefore(this.from)) {
            return -1;
        }
        return Math.toIntExact(this.getTo().toEpochDay() - this.from.toEpochDay());
    }


    public LocalDate getFrom(){
        return this.from;
    }

    public LocalDate getTo() {
        return this.from.plusDays(Math.max(0, this.sizeInDays));
    }


    public boolean includesDate(LocalDate other){
        return (!other.isBefore(this.from) && !other.isAfter(this.getTo()));
    }
}
