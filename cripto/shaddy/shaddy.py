#!/usr/bin/env python
# -*- coding: utf-8 -*-
#
#  shaddy.py
# 

import random as rnd
import base64 as b64
import sys

def toHex(dec, zc):
    bh = hex(dec).split('x')[1].upper()
    bh = bh.replace('L', '')
    return ( '0' * ( zc - len(bh) ) ) + bh

def vPrim(num):
    res = True
    i = 2
    while(res and i < ( (num/2)+1) ):
        if( (num % i) == 0 ):
            res = False
        i += 1
    
    return res

def rPrim(space):
    space = int(space)
    
    if(vPrim(space)):
        return space
    else:
        d = 1
        while True:
            if(vPrim(space+d)):
                return space+d
            elif(vPrim(space-d)):
                return space-d
            else:
                d = d+1
    
def genKeys(space, parts):
    parts = int(parts)
    p = rPrim(space)
    if(p < 256):
        print "ERROR:\t El espacio minimo es de 256!."
    else:
        ph = toHex(p, 0)
        t = len(ph)
        
        g = rnd.randint(2, p-1)
        gh = toHex(g, t)
        sK = [0]*parts
        sQ = [0]*parts
        
        for s in range(parts):
            a = rnd.randint(2, p-1)
            A = (g**a) % p
            sK[s] = toHex(a, t)
            sQ[s] = toHex(A, t)
        
        sK = ''.join(sK)+':'+ph
        sQ = gh+''.join(sQ)+':'+ph
        
        print "\nCLAVE PRIVADA:\t"+b64.b64encode(sK)
        print "CLAVE PUBLICA:\t"+b64.b64encode(sQ)+"\n"

def getKeyParts(sQ):
    sQ = b64.b64decode(sQ)
    sQ = sQ.split(':')
    
    t = len(sQ[1])
    p = int(sQ[1], 16)
    Q = [t, p]
    
    sQ = sQ[0]
    while (sQ != ''):
        Q.append(int(sQ[:t], 16))
        sQ = sQ[t:]
    
    return Q

def egCif(t, p, A, b, m):
    y1 = ( ( A ** b ) * m ) % p
    hy1 = toHex(y1, t)
    return hy1

def egDcif(p, a, y0, y1):
    m = ( ( y0 ** ( p-1-a ) ) * y1 ) % p
    return chr(m)

def getCif(msg, sQ):
    Q = getKeyParts(sQ)
    t = Q[0]
    p = Q[1]
    g = Q[2]
    Q = Q[3:]
       
    b = rnd.randint(2, Q[0]-1)
    y0 = (g**b) % p
    hy0 = toHex(y0, t)
    
    CF = ''
    L = -1
    R = len(Q)
    while (msg != ''):
        
        L = L+1
        if L == R:
            L = 0
        
        CF = CF+egCif(t, p, Q[L], b, ord(msg[:1]))
        msg = msg[1:]
    
    CF = CF+':'+hy0
    
    print "\nMENSAJE CIFRADO:\n"+b64.b64encode(CF)+"\n"
    
def getMsg(cf, sK):
    K = getKeyParts(sK)
    t = K[0]
    p = K[1]
    K = K[2:]
    
    cf = b64.b64decode(cf).split(':')
    y0 = cf[1]
    y0 = int(y0, 16)
    cf = cf[0]
    
    MSG = ''
    L = -1
    R = len(K)
    while (cf != ''):
        
        L = L+1
        if L == R:
            L = 0
        y1 = int(cf[:t], 16)
        MSG = MSG+egDcif(p, K[L], y0, y1)
        cf = cf[t:]
    
    print "\nMENSAJE DESCIFRADO:\n"+MSG+"\n"

def usMsg():
    print "\nGenerar claves:\n\t$ shaddy -g [int:espacio] [int:partes]"
    print "Cifrar mensaje:\n\t$ shaddy -c [mensaje] [clave_publica]"
    print "Descifrar mensaje:\n\t$ shaddy -d [mensaje_cifrado] [clave_privada]\n"

def main():
    if(len(sys.argv) != 4):
        usMsg()
    elif(sys.argv[1] == '-g'):
        genKeys(sys.argv[2], sys.argv[3])
    elif(sys.argv[1] == '-c'):
        getCif(sys.argv[2], sys.argv[3])
    elif(sys.argv[1] == '-d'):
        getMsg(sys.argv[2], sys.argv[3])
    else:
        usMsg()
        
    return 0

if __name__ == '__main__':
    main()
