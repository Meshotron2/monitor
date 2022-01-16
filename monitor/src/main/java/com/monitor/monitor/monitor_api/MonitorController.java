package com.monitor.monitor.monitor_api;

import org.springframework.web.bind.annotation.PostMapping;

public class MonitorController {
    @PostMapping(path = "/node")
    public void postNode(int pid, double cpu, int ram, int progress){

    }

    @PostMapping(path = "/thread")
    public void postThread(int cores, int threads, double cpu_usage, int tram, int uram, double temperature[]){

    }
}
