# Explain STM32

Rust learning project. CLI tool which explains STM32 device names.

Example:

STM32F103C8T6

Family: STM32
Series: F1 (preformance line)
Sub-family: 103
Pin count: C (48-pins)
Flash memory size: 8 (64kB)
Package: T (LQFP)
Temperature range: 6 (Industrial, -40 to 85)

## F0 devices

 - _Name (RAM, FLASH, package)_
 - STM32F030F4 (4kB, 16kB, 20-pin TSSOP)
 - STM32F030K6 (4kB, 32kB, 32-pin LQFP)
 - STM32F030C6 (4kB, 32kB, 48-pin LQFP)
 - STM32F070F6 (6kB, 32kB, 20-pin TSSOP) - With USB interface
 - STM32F070C6 (6kB, 32kB, 48-pin LQFP) - With USB interface
 - STM32F030C8 (8kB, 64kB, 48-pin LQFP)
 - STM32F030R8 (8kB, 64kB, 64-pin LQFP)
 - STM32F070CB (16kB, 128kB, 48-pin LQFP) - With USB interface
 - STM32F070RB (16kB, 128kB, 64-pin LQFP) - With USB interface
 - STM32F030CC (32kB, 256kB, 48-pin LQFP)
 - STM32F030RC (32kB, 256kB, 64-pin LQFP)

Package codes:

 - F = 20-pin TSSOP
 - K = 32-pin LQFP
 - C = 48-pin LQFP
 - R = 64-pin LQFP

Memory size codes:

 - 4 = 4kB, 16kB
 - 6 = 4kB, 32kB
 - 6 = 6kB, 32kB (on 070)
 - 8 = 8kB, 64kB
 - B = 16kB, 128kB
 - C = 32kB, 256kB
