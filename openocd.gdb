
target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind
# # run the next few lines so the panic message is printed immediately
# # the number needs to be adjusted for your panic handler
# commands $bpnum
# next 4
# end

# *try* to stop at the user entry point (it might be gone due to inlining)
break main
monitor arm semihosting enable
monitor rtt stop
monitor rtt server stop 9090
monitor rtt server start 9090 0

#run this command to get address and size : rust-nm -S target/thumbv7em-none-eabihf/debug/rtt_prints|grep RTT|awk '{print $1}'
monitor rtt setup 0x20000000 0x00000030 "SEGGER RTT"
monitor rtt start


load

# start the process but immediately halt the processor
stepi
