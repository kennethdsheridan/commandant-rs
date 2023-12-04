// StressTest trait is a port in the hexagonal architecture.
pub trait StressTest {
    fn run_cpu_tests(&self);
    // Other stress test methods can be added here
}