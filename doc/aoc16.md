# AoC 2019 #16

## Part B

I think the way out of this is to note that the message offset, at the start, is
very large. The first seven digits are 5_976_463. The input is 650 bytes
repeated 10k times, so the expanded input is 6_650_000 bytes long. So in fact
perhaps it doesn't wrap around many times...