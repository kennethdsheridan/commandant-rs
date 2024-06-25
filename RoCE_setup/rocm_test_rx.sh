#!/bin/bash

# Set the log file name
LOGFILE="$HOME/perftest/logs/ib_write_bw_server.log"

# Create log directory if it doesn't exist
mkdir -p $(dirname $LOGFILE)

# List of RDMA devices
RDMA_DEVICES=("bnxt_re0" "bnxt_re1" "bnxt_re2" "bnxt_re3" "bnxt_re6" "bnxt_re7" "bnxt_re8" "bnxt_re9")

# List of ROCm devices
ROCM_DEVICES=(0 1 2 3 4 5 6 7)

# Function to run the command and log the output
run_server_test() {
    local rdma_device=$1
    local rocm_device=$2
    echo "Running ib_write_bw server test at $(date) for RDMA device $rdma_device and ROCm device $rocm_device" | tee -a $LOGFILE
    $HOME/perftest/bin/ib_write_bw -d $rdma_device --use_rocm=$rocm_device -a -F -x 3 -q 4 --report_gbits -b 2>&1 | tee -a $LOGFILE
    echo "Server test completed at $(date) for RDMA device $rdma_device and ROCm device $rocm_device" | tee -a $LOGFILE
    echo "----------------------------------------" | tee -a $LOGFILE
}

# Loop to run the test for all combinations of RDMA and ROCm devices
while true; do
    for rdma in "${RDMA_DEVICES[@]}"; do
        for rocm in "${ROCM_DEVICES[@]}"; do
            run_server_test $rdma $rocm
            # Wait time between tests (in seconds)
            sleep 10
        done
    done
done

