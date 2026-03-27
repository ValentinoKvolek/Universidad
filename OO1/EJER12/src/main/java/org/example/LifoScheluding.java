package org.example;

public class LifoScheluding extends JobScheduler {

    public  JobDescription next() {
        JobDescription nextJob = null;
        nextJob = jobs.get(jobs.size() - 1);
        this.unschedule(nextJob);
        return nextJob;
    }
}
