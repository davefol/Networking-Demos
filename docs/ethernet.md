# Ethernet 

## Ethernet Frame
Pre-amble: 64 bytes ending in 11

Frame has to be minimum of 46 bytes

1. destination MAC (6 bytes)
2. source MAC (6 bytes)
3. type (2 bytes)
4. data (46-1500 bytes)
5. FCS (4 bytes)