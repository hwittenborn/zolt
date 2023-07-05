pub use colored;

zolt_proc::msg_macros!(info, "general", "Info", cyan, print);
zolt_proc::msg_macros!(warn, "warning", "Warning", bright_yellow, eprint);
zolt_proc::msg_macros!(err, "error", "Error", red, eprint);
zolt_proc::msg_macros!(debug, "debug", "Debug", green, eprint);
