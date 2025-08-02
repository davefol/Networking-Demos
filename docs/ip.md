# IP

Capturing one IP packet and inspecting it
```
sudo tshark -i en0 -f "ip" -c 1 -x
```

```
Capturing on 'Wi-Fi: en0'
0000  12 54 10 9a f5 a1 74 24 9f a0 1d 4f 08 00 45 88   .T....t$...O..E.
0010  00 5e ac 27 40 00 39 06 a6 d1 a2 9f 88 ea c0 a8   .^.'@.9.........
0020  01 e7 01 bb d7 a8 f7 be 13 38 3f c2 41 af 80 18   .........8?.A...
0030  00 10 88 2b 00 00 01 01 08 0a b0 8e ce 8e cb 5d   ...+...........]
0040  17 6a 17 03 03 00 25 8d 68 20 18 bd b7 73 34 60   .j....%.h ...s4`
0050  7b 6f 5f 9f 3f 99 5c 81 ca e5 61 c8 cc 98 19 18   {o_.?.\...a.....
0060  0a 4e ec d9 a4 89 86 40 9b a6 45 22               .N.....@..E"
```