# ARP
Address resolution protocol

## ARP broadcast
Broadcast to every device on the network asking a specific computer to tell its MAC address

## Example ARP packet
```
sudo tshark -i en0 -f "arp" -c 1 -x
```

```
Capturing on 'Wi-Fi: en0'
0000  ff ff ff ff ff ff 80 48 2c 1c 93 92 08 06 00 01   .......H,.......
0010  08 00 06 04 00 01 80 48 2c 1c 93 92 c0 a8 01 41   .......H,......A
0020  00 00 00 00 00 00 c0 a8 01 41                     .........A

1 packet captured
```

1. Destination MAC address is 
```
ff ff ff ff ff ff
```
for broadcast

2. Source MAC address is 
```
80 48 2c 1c 93 92
```
 