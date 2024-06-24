#!/bin/bash

# Set the log file name
LOGFILE="$HOME/perftest/logs/ib_write_bw_server.log"

# Create log directory if it doesn't exist
mkdir -p $(dirname $LOGFILE)

# List of NIC devices
NIC_DEVICES=("192.168.42.0" "192.168.42.1" "192.168.42.2" "192.168.42.3" "192.168.42.4" "192.168.42.5" "192.168.42.6" "192.168.42.7")

# Function to run the command and log the output
run_server_test() {
    local nic_device=$1
    echo "Running ib_write_bw server test at $(date) for NIC $nic_device" | tee -a $LOGFILE
    $HOME/perftest/bin/ib_write_bw -d bnxt_re0 --use_rocm=1 -a -F -x 3 -q 4 --report_gbits -b --bind_source_ip $nic_device 2>&1 | tee -a $LOGFILE
    echo "Server test completed at $(date) for NIC $nic_device" | tee -a $LOGFILE
    echo "----------------------------------------" | tee -a $LOGFILE
}

# Loop to run the test for all NIC devices
while true; do
    for nic in "${NIC_DEVICES[@]}"; do
        run_server_test $nic
        # Wait time between tests (in seconds)
        sleep 10
    done
done

