#!/usr/bin/python
import sys
import msgpack

file = open(sys.argv[1], 'rb')
outfile = open(sys.argv[1] + '.decoded', 'w')

unpacker = msgpack.Unpacker(file)
for pack in unpacker:
    outfile.write(str(pack) + '\n')
