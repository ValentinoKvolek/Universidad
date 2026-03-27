package org.example;

public class FifoScheluding extends JobScheduler {


    public  JobDescription next(){
        JobDescription nextJob = null;
        nextJob = jobs.get(0);
        this.unschedule(nextJob);
        return nextJob;
    }
}
