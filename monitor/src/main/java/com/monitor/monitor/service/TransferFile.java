package com.monitor.monitor.service;

public interface TransferFile {
    
    boolean downloadFile(String localFilePath, String removeFilePath);

    boolean uploadFile(String localFilePath, String removeFilePath);
}
