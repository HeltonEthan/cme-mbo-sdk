# TODOS.md

**Dataset**
- [x] Parse through directory folder & collect the correct files
- [x] Decode dbn files & stream them
- [x] Implment a callback into the live stream that recieves an mbo_msg
- [x] Give the orderbook each mbo_msg
- [x] Implement threading across files

**Order book:**
- [x] Implement order book reconstruction
- [] Implement requested orders into live order book

**Exchange**
- [] Implement order creation
- [] Implement FIFO matching algorithm
- [] Implement ACK responses
- [] Implement dynamic modification
- [] Implement trading session calendar
- [x] Implement latency
