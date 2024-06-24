#!/bin/bash

# Set the log file name
LOGFILE="$HOME/perftest/logs/ib_write_bw.log"

# Create log directory if it doesn't exist
mkdir -p $(dirname $LOGFILE)

# Function to run the command and log the output
run_test() {
    echo "Running ib_write_bw test at $(date)" | tee -a $LOGFILE
    $HOME/perftest/bin/ib_write_bw -d bnxt_re0 --use_rocm=1 -a -F -x 3 -q 4 --report_gbits -b 2>&1 | tee -a $LOGFILE
    echo "Test completed at $(date)" | tee -a $LOGFILE
    echo "----------------------------------------" | tee -a $LOGFILE
}

# Loop to run the test continuously
while true; do
    run_test
    # Wait time between tests (in seconds)
    sleep 10
done

