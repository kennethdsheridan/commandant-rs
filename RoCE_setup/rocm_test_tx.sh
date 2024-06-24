#!/bin/bash

# Set the log file name
LOGFILE="$HOME/perftest/logs/ib_write_bw_client.log"

# Create log directory if it doesn't exist
mkdir -p $(dirname $LOGFILE)

# List of GPU devices
GPU_DEVICES=("bnxt_re0" "bnxt_re1" "bnxt_re2" "bnxt_re3" "bnxt_re4" "bnxt_re5" "bnxt_re6" "bnxt_re7")

# List of NIC devices
NIC_DEVICES=("192.168.42.0" "192.168.42.1" "192.168.42.2" "192.168.42.3" "192.168.42.4" "192.168.42.5" "192.168.42.6" "192.168.42.7")

# Function to run the command and log the output
run_client_test() {
    local gpu_device=$1
    local nic_device=$2
    echo "Running ib_write_bw client test at $(date) for GPU $gpu_device and NIC $nic_device" | tee -a $LOGFILE
    $HOME/perftest/bin/ib_write_bw -d $gpu_device --use_rocm=0 -a -F -x 3 -q 4 --report_gbits -b --bind_source_ip $nic_device 192.168.42.2 2>&1 | tee -a $LOGFILE
    echo "Client test completed at $(date) for GPU $gpu_device and NIC $nic_device" | tee -a $LOGFILE
    echo "----------------------------------------" | tee -a $LOGFILE
}

# Loop to run the test for all combinations of GPU and NIC devices
while true; do
    for gpu in "${GPU_DEVICES[@]}"; do
        for nic in "${NIC_DEVICES[@]}"; do
            run_client_test $gpu $nic
            # Wait time between tests (in seconds)
            sleep 10
        done
    done
done

