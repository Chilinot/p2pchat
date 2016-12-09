import os

for i in range(0,1000):
    os.system('echo \'{\"username\":\"%i\", \"message\":\"\", \"type\":\"handshake\"}\' | nc 130.238.18.73 8888 &' % i)
