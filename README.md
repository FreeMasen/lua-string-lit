# Lua String Lit

A tool that will scan a directory for any `.lua` files and collect all string literals they contain.
The output is a map keyed with the file name where the value is a list of objects containing the
literal value and the raw byte offset from the file.

## Example Output

The following is example output from the
[cosock](https://github.com/cosock/cosock/tree/f20c6ca130c7dcbe2ad56404a3c45cb6af6880bb) library

```json
{
  "/home/rfm/projects/cosock/cosock.lua": [
    {
      "value": "\"cosock.socket\"",
      "start_offset": 23,
      "end_offset": 38
    },
    {
      "value": "\"socket\"",
      "start_offset": 68,
      "end_offset": 76
    },
    {
      "value": "\"cosock.channel\"",
      "start_offset": 101,
      "end_offset": 117
    },
    {
      "value": "\"cosock.ssl\"",
      "start_offset": 138,
      "end_offset": 150
    },
    {
      "value": "\"vvvvvvvvvvvvvvvvvvvvvvvv DUMP STATE vvvvvvvvvvvvvvvvvvvvvvvv\"",
      "start_offset": 1104,
      "end_offset": 1166
    },
    {
      "value": "\"debug.traceback not avaliable\"",
      "start_offset": 1233,
      "end_offset": 1264
    },
    {
      "value": "\"threads woken in last turn (\"",
      "start_offset": 1284,
      "end_offset": 1314
    },
    {
      "value": "\")\"",
      "start_offset": 1352,
      "end_offset": 1355
    },
    {
      "value": "\"=========================================================\"",
      "start_offset": 1371,
      "end_offset": 1430
    },
    {
      "value": "\"recvt:\"",
      "start_offset": 1563,
      "end_offset": 1571
    },
    {
      "value": "\"sendt:\"",
      "start_offset": 1639,
      "end_offset": 1647
    },
    {
      "value": "\"timeout:\"",
      "start_offset": 1715,
      "end_offset": 1725
    },
    {
      "value": "\"---------------------------------------------------------\"",
      "start_offset": 1786,
      "end_offset": 1845
    },
    {
      "value": "\"threads not woken in last turn (\"",
      "start_offset": 1867,
      "end_offset": 1901
    },
    {
      "value": "\")\"",
      "start_offset": 1961,
      "end_offset": 1964
    },
    {
      "value": "\"=========================================================\"",
      "start_offset": 1980,
      "end_offset": 2039
    },
    {
      "value": "\"recvt:\"",
      "start_offset": 2210,
      "end_offset": 2218
    },
    {
      "value": "\"sendt:\"",
      "start_offset": 2288,
      "end_offset": 2296
    },
    {
      "value": "\"timeout:\"",
      "start_offset": 2366,
      "end_offset": 2376
    },
    {
      "value": "\"---------------------------------------------------------\"",
      "start_offset": 2439,
      "end_offset": 2498
    },
    {
      "value": "\"^^^^^^^^^^^^^^^^^^^^^^^^ DUMP STATE ^^^^^^^^^^^^^^^^^^^^^^^^\"",
      "start_offset": 2529,
      "end_offset": 2591
    },
    {
      "value": "\"0.2.0\"",
      "start_offset": 2696,
      "end_offset": 2703
    },
    {
      "value": "\"timer set: %s,%s\"",
      "start_offset": 3002,
      "end_offset": 3020
    },
    {
      "value": "\"timeout time\"",
      "start_offset": 4459,
      "end_offset": 4473
    },
    {
      "value": "\"earliest timeout\"",
      "start_offset": 4558,
      "end_offset": 4576
    },
    {
      "value": "\"socket\"",
      "start_offset": 4963,
      "end_offset": 4971
    },
    {
      "value": "\"cosock.socket\"",
      "start_offset": 4992,
      "end_offset": 5007
    },
    {
      "value": "\"ssl\"",
      "start_offset": 5027,
      "end_offset": 5032
    },
    {
      "value": "\"cosock.ssl\"",
      "start_offset": 5053,
      "end_offset": 5065
    },
    {
      "value": "\"socket\"",
      "start_offset": 5297,
      "end_offset": 5305
    },
    {
      "value": "\"cosock.socket\"",
      "start_offset": 5318,
      "end_offset": 5333
    },
    {
      "value": "\"ssl\"",
      "start_offset": 5353,
      "end_offset": 5358
    },
    {
      "value": "\"cosock.ssl\"",
      "start_offset": 5371,
      "end_offset": 5383
    },
    {
      "value": "\"function\"",
      "start_offset": 5587,
      "end_offset": 5597
    },
    {
      "value": "\"_ENV\"",
      "start_offset": 5874,
      "end_offset": 5880
    },
    {
      "value": "\"socket spawn\"",
      "start_offset": 6678,
      "end_offset": 6692
    },
    {
      "value": "\"wake thread\"",
      "start_offset": 6863,
      "end_offset": 6876
    },
    {
      "value": "\"wake thread err\"",
      "start_offset": 7108,
      "end_offset": 7125
    },
    {
      "value": "\"================= %s ======================\"",
      "start_offset": 7933,
      "end_offset": 7978
    },
    {
      "value": "\"waking\"",
      "start_offset": 8747,
      "end_offset": 8755
    },
    {
      "value": "\"suspended\"",
      "start_offset": 8865,
      "end_offset": 8876
    },
    {
      "value": "\"timeout\"",
      "start_offset": 9078,
      "end_offset": 9087
    },
    {
      "value": "\"non-wakeable socket\"",
      "start_offset": 9172,
      "end_offset": 9193
    },
    {
      "value": "\"unset waker\"",
      "start_offset": 9215,
      "end_offset": 9228
    },
    {
      "value": "\"suspended\"",
      "start_offset": 9539,
      "end_offset": 9550
    },
    {
      "value": "\"suspending\"",
      "start_offset": 9621,
      "end_offset": 9633
    },
    {
      "value": "\"timeout\"",
      "start_offset": 10008,
      "end_offset": 10017
    },
    {
      "value": "\"non-wakeable socket\"",
      "start_offset": 10106,
      "end_offset": 10127
    },
    {
      "value": "\"set waker\"",
      "start_offset": 10151,
      "end_offset": 10162
    },
    {
      "value": "\"timeout\"",
      "start_offset": 10604,
      "end_offset": 10613
    },
    {
      "value": "\"dead\"",
      "start_offset": 10685,
      "end_offset": 10691
    },
    {
      "value": "\"dead\"",
      "start_offset": 11004,
      "end_offset": 11010
    },
    {
      "value": "\"non-suspended thread encountered\"",
      "start_offset": 11182,
      "end_offset": 11216
    },
    {
      "value": "\"thread\"",
      "start_offset": 11415,
      "end_offset": 11423
    },
    {
      "value": "\"dead\"",
      "start_offset": 11519,
      "end_offset": 11525
    },
    {
      "value": "\"thread for recvt:\"",
      "start_offset": 11881,
      "end_offset": 11900
    },
    {
      "value": "\"thread for sendt:\"",
      "start_offset": 12192,
      "end_offset": 12211
    },
    {
      "value": "\"thread woken during execution of other threads, no timeout\"",
      "start_offset": 12510,
      "end_offset": 12570
    },
    {
      "value": "\"cosock tried to call socket.select with no sockets and no timeout. \"",
      "start_offset": 12729,
      "end_offset": 12798
    },
    {
      "value": "\"this is a bug, please report it, including the above dump state\"",
      "start_offset": 12813,
      "end_offset": 12878
    },
    {
      "value": "\"start select\"",
      "start_offset": 12899,
      "end_offset": 12913
    },
    {
      "value": "\"return select\"",
      "start_offset": 13131,
      "end_offset": 13146
    },
    {
      "value": "\"timeout\"",
      "start_offset": 13203,
      "end_offset": 13212
    },
    {
      "value": "\"unknown socket\"",
      "start_offset": 13377,
      "end_offset": 13393
    },
    {
      "value": "\"unwakeable socket\"",
      "start_offset": 13419,
      "end_offset": 13438
    },
    {
      "value": "\"recvr\"",
      "start_offset": 13456,
      "end_offset": 13463
    },
    {
      "value": "\"unknown socket\"",
      "start_offset": 13614,
      "end_offset": 13630
    },
    {
      "value": "\"unwakeable socket\"",
      "start_offset": 13656,
      "end_offset": 13675
    },
    {
      "value": "\"sendr\"",
      "start_offset": 13693,
      "end_offset": 13700
    },
    {
      "value": "\"run exit\"",
      "start_offset": 13725,
      "end_offset": 13735
    }
  ],
  "/home/rfm/projects/cosock/cosock/ssl/https.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 15,
      "end_offset": 23
    },
    {
      "value": "\"ssl.https\"",
      "start_offset": 34,
      "end_offset": 45
    }
  ],
  "/home/rfm/projects/cosock/test/asyncify/asyncify-works.lua": [
    {
      "value": "\"Lua 5.1\"",
      "start_offset": 68,
      "end_offset": 77
    },
    {
      "value": "\"cosock\"",
      "start_offset": 121,
      "end_offset": 129
    },
    {
      "value": "\"socket\"",
      "start_offset": 220,
      "end_offset": 228
    },
    {
      "value": "\"test.asyncify.one\"",
      "start_offset": 255,
      "end_offset": 274
    },
    {
      "value": "\"test.asyncify.two\"",
      "start_offset": 301,
      "end_offset": 320
    },
    {
      "value": "\"Trying %s\"",
      "start_offset": 387,
      "end_offset": 398
    },
    {
      "value": "\"string\"",
      "start_offset": 498,
      "end_offset": 506
    },
    {
      "value": "\"Expected table found string for %s: %s\"",
      "start_offset": 508,
      "end_offset": 548
    },
    {
      "value": "\"require produced the same result as asyncify for\"",
      "start_offset": 646,
      "end_offset": 696
    },
    {
      "value": "\"selected async w/o error for\"",
      "start_offset": 856,
      "end_offset": 886
    },
    {
      "value": "\"string\"",
      "start_offset": 918,
      "end_offset": 926
    },
    {
      "value": "\"No error for\"",
      "start_offset": 928,
      "end_offset": 942
    },
    {
      "value": "\"attempt to yield from outside a coroutine\"",
      "start_offset": 971,
      "end_offset": 1014
    },
    {
      "value": "\"bad async error for %s: %s\"",
      "start_offset": 1039,
      "end_offset": 1067
    },
    {
      "value": "\"selected sync read for\"",
      "start_offset": 1227,
      "end_offset": 1251
    },
    {
      "value": "\"selected sync write for\"",
      "start_offset": 1284,
      "end_offset": 1309
    },
    {
      "value": "\"timeout\"",
      "start_offset": 1340,
      "end_offset": 1349
    },
    {
      "value": "\"bad sync erorr for\"",
      "start_offset": 1351,
      "end_offset": 1371
    },
    {
      "value": "\"socket\"",
      "start_offset": 1443,
      "end_offset": 1451
    },
    {
      "value": "\"direct\"",
      "start_offset": 1481,
      "end_offset": 1489
    },
    {
      "value": "\"test.asyncify.one\"",
      "start_offset": 1571,
      "end_offset": 1590
    },
    {
      "value": "\"non-nested\"",
      "start_offset": 1618,
      "end_offset": 1630
    },
    {
      "value": "\"test.asyncify.two\"",
      "start_offset": 1738,
      "end_offset": 1757
    },
    {
      "value": "\"nested\"",
      "start_offset": 1785,
      "end_offset": 1793
    },
    {
      "value": "\"table\"",
      "start_offset": 1877,
      "end_offset": 1884
    },
    {
      "value": "\"table\"",
      "start_offset": 1912,
      "end_offset": 1919
    },
    {
      "value": "\"%s\"",
      "start_offset": 1951,
      "end_offset": 1955
    },
    {
      "value": "\"test.asyncify.nested.table\"",
      "start_offset": 1990,
      "end_offset": 2018
    },
    {
      "value": "\"%s\"",
      "start_offset": 2051,
      "end_offset": 2055
    }
  ],
  "/home/rfm/projects/cosock/cosock/socket.lua": [
    {
      "value": "\"socket\"",
      "start_offset": 26,
      "end_offset": 34
    },
    {
      "value": "\"cosock.socket.tcp\"",
      "start_offset": 63,
      "end_offset": 82
    },
    {
      "value": "\"cosock.socket.udp\"",
      "start_offset": 111,
      "end_offset": 130
    },
    {
      "value": "\"reuseaddr\"",
      "start_offset": 817,
      "end_offset": 828
    },
    {
      "value": "\"function\"",
      "start_offset": 2128,
      "end_offset": 2138
    },
    {
      "value": "\"table\"",
      "start_offset": 2400,
      "end_offset": 2407
    }
  ],
  "/home/rfm/projects/cosock/test/tcp/client-multi.lua": [
    {
      "value": "\"----------------------------------------\"",
      "start_offset": 6,
      "end_offset": 48
    },
    {
      "value": "\"cosock\"",
      "start_offset": 73,
      "end_offset": 81
    },
    {
      "value": "\"require something\"",
      "start_offset": 98,
      "end_offset": 117
    },
    {
      "value": "\"table\"",
      "start_offset": 142,
      "end_offset": 149
    },
    {
      "value": "\"cosock is table\"",
      "start_offset": 151,
      "end_offset": 168
    },
    {
      "value": "\"client running\"",
      "start_offset": 357,
      "end_offset": 373
    },
    {
      "value": "\"client connect\"",
      "start_offset": 427,
      "end_offset": 443
    },
    {
      "value": "\"connect: \"",
      "start_offset": 508,
      "end_offset": 519
    },
    {
      "value": "\"client send\"",
      "start_offset": 547,
      "end_offset": 560
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 573,
      "end_offset": 580
    },
    {
      "value": "\"client reveived:\"",
      "start_offset": 627,
      "end_offset": 645
    },
    {
      "value": "\"foo\"",
      "start_offset": 677,
      "end_offset": 682
    },
    {
      "value": "\"client exit\"",
      "start_offset": 767,
      "end_offset": 780
    },
    {
      "value": "\"client\"",
      "start_offset": 804,
      "end_offset": 812
    },
    {
      "value": "\"dclient running\"",
      "start_offset": 900,
      "end_offset": 917
    },
    {
      "value": "\"dclient connect\"",
      "start_offset": 1016,
      "end_offset": 1033
    },
    {
      "value": "\"connect: \"",
      "start_offset": 1106,
      "end_offset": 1117
    },
    {
      "value": "\"connect: \"",
      "start_offset": 1214,
      "end_offset": 1225
    },
    {
      "value": "\"dclient send\"",
      "start_offset": 1261,
      "end_offset": 1275
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 1289,
      "end_offset": 1296
    },
    {
      "value": "\"bar\\n\"",
      "start_offset": 1310,
      "end_offset": 1317
    },
    {
      "value": "\"baz\\n\"",
      "start_offset": 1331,
      "end_offset": 1338
    },
    {
      "value": "\"dclient call select\"",
      "start_offset": 1608,
      "end_offset": 1629
    },
    {
      "value": "\"dclient select ret\"",
      "start_offset": 1710,
      "end_offset": 1730
    },
    {
      "value": "\"nil recvr\"",
      "start_offset": 1798,
      "end_offset": 1809
    },
    {
      "value": "\"table\"",
      "start_offset": 1839,
      "end_offset": 1846
    },
    {
      "value": "\"non-table recvr\"",
      "start_offset": 1848,
      "end_offset": 1865
    },
    {
      "value": "\"empty recvr\"",
      "start_offset": 1892,
      "end_offset": 1905
    },
    {
      "value": "\"dclient received:\"",
      "start_offset": 2019,
      "end_offset": 2038
    },
    {
      "value": "\"wrong data, expected '%s', got '%s'\"",
      "start_offset": 2204,
      "end_offset": 2241
    },
    {
      "value": "\"@@@@@@@@@@@@@@@ dclient left#:\"",
      "start_offset": 2381,
      "end_offset": 2413
    },
    {
      "value": "\"dclient exit\"",
      "start_offset": 2503,
      "end_offset": 2517
    },
    {
      "value": "\"double client\"",
      "start_offset": 2526,
      "end_offset": 2541
    },
    {
      "value": "\"server running\"",
      "start_offset": 2580,
      "end_offset": 2596
    },
    {
      "value": "\"no sock\"",
      "start_offset": 2635,
      "end_offset": 2644
    },
    {
      "value": "\"127.0.0.1\"",
      "start_offset": 2663,
      "end_offset": 2674
    },
    {
      "value": "\"bind\"",
      "start_offset": 2680,
      "end_offset": 2686
    },
    {
      "value": "\"server spawn clients\"",
      "start_offset": 2747,
      "end_offset": 2769
    },
    {
      "value": "\"server accept\"",
      "start_offset": 2856,
      "end_offset": 2871
    },
    {
      "value": "\"accepted socket\"",
      "start_offset": 2912,
      "end_offset": 2929
    },
    {
      "value": "\"server spawn recv\"",
      "start_offset": 2941,
      "end_offset": 2960
    },
    {
      "value": "\"coserver recvive\"",
      "start_offset": 3002,
      "end_offset": 3020
    },
    {
      "value": "\"coserver received:\"",
      "start_offset": 3062,
      "end_offset": 3082
    },
    {
      "value": "\"\\n\"",
      "start_offset": 3128,
      "end_offset": 3132
    },
    {
      "value": "\"server \"",
      "start_offset": 3192,
      "end_offset": 3201
    },
    {
      "value": "\"listen server\"",
      "start_offset": 3217,
      "end_offset": 3232
    },
    {
      "value": "\"----------------- exit -----------------\"",
      "start_offset": 3255,
      "end_offset": 3297
    }
  ],
  "/home/rfm/projects/cosock/cosock/socket/udp.lua": [
    {
      "value": "\"socket\"",
      "start_offset": 24,
      "end_offset": 32
    },
    {
      "value": "\"cosock.socket.internals\"",
      "start_offset": 59,
      "end_offset": 84
    },
    {
      "value": "\"close\"",
      "start_offset": 632,
      "end_offset": 639
    },
    {
      "value": "\"dirty\"",
      "start_offset": 664,
      "end_offset": 671
    },
    {
      "value": "\"getfamily\"",
      "start_offset": 700,
      "end_offset": 711
    },
    {
      "value": "\"getfd\"",
      "start_offset": 736,
      "end_offset": 743
    },
    {
      "value": "\"getoption\"",
      "start_offset": 772,
      "end_offset": 783
    },
    {
      "value": "\"getpeername\"",
      "start_offset": 814,
      "end_offset": 827
    },
    {
      "value": "\"getsockname\"",
      "start_offset": 858,
      "end_offset": 871
    },
    {
      "value": "\"receive\"",
      "start_offset": 898,
      "end_offset": 907
    },
    {
      "value": "\"receivefrom\"",
      "start_offset": 938,
      "end_offset": 951
    },
    {
      "value": "\"send\"",
      "start_offset": 975,
      "end_offset": 981
    },
    {
      "value": "\"sendto\"",
      "start_offset": 1007,
      "end_offset": 1015
    },
    {
      "value": "\"setfd\"",
      "start_offset": 1040,
      "end_offset": 1047
    },
    {
      "value": "\"setoption\"",
      "start_offset": 1076,
      "end_offset": 1087
    },
    {
      "value": "\"setpeername\"",
      "start_offset": 1118,
      "end_offset": 1131
    },
    {
      "value": "\"setsockname\"",
      "start_offset": 1162,
      "end_offset": 1175
    }
  ],
  "/home/rfm/projects/cosock/test/http/https-via-ssl.lua": [
    {
      "value": "\"Lua 5.1\"",
      "start_offset": 68,
      "end_offset": 77
    },
    {
      "value": "\"cosock\"",
      "start_offset": 121,
      "end_offset": 129
    },
    {
      "value": "\"test.http.perform_test\"",
      "start_offset": 159,
      "end_offset": 183
    },
    {
      "value": "\"https-via-ssl\"",
      "start_offset": 198,
      "end_offset": 213
    },
    {
      "value": "\"https\"",
      "start_offset": 215,
      "end_offset": 222
    },
    {
      "value": "\"ssl.https\"",
      "start_offset": 240,
      "end_offset": 251
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 269,
      "end_offset": 311
    }
  ],
  "/home/rfm/projects/cosock/test/asyncify/nested/module.lua": [
    {
      "value": "\"socket\"",
      "start_offset": 15,
      "end_offset": 23
    }
  ],
  "/home/rfm/projects/cosock/test/http/perform_test.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "\"cosock.socket\"",
      "start_offset": 55,
      "end_offset": 70
    },
    {
      "value": "\"id\"",
      "start_offset": 188,
      "end_offset": 192
    },
    {
      "value": "\"%s://httpbin.org/delay/3\"",
      "start_offset": 334,
      "end_offset": 360
    },
    {
      "value": "\"request took %s seconds\"",
      "start_offset": 433,
      "end_offset": 458
    },
    {
      "value": "\"body\"",
      "start_offset": 493,
      "end_offset": 499
    },
    {
      "value": "\"status\"",
      "start_offset": 517,
      "end_offset": 525
    },
    {
      "value": "\"request failed\"",
      "start_offset": 561,
      "end_offset": 577
    },
    {
      "value": "\"start\"",
      "start_offset": 720,
      "end_offset": 727
    },
    {
      "value": "\"requests started\"",
      "start_offset": 944,
      "end_offset": 962
    },
    {
      "value": "\" not all requests (or too many) started\"",
      "start_offset": 1024,
      "end_offset": 1065
    },
    {
      "value": "\"requests finished\"",
      "start_offset": 1077,
      "end_offset": 1096
    },
    {
      "value": "\" some requests already finished\"",
      "start_offset": 1160,
      "end_offset": 1193
    },
    {
      "value": "\"checker\"",
      "start_offset": 1204,
      "end_offset": 1213
    },
    {
      "value": "\"-slow1\"",
      "start_offset": 1307,
      "end_offset": 1315
    },
    {
      "value": "\"-slow2\"",
      "start_offset": 1409,
      "end_offset": 1417
    },
    {
      "value": "\"-slow3\"",
      "start_offset": 1511,
      "end_offset": 1519
    },
    {
      "value": "\" requests finished\"",
      "start_offset": 1550,
      "end_offset": 1570
    },
    {
      "value": "\": not all requests (or too many) finished\"",
      "start_offset": 1632,
      "end_offset": 1675
    }
  ],
  "/home/rfm/projects/cosock/test/ssl/client-timeout.lua": [
    {
      "value": "\"----------------------------------------\"",
      "start_offset": 6,
      "end_offset": 48
    },
    {
      "value": "\"cosock\"",
      "start_offset": 73,
      "end_offset": 81
    },
    {
      "value": "\"require something\"",
      "start_offset": 98,
      "end_offset": 117
    },
    {
      "value": "\"table\"",
      "start_offset": 142,
      "end_offset": 149
    },
    {
      "value": "\"cosock is table\"",
      "start_offset": 151,
      "end_offset": 168
    },
    {
      "value": "\"nobl client running\"",
      "start_offset": 372,
      "end_offset": 393
    },
    {
      "value": "\"nobl client connect\"",
      "start_offset": 467,
      "end_offset": 488
    },
    {
      "value": "\"connect: \"",
      "start_offset": 564,
      "end_offset": 575
    },
    {
      "value": "\"client ssl wrap tcp\"",
      "start_offset": 990,
      "end_offset": 1011
    },
    {
      "value": "\"ssl wrap on client socket: \"",
      "start_offset": 1096,
      "end_offset": 1125
    },
    {
      "value": "\"client ssl dohandshake\"",
      "start_offset": 1162,
      "end_offset": 1186
    },
    {
      "value": "\"ssl handshake on client socket: \"",
      "start_offset": 1273,
      "end_offset": 1307
    },
    {
      "value": "\"nobl client send\"",
      "start_offset": 1349,
      "end_offset": 1367
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 1389,
      "end_offset": 1396
    },
    {
      "value": "\"nobl client reveived:\"",
      "start_offset": 1484,
      "end_offset": 1507
    },
    {
      "value": "\"wantread\"",
      "start_offset": 1540,
      "end_offset": 1550
    },
    {
      "value": "\"nobl client exit\"",
      "start_offset": 1564,
      "end_offset": 1582
    },
    {
      "value": "\"nobl client\"",
      "start_offset": 1621,
      "end_offset": 1634
    },
    {
      "value": "\"fast client running\"",
      "start_offset": 1745,
      "end_offset": 1766
    },
    {
      "value": "\"fast client connect\"",
      "start_offset": 1840,
      "end_offset": 1861
    },
    {
      "value": "\"connect: \"",
      "start_offset": 1937,
      "end_offset": 1948
    },
    {
      "value": "\"client ssl wrap tcp\"",
      "start_offset": 2363,
      "end_offset": 2384
    },
    {
      "value": "\"ssl wrap on client socket: \"",
      "start_offset": 2469,
      "end_offset": 2498
    },
    {
      "value": "\"client ssl dohandshake\"",
      "start_offset": 2535,
      "end_offset": 2559
    },
    {
      "value": "\"ssl handshake on client socket: \"",
      "start_offset": 2646,
      "end_offset": 2680
    },
    {
      "value": "\"fast client send\"",
      "start_offset": 2722,
      "end_offset": 2740
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 2762,
      "end_offset": 2769
    },
    {
      "value": "\"fast client reveived:\"",
      "start_offset": 2861,
      "end_offset": 2884
    },
    {
      "value": "\"wantread\"",
      "start_offset": 2917,
      "end_offset": 2927
    },
    {
      "value": "\"fast client exit\"",
      "start_offset": 2941,
      "end_offset": 2959
    },
    {
      "value": "\"fast client\"",
      "start_offset": 2998,
      "end_offset": 3011
    },
    {
      "value": "\"slow client running\"",
      "start_offset": 3117,
      "end_offset": 3138
    },
    {
      "value": "\"slow client connect\"",
      "start_offset": 3212,
      "end_offset": 3233
    },
    {
      "value": "\"connect: \"",
      "start_offset": 3309,
      "end_offset": 3320
    },
    {
      "value": "\"client ssl wrap tcp\"",
      "start_offset": 3735,
      "end_offset": 3756
    },
    {
      "value": "\"ssl wrap on client socket: \"",
      "start_offset": 3841,
      "end_offset": 3870
    },
    {
      "value": "\"client ssl dohandshake\"",
      "start_offset": 3907,
      "end_offset": 3931
    },
    {
      "value": "\"ssl handshake on client socket: \"",
      "start_offset": 4018,
      "end_offset": 4052
    },
    {
      "value": "\"slow client send\"",
      "start_offset": 4094,
      "end_offset": 4112
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 4134,
      "end_offset": 4141
    },
    {
      "value": "\"slow client reveived:\"",
      "start_offset": 4230,
      "end_offset": 4253
    },
    {
      "value": "\"foo\"",
      "start_offset": 4287,
      "end_offset": 4292
    },
    {
      "value": "\"slow client exit\"",
      "start_offset": 4379,
      "end_offset": 4397
    },
    {
      "value": "\"slow client\"",
      "start_offset": 4436,
      "end_offset": 4449
    },
    {
      "value": "\"server running\"",
      "start_offset": 4496,
      "end_offset": 4512
    },
    {
      "value": "\"no sock\"",
      "start_offset": 4565,
      "end_offset": 4574
    },
    {
      "value": "\"127.0.0.1\"",
      "start_offset": 4600,
      "end_offset": 4611
    },
    {
      "value": "\"bind\"",
      "start_offset": 4617,
      "end_offset": 4623
    },
    {
      "value": "\"server spawn clients\"",
      "start_offset": 4700,
      "end_offset": 4722
    },
    {
      "value": "\"server accept\"",
      "start_offset": 5198,
      "end_offset": 5213
    },
    {
      "value": "\"accepted socket\"",
      "start_offset": 5277,
      "end_offset": 5294
    },
    {
      "value": "\"server spawn recv\"",
      "start_offset": 5308,
      "end_offset": 5327
    },
    {
      "value": "\"server ssl wrap accepted socket\"",
      "start_offset": 5342,
      "end_offset": 5375
    },
    {
      "value": "\"ssl wrap on accepted socket: \"",
      "start_offset": 5460,
      "end_offset": 5491
    },
    {
      "value": "\"server ssl dohandshake\"",
      "start_offset": 5528,
      "end_offset": 5552
    },
    {
      "value": "\"ssl handshake on accepted socket: \"",
      "start_offset": 5626,
      "end_offset": 5662
    },
    {
      "value": "\"coserver recvive\"",
      "start_offset": 5737,
      "end_offset": 5755
    },
    {
      "value": "\"coserver received:\"",
      "start_offset": 5812,
      "end_offset": 5832
    },
    {
      "value": "\"\\n\"",
      "start_offset": 5960,
      "end_offset": 5964
    },
    {
      "value": "\"server \"",
      "start_offset": 6063,
      "end_offset": 6072
    },
    {
      "value": "\"listen server\"",
      "start_offset": 6103,
      "end_offset": 6118
    },
    {
      "value": "\"----------------- exit -----------------\"",
      "start_offset": 6142,
      "end_offset": 6184
    }
  ],
  "/home/rfm/projects/cosock/test/channel/recv-timesout.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "\"Expected no message on receieve, got \"",
      "start_offset": 168,
      "end_offset": 207
    },
    {
      "value": "\"timeout\"",
      "start_offset": 242,
      "end_offset": 251
    },
    {
      "value": "\"expected err on receive to be `timeout` found \"",
      "start_offset": 253,
      "end_offset": 301
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 345,
      "end_offset": 387
    }
  ],
  "/home/rfm/projects/cosock/test/tcp/client-timeout.lua": [
    {
      "value": "\"----------------------------------------\"",
      "start_offset": 6,
      "end_offset": 48
    },
    {
      "value": "\"cosock\"",
      "start_offset": 73,
      "end_offset": 81
    },
    {
      "value": "\"require something\"",
      "start_offset": 98,
      "end_offset": 117
    },
    {
      "value": "\"table\"",
      "start_offset": 142,
      "end_offset": 149
    },
    {
      "value": "\"cosock is table\"",
      "start_offset": 151,
      "end_offset": 168
    },
    {
      "value": "\"nobl client running\"",
      "start_offset": 380,
      "end_offset": 401
    },
    {
      "value": "\"nobl client connect\"",
      "start_offset": 455,
      "end_offset": 476
    },
    {
      "value": "\"connect: \"",
      "start_offset": 541,
      "end_offset": 552
    },
    {
      "value": "\"nobl client send\"",
      "start_offset": 580,
      "end_offset": 598
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 611,
      "end_offset": 618
    },
    {
      "value": "\"nobl client reveived:\"",
      "start_offset": 686,
      "end_offset": 709
    },
    {
      "value": "\"timeout\"",
      "start_offset": 740,
      "end_offset": 749
    },
    {
      "value": "\"nobl client exit\"",
      "start_offset": 761,
      "end_offset": 779
    },
    {
      "value": "\"nobl client\"",
      "start_offset": 803,
      "end_offset": 816
    },
    {
      "value": "\"fast client running\"",
      "start_offset": 917,
      "end_offset": 938
    },
    {
      "value": "\"fast client connect\"",
      "start_offset": 992,
      "end_offset": 1013
    },
    {
      "value": "\"connect: \"",
      "start_offset": 1078,
      "end_offset": 1089
    },
    {
      "value": "\"fast client send\"",
      "start_offset": 1117,
      "end_offset": 1135
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 1148,
      "end_offset": 1155
    },
    {
      "value": "\"fast client reveived:\"",
      "start_offset": 1227,
      "end_offset": 1250
    },
    {
      "value": "\"timeout\"",
      "start_offset": 1281,
      "end_offset": 1290
    },
    {
      "value": "\"fast client exit\"",
      "start_offset": 1302,
      "end_offset": 1320
    },
    {
      "value": "\"fast client\"",
      "start_offset": 1344,
      "end_offset": 1357
    },
    {
      "value": "\"slow client running\"",
      "start_offset": 1453,
      "end_offset": 1474
    },
    {
      "value": "\"slow client connect\"",
      "start_offset": 1528,
      "end_offset": 1549
    },
    {
      "value": "\"connect: \"",
      "start_offset": 1614,
      "end_offset": 1625
    },
    {
      "value": "\"slow client send\"",
      "start_offset": 1653,
      "end_offset": 1671
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 1684,
      "end_offset": 1691
    },
    {
      "value": "\"slow client reveived:\"",
      "start_offset": 1760,
      "end_offset": 1783
    },
    {
      "value": "\"foo\"",
      "start_offset": 1815,
      "end_offset": 1820
    },
    {
      "value": "\"slow client exit\"",
      "start_offset": 1905,
      "end_offset": 1923
    },
    {
      "value": "\"slow client\"",
      "start_offset": 1947,
      "end_offset": 1960
    },
    {
      "value": "\"server running\"",
      "start_offset": 1999,
      "end_offset": 2015
    },
    {
      "value": "\"no sock\"",
      "start_offset": 2054,
      "end_offset": 2063
    },
    {
      "value": "\"127.0.0.1\"",
      "start_offset": 2082,
      "end_offset": 2093
    },
    {
      "value": "\"bind\"",
      "start_offset": 2099,
      "end_offset": 2105
    },
    {
      "value": "\"server spawn clients\"",
      "start_offset": 2166,
      "end_offset": 2188
    },
    {
      "value": "\"server accept\"",
      "start_offset": 2290,
      "end_offset": 2305
    },
    {
      "value": "\"accepted socket\"",
      "start_offset": 2346,
      "end_offset": 2363
    },
    {
      "value": "\"server spawn recv\"",
      "start_offset": 2375,
      "end_offset": 2394
    },
    {
      "value": "\"coserver recvive\"",
      "start_offset": 2436,
      "end_offset": 2454
    },
    {
      "value": "\"coserver received:\"",
      "start_offset": 2496,
      "end_offset": 2516
    },
    {
      "value": "\"\\n\"",
      "start_offset": 2608,
      "end_offset": 2612
    },
    {
      "value": "\"server \"",
      "start_offset": 2672,
      "end_offset": 2681
    },
    {
      "value": "\"listen server\"",
      "start_offset": 2697,
      "end_offset": 2712
    },
    {
      "value": "\"----------------- exit -----------------\"",
      "start_offset": 2735,
      "end_offset": 2777
    }
  ],
  "/home/rfm/projects/cosock/test/asyncify/two.lua": [
    {
      "value": "\"test.asyncify.nested.module\"",
      "start_offset": 15,
      "end_offset": 44
    }
  ],
  "/home/rfm/projects/cosock/test/channel/via-select.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "\"receiver spawn\"",
      "start_offset": 125,
      "end_offset": 141
    },
    {
      "value": "\"server exit\"",
      "start_offset": 432,
      "end_offset": 445
    },
    {
      "value": "\"receiver\"",
      "start_offset": 452,
      "end_offset": 462
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 485,
      "end_offset": 527
    }
  ],
  "/home/rfm/projects/cosock/cosock/ssl.lua": [
    {
      "value": "\"ssl\"",
      "start_offset": 23,
      "end_offset": 28
    },
    {
      "value": "\"cosock.socket.internals\"",
      "start_offset": 55,
      "end_offset": 80
    },
    {
      "value": "\"close\"",
      "start_offset": 446,
      "end_offset": 453
    },
    {
      "value": "\"config\"",
      "start_offset": 479,
      "end_offset": 487
    },
    {
      "value": "\"dirty\"",
      "start_offset": 512,
      "end_offset": 519
    },
    {
      "value": "\"dohandshake\"",
      "start_offset": 550,
      "end_offset": 563
    },
    {
      "value": "\"getalpn\"",
      "start_offset": 590,
      "end_offset": 599
    },
    {
      "value": "\"getfinished\"",
      "start_offset": 630,
      "end_offset": 643
    },
    {
      "value": "\"getpeercertificate\"",
      "start_offset": 681,
      "end_offset": 701
    },
    {
      "value": "\"getpeerchain\"",
      "start_offset": 733,
      "end_offset": 747
    },
    {
      "value": "\"getpeerverification\"",
      "start_offset": 786,
      "end_offset": 807
    },
    {
      "value": "\"getpeerfinished\"",
      "start_offset": 842,
      "end_offset": 859
    },
    {
      "value": "\"getsniname\"",
      "start_offset": 889,
      "end_offset": 901
    },
    {
      "value": "\"getstats\"",
      "start_offset": 929,
      "end_offset": 939
    },
    {
      "value": "\"loadcertificate\"",
      "start_offset": 974,
      "end_offset": 991
    },
    {
      "value": "\"newcontext\"",
      "start_offset": 1021,
      "end_offset": 1033
    },
    {
      "value": "\"receive\"",
      "start_offset": 1060,
      "end_offset": 1069
    },
    {
      "value": "\"send\"",
      "start_offset": 1228,
      "end_offset": 1234
    },
    {
      "value": "\"setdane\"",
      "start_offset": 1401,
      "end_offset": 1410
    },
    {
      "value": "\"setstats\"",
      "start_offset": 1438,
      "end_offset": 1448
    },
    {
      "value": "\"settlsa\"",
      "start_offset": 1475,
      "end_offset": 1484
    },
    {
      "value": "\"sni\"",
      "start_offset": 1507,
      "end_offset": 1512
    },
    {
      "value": "\"want\"",
      "start_offset": 1536,
      "end_offset": 1542
    },
    {
      "value": "\"tcp inner_sock is null\"",
      "start_offset": 1615,
      "end_offset": 1639
    }
  ],
  "/home/rfm/projects/cosock/test/udp/client-timeout.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "\"fast client spawn\"",
      "start_offset": 208,
      "end_offset": 227
    },
    {
      "value": "\"foo\"",
      "start_offset": 298,
      "end_offset": 303
    },
    {
      "value": "\"timeout\"",
      "start_offset": 406,
      "end_offset": 415
    },
    {
      "value": "\"fast client exit\"",
      "start_offset": 461,
      "end_offset": 479
    },
    {
      "value": "\"fast (0.01) client\"",
      "start_offset": 488,
      "end_offset": 508
    },
    {
      "value": "\"slow client spawn\"",
      "start_offset": 591,
      "end_offset": 610
    },
    {
      "value": "\"foo\"",
      "start_offset": 681,
      "end_offset": 686
    },
    {
      "value": "\"foo\"",
      "start_offset": 833,
      "end_offset": 838
    },
    {
      "value": "\"slow client exit\"",
      "start_offset": 883,
      "end_offset": 901
    },
    {
      "value": "\"slow (0.3) client\"",
      "start_offset": 910,
      "end_offset": 929
    },
    {
      "value": "\"server spawn\"",
      "start_offset": 968,
      "end_offset": 982
    },
    {
      "value": "\"sock\"",
      "start_offset": 1017,
      "end_offset": 1023
    },
    {
      "value": "\"*\"",
      "start_offset": 1048,
      "end_offset": 1051
    },
    {
      "value": "\"bind server\"",
      "start_offset": 1057,
      "end_offset": 1070
    },
    {
      "value": "\"receivefrom\"",
      "start_offset": 1251,
      "end_offset": 1264
    },
    {
      "value": "\"server exit\"",
      "start_offset": 1344,
      "end_offset": 1357
    },
    {
      "value": "\"server\"",
      "start_offset": 1364,
      "end_offset": 1372
    },
    {
      "value": "\"fast client not finished\"",
      "start_offset": 1420,
      "end_offset": 1446
    },
    {
      "value": "\"slow client not finished\"",
      "start_offset": 1477,
      "end_offset": 1503
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 1512,
      "end_offset": 1554
    }
  ],
  "/home/rfm/projects/cosock/test/http/https-via-http.lua": [
    {
      "value": "\"Lua 5.1\"",
      "start_offset": 68,
      "end_offset": 77
    },
    {
      "value": "\"cosock\"",
      "start_offset": 121,
      "end_offset": 129
    },
    {
      "value": "\"test.http.perform_test\"",
      "start_offset": 159,
      "end_offset": 183
    },
    {
      "value": "\"https-via-ssl\"",
      "start_offset": 198,
      "end_offset": 213
    },
    {
      "value": "\"https\"",
      "start_offset": 215,
      "end_offset": 222
    },
    {
      "value": "\"socket.http\"",
      "start_offset": 240,
      "end_offset": 253
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 284,
      "end_offset": 326
    }
  ],
  "/home/rfm/projects/cosock/test/asyncify/one.lua": [
    {
      "value": "\"socket\"",
      "start_offset": 15,
      "end_offset": 23
    }
  ],
  "/home/rfm/projects/cosock/cosock/channel.lua": [
    {
      "value": "\"closed\"",
      "start_offset": 696,
      "end_offset": 704
    },
    {
      "value": "\"recvr\"",
      "start_offset": 1065,
      "end_offset": 1072
    },
    {
      "value": "\"unsupported wake kind: \"",
      "start_offset": 1074,
      "end_offset": 1099
    },
    {
      "value": "\"waker already set, receive can't be waited on from multiple places at once\"",
      "start_offset": 1175,
      "end_offset": 1251
    },
    {
      "value": "\"closed\"",
      "start_offset": 1850,
      "end_offset": 1858
    }
  ],
  "/home/rfm/projects/cosock/test/threads/spawn-child-and-die.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "\"child spawn\"",
      "start_offset": 116,
      "end_offset": 129
    },
    {
      "value": "\"child exit\"",
      "start_offset": 163,
      "end_offset": 175
    },
    {
      "value": "\"child\"",
      "start_offset": 184,
      "end_offset": 191
    },
    {
      "value": "\"parent spawn\"",
      "start_offset": 231,
      "end_offset": 245
    },
    {
      "value": "\"parent exit\"",
      "start_offset": 267,
      "end_offset": 280
    },
    {
      "value": "\"parent\"",
      "start_offset": 287,
      "end_offset": 295
    },
    {
      "value": "\"child didn't run\"",
      "start_offset": 329,
      "end_offset": 347
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 356,
      "end_offset": 398
    }
  ],
  "/home/rfm/projects/cosock/cosock/socket/tcp.lua": [
    {
      "value": "\"socket\"",
      "start_offset": 26,
      "end_offset": 34
    },
    {
      "value": "\"cosock.socket.internals\"",
      "start_offset": 61,
      "end_offset": 86
    },
    {
      "value": "\"accept\"",
      "start_offset": 740,
      "end_offset": 748
    },
    {
      "value": "\"bind\"",
      "start_offset": 998,
      "end_offset": 1004
    },
    {
      "value": "\"close\"",
      "start_offset": 1092,
      "end_offset": 1099
    },
    {
      "value": "\"connect\"",
      "start_offset": 1126,
      "end_offset": 1135
    },
    {
      "value": "\"dirty\"",
      "start_offset": 1160,
      "end_offset": 1167
    },
    {
      "value": "\"getfamily\"",
      "start_offset": 1196,
      "end_offset": 1207
    },
    {
      "value": "\"getfd\"",
      "start_offset": 1232,
      "end_offset": 1239
    },
    {
      "value": "\"getoption\"",
      "start_offset": 1268,
      "end_offset": 1279
    },
    {
      "value": "\"getpeername\"",
      "start_offset": 1310,
      "end_offset": 1323
    },
    {
      "value": "\"getsockname\"",
      "start_offset": 1354,
      "end_offset": 1367
    },
    {
      "value": "\"getstats\"",
      "start_offset": 1395,
      "end_offset": 1405
    },
    {
      "value": "\"listen\"",
      "start_offset": 1431,
      "end_offset": 1439
    },
    {
      "value": "\"receive\"",
      "start_offset": 1466,
      "end_offset": 1475
    },
    {
      "value": "\"string\"",
      "start_offset": 1636,
      "end_offset": 1644
    },
    {
      "value": "\"send\"",
      "start_offset": 2971,
      "end_offset": 2977
    },
    {
      "value": "\"setfd\"",
      "start_offset": 3002,
      "end_offset": 3009
    },
    {
      "value": "\"setoption\"",
      "start_offset": 3038,
      "end_offset": 3049
    },
    {
      "value": "\"setstats\"",
      "start_offset": 3077,
      "end_offset": 3087
    }
  ],
  "/home/rfm/projects/cosock/cosock/socket/internals.lua": [
    {
      "value": "\"function\"",
      "start_offset": 657,
      "end_offset": 667
    },
    {
      "value": "\"table\"",
      "start_offset": 759,
      "end_offset": 766
    },
    {
      "value": "\"transformer must be table or function that returns table\"",
      "start_offset": 768,
      "end_offset": 826
    },
    {
      "value": "\"function\"",
      "start_offset": 891,
      "end_offset": 901
    },
    {
      "value": "\"input transformer not a function\"",
      "start_offset": 903,
      "end_offset": 937
    },
    {
      "value": "\"function\"",
      "start_offset": 1006,
      "end_offset": 1016
    },
    {
      "value": "\"blocked transformer not a function\"",
      "start_offset": 1018,
      "end_offset": 1054
    },
    {
      "value": "\"function\"",
      "start_offset": 1121,
      "end_offset": 1131
    },
    {
      "value": "\"output transformer not a function\"",
      "start_offset": 1133,
      "end_offset": 1168
    },
    {
      "value": "\"recvr\"",
      "start_offset": 1866,
      "end_offset": 1873
    },
    {
      "value": "\"sendr\"",
      "start_offset": 1916,
      "end_offset": 1923
    },
    {
      "value": "\"about to yield on method that is niether recv nor send\"",
      "start_offset": 1948,
      "end_offset": 2004
    },
    {
      "value": "\"recvr\"",
      "start_offset": 2068,
      "end_offset": 2075
    },
    {
      "value": "\"sendr\"",
      "start_offset": 2156,
      "end_offset": 2163
    },
    {
      "value": "\"recvr\"",
      "start_offset": 2577,
      "end_offset": 2584
    },
    {
      "value": "\"thread resumed without awaited socket or error (or too many sockets)\"",
      "start_offset": 2632,
      "end_offset": 2702
    },
    {
      "value": "\"thread resumed with unexpected socket\"",
      "start_offset": 2752,
      "end_offset": 2791
    },
    {
      "value": "\"thread resumed with unexpected socket\"",
      "start_offset": 2856,
      "end_offset": 2895
    },
    {
      "value": "\"thread resumed without awaited socket or error (or too many sockets)\"",
      "start_offset": 2939,
      "end_offset": 3009
    },
    {
      "value": "\"unsupported wake kind: \"",
      "start_offset": 3508,
      "end_offset": 3533
    },
    {
      "value": "\" waker already set, sockets can only block one thread per waker kind\"",
      "start_offset": 3660,
      "end_offset": 3730
    },
    {
      "value": "\"warning attempt to wake, but no waker set\"",
      "start_offset": 3962,
      "end_offset": 4005
    }
  ],
  "/home/rfm/projects/cosock/test/channel/basic.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "\"sender spawn\"",
      "start_offset": 290,
      "end_offset": 304
    },
    {
      "value": "\"sender client exit\"",
      "start_offset": 454,
      "end_offset": 474
    },
    {
      "value": "\"sender \"",
      "start_offset": 838,
      "end_offset": 847
    },
    {
      "value": "\"receiver spawn\"",
      "start_offset": 900,
      "end_offset": 916
    },
    {
      "value": "\"closed\"",
      "start_offset": 1163,
      "end_offset": 1171
    },
    {
      "value": "\"receiver received no message nor error\"",
      "start_offset": 1210,
      "end_offset": 1250
    },
    {
      "value": "\"receive error: \"",
      "start_offset": 1272,
      "end_offset": 1289
    },
    {
      "value": "\"server exit\"",
      "start_offset": 1434,
      "end_offset": 1447
    },
    {
      "value": "\"receiver\"",
      "start_offset": 1454,
      "end_offset": 1464
    },
    {
      "value": "\"no senders started\"",
      "start_offset": 1509,
      "end_offset": 1529
    },
    {
      "value": "\"not all senders finished\"",
      "start_offset": 1575,
      "end_offset": 1601
    },
    {
      "value": "\"no messages sent\"",
      "start_offset": 1629,
      "end_offset": 1647
    },
    {
      "value": "\"all messages not recieved: \"",
      "start_offset": 1692,
      "end_offset": 1721
    },
    {
      "value": "\", \"",
      "start_offset": 1738,
      "end_offset": 1742
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 1770,
      "end_offset": 1812
    }
  ],
  "/home/rfm/projects/cosock/test/tcp/prefix.lua": [
    {
      "value": "\"----------------------------------------\"",
      "start_offset": 6,
      "end_offset": 48
    },
    {
      "value": "\"cosock\"",
      "start_offset": 73,
      "end_offset": 81
    },
    {
      "value": "\"require something\"",
      "start_offset": 97,
      "end_offset": 116
    },
    {
      "value": "\"table\"",
      "start_offset": 141,
      "end_offset": 148
    },
    {
      "value": "\"cosock is table\"",
      "start_offset": 150,
      "end_offset": 167
    },
    {
      "value": "\"cosock.socket\"",
      "start_offset": 192,
      "end_offset": 207
    },
    {
      "value": "\"*\"",
      "start_offset": 587,
      "end_offset": 590
    },
    {
      "value": "\"reuseaddr\"",
      "start_offset": 619,
      "end_offset": 630
    },
    {
      "value": "\"connected to\"",
      "start_offset": 841,
      "end_offset": 855
    },
    {
      "value": "\"*l\"",
      "start_offset": 906,
      "end_offset": 910
    },
    {
      "value": "\"prefix\"",
      "start_offset": 912,
      "end_offset": 920
    },
    {
      "value": "\"client-task\"",
      "start_offset": 956,
      "end_offset": 969
    },
    {
      "value": "\"chunk\"",
      "start_offset": 1066,
      "end_offset": 1073
    },
    {
      "value": "\"\\n\"",
      "start_offset": 1170,
      "end_offset": 1174
    },
    {
      "value": "\"prefixchunk1chunk2chunk3chunk4\"",
      "start_offset": 1447,
      "end_offset": 1479
    },
    {
      "value": "\"Invalid outout: \"",
      "start_offset": 1481,
      "end_offset": 1499
    },
    {
      "value": "\"send new line\"",
      "start_offset": 1527,
      "end_offset": 1542
    },
    {
      "value": "\"prefixchunk1chunk2chunk3\"",
      "start_offset": 1741,
      "end_offset": 1767
    },
    {
      "value": "\"Invalid outout: %s\"",
      "start_offset": 1784,
      "end_offset": 1804
    },
    {
      "value": "\"don't send new line\"",
      "start_offset": 1831,
      "end_offset": 1852
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 1875,
      "end_offset": 1917
    }
  ],
  "/home/rfm/projects/cosock/cosock/socket/http.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 15,
      "end_offset": 23
    },
    {
      "value": "\"socket.http\"",
      "start_offset": 34,
      "end_offset": 47
    }
  ],
  "/home/rfm/projects/cosock/test/http/http.lua": [
    {
      "value": "\"Lua 5.1\"",
      "start_offset": 68,
      "end_offset": 77
    },
    {
      "value": "\"cosock\"",
      "start_offset": 121,
      "end_offset": 129
    },
    {
      "value": "\"test.http.perform_test\"",
      "start_offset": 159,
      "end_offset": 183
    },
    {
      "value": "\"http\"",
      "start_offset": 198,
      "end_offset": 204
    },
    {
      "value": "\"http\"",
      "start_offset": 206,
      "end_offset": 212
    },
    {
      "value": "\"socket.http\"",
      "start_offset": 230,
      "end_offset": 243
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 274,
      "end_offset": 316
    }
  ],
  "/home/rfm/projects/cosock/test/tcp/client-server-large-payload.lua": [
    {
      "value": "'cosock'",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "'0.0.0.0'",
      "start_offset": 186,
      "end_offset": 195
    },
    {
      "value": "'reuseaddr'",
      "start_offset": 215,
      "end_offset": 226
    },
    {
      "value": "'listening on:'",
      "start_offset": 256,
      "end_offset": 271
    },
    {
      "value": "'recieved request for:'",
      "start_offset": 817,
      "end_offset": 840
    },
    {
      "value": "'*'",
      "start_offset": 931,
      "end_offset": 934
    },
    {
      "value": "''",
      "start_offset": 942,
      "end_offset": 944
    },
    {
      "value": "'sent'",
      "start_offset": 960,
      "end_offset": 966
    },
    {
      "value": "\"send incomplete\"",
      "start_offset": 997,
      "end_offset": 1014
    },
    {
      "value": "'blob server'",
      "start_offset": 1098,
      "end_offset": 1111
    },
    {
      "value": "\"\\n\"",
      "start_offset": 1357,
      "end_offset": 1361
    },
    {
      "value": "''",
      "start_offset": 1378,
      "end_offset": 1380
    },
    {
      "value": "'blob client'",
      "start_offset": 2189,
      "end_offset": 2202
    },
    {
      "value": "'Exited successfully'",
      "start_offset": 2228,
      "end_offset": 2249
    }
  ],
  "/home/rfm/projects/cosock/test/error-handling/try-protect.lua": [
    {
      "value": "\"cosock\"",
      "start_offset": 23,
      "end_offset": 31
    },
    {
      "value": "\"all good\"",
      "start_offset": 242,
      "end_offset": 252
    },
    {
      "value": "\"your computer is super broke\"",
      "start_offset": 277,
      "end_offset": 307
    },
    {
      "value": "\"I meant to do that\"",
      "start_offset": 381,
      "end_offset": 401
    },
    {
      "value": "\"is this really my purpose in life?\"",
      "start_offset": 478,
      "end_offset": 514
    },
    {
      "value": "\"all good\"",
      "start_offset": 747,
      "end_offset": 757
    },
    {
      "value": "\"not all good\"",
      "start_offset": 759,
      "end_offset": 773
    },
    {
      "value": "\"internal error didn't error\"",
      "start_offset": 872,
      "end_offset": 901
    },
    {
      "value": "\"all good\"",
      "start_offset": 1313,
      "end_offset": 1323
    },
    {
      "value": "\"not all good\"",
      "start_offset": 1325,
      "end_offset": 1339
    },
    {
      "value": "\"internal error didn't error\"",
      "start_offset": 1437,
      "end_offset": 1466
    },
    {
      "value": "\"functions didn't run\"",
      "start_offset": 1674,
      "end_offset": 1696
    },
    {
      "value": "\"finalizers didn't run\"",
      "start_offset": 1725,
      "end_offset": 1748
    },
    {
      "value": "\"--------------- SUCCESS ----------------\"",
      "start_offset": 1757,
      "end_offset": 1799
    }
  ],
  "/home/rfm/projects/cosock/test/ssl/client-multi.lua": [
    {
      "value": "\"----------------------------------------\"",
      "start_offset": 6,
      "end_offset": 48
    },
    {
      "value": "\"cosock\"",
      "start_offset": 73,
      "end_offset": 81
    },
    {
      "value": "\"require something\"",
      "start_offset": 98,
      "end_offset": 117
    },
    {
      "value": "\"table\"",
      "start_offset": 142,
      "end_offset": 149
    },
    {
      "value": "\"cosock is table\"",
      "start_offset": 151,
      "end_offset": 168
    },
    {
      "value": "\"client running\"",
      "start_offset": 349,
      "end_offset": 365
    },
    {
      "value": "\"client tcp connect\"",
      "start_offset": 439,
      "end_offset": 459
    },
    {
      "value": "\"connect: \"",
      "start_offset": 535,
      "end_offset": 546
    },
    {
      "value": "\"client ssl wrap tcp\"",
      "start_offset": 961,
      "end_offset": 982
    },
    {
      "value": "\"ssl wrap on client socket: \"",
      "start_offset": 1067,
      "end_offset": 1096
    },
    {
      "value": "\"client ssl dohandshake\"",
      "start_offset": 1133,
      "end_offset": 1157
    },
    {
      "value": "\"ssl handshake on client socket: \"",
      "start_offset": 1244,
      "end_offset": 1278
    },
    {
      "value": "\"client send\"",
      "start_offset": 1320,
      "end_offset": 1333
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 1355,
      "end_offset": 1362
    },
    {
      "value": "\"client reveived:\"",
      "start_offset": 1420,
      "end_offset": 1438
    },
    {
      "value": "\"foo\"",
      "start_offset": 1472,
      "end_offset": 1477
    },
    {
      "value": "\"client exit\"",
      "start_offset": 1564,
      "end_offset": 1577
    },
    {
      "value": "\"client\"",
      "start_offset": 1616,
      "end_offset": 1624
    },
    {
      "value": "\"dclient running\"",
      "start_offset": 1722,
      "end_offset": 1739
    },
    {
      "value": "\"dclient tcp connect\"",
      "start_offset": 1876,
      "end_offset": 1897
    },
    {
      "value": "\"connect: \"",
      "start_offset": 1987,
      "end_offset": 1998
    },
    {
      "value": "\"connect: \"",
      "start_offset": 2116,
      "end_offset": 2127
    },
    {
      "value": "\"dclient1 ssl wrap tcp\"",
      "start_offset": 2553,
      "end_offset": 2576
    },
    {
      "value": "\"ssl wrap on dclient1 socket: \"",
      "start_offset": 2666,
      "end_offset": 2697
    },
    {
      "value": "\"dclient1 ssl dohandshake\"",
      "start_offset": 2735,
      "end_offset": 2761
    },
    {
      "value": "\"ssl handshake on dclient1 socket: \"",
      "start_offset": 2849,
      "end_offset": 2885
    },
    {
      "value": "\"dclient2 ssl wrap tcp\"",
      "start_offset": 3311,
      "end_offset": 3334
    },
    {
      "value": "\"ssl wrap on dclient2 socket: \"",
      "start_offset": 3424,
      "end_offset": 3455
    },
    {
      "value": "\"dclient2 ssl dohandshake\"",
      "start_offset": 3493,
      "end_offset": 3519
    },
    {
      "value": "\"ssl handshake on dclient2 socket: \"",
      "start_offset": 3607,
      "end_offset": 3643
    },
    {
      "value": "\"dclient send\"",
      "start_offset": 3685,
      "end_offset": 3699
    },
    {
      "value": "\"foo\\n\"",
      "start_offset": 3722,
      "end_offset": 3729
    },
    {
      "value": "\"bar\\n\"",
      "start_offset": 3752,
      "end_offset": 3759
    },
    {
      "value": "\"baz\\n\"",
      "start_offset": 3782,
      "end_offset": 3789
    },
    {
      "value": "\"ssl dclient call select\"",
      "start_offset": 4129,
      "end_offset": 4154
    },
    {
      "value": "\"ssl dclient select ret\"",
      "start_offset": 4253,
      "end_offset": 4277
    },
    {
      "value": "\"nil recvr\"",
      "start_offset": 4349,
      "end_offset": 4360
    },
    {
      "value": "\"table\"",
      "start_offset": 4392,
      "end_offset": 4399
    },
    {
      "value": "\"non-table recvr\"",
      "start_offset": 4401,
      "end_offset": 4418
    },
    {
      "value": "\"empty recvr\"",
      "start_offset": 4447,
      "end_offset": 4460
    },
    {
      "value": "\"dclient received:\"",
      "start_offset": 4589,
      "end_offset": 4608
    },
    {
      "value": "\"wrong data, expected '%s', got '%s'\"",
      "start_offset": 4804,
      "end_offset": 4841
    },
    {
      "value": "\"@@@@@@@@@@@@@@@ dclient left#:\"",
      "start_offset": 4993,
      "end_offset": 5025
    },
    {
      "value": "\"dclient exit\"",
      "start_offset": 5157,
      "end_offset": 5171
    },
    {
      "value": "\"double client\"",
      "start_offset": 5186,
      "end_offset": 5201
    },
    {
      "value": "\"server running\"",
      "start_offset": 5248,
      "end_offset": 5264
    },
    {
      "value": "\"no sock\"",
      "start_offset": 5307,
      "end_offset": 5316
    },
    {
      "value": "\"127.0.0.1\"",
      "start_offset": 5337,
      "end_offset": 5348
    },
    {
      "value": "\"bind\"",
      "start_offset": 5354,
      "end_offset": 5360
    },
    {
      "value": "\"server spawn clients\"",
      "start_offset": 5427,
      "end_offset": 5449
    },
    {
      "value": "\"server accept\"",
      "start_offset": 5908,
      "end_offset": 5923
    },
    {
      "value": "\"accepted socket\"",
      "start_offset": 5968,
      "end_offset": 5985
    },
    {
      "value": "\"server ssl wrap accepted socket\"",
      "start_offset": 6000,
      "end_offset": 6033
    },
    {
      "value": "\"ssl wrap on accepted socket: \"",
      "start_offset": 6111,
      "end_offset": 6142
    },
    {
      "value": "\"server ssl dohandshake\"",
      "start_offset": 6179,
      "end_offset": 6203
    },
    {
      "value": "\"ssl handshake on accepted socket: \"",
      "start_offset": 6277,
      "end_offset": 6313
    },
    {
      "value": "\"server spawn recv\"",
      "start_offset": 6345,
      "end_offset": 6364
    },
    {
      "value": "\"coserver recvive\"",
      "start_offset": 6421,
      "end_offset": 6439
    },
    {
      "value": "\"coserver received:\"",
      "start_offset": 6496,
      "end_offset": 6516
    },
    {
      "value": "\"\\n\"",
      "start_offset": 6593,
      "end_offset": 6597
    },
    {
      "value": "\"server \"",
      "start_offset": 6696,
      "end_offset": 6705
    },
    {
      "value": "\"listen server\"",
      "start_offset": 6736,
      "end_offset": 6751
    },
    {
      "value": "\"----------------- exit -----------------\"",
      "start_offset": 6775,
      "end_offset": 6817
    }
  ],
  "/home/rfm/projects/cosock/test/asyncify/nested/table.lua": [
    {
      "value": "\"table\"",
      "start_offset": 15,
      "end_offset": 22
    }
  ]
}
```
