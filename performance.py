import subprocess
import os

for i in range(0,1000):
    #os.system('echo \"say tjenare\" | ./p2pchat client%i %i -r 130.238.18.73:8888 &' % (i, 8000+i))
    subprocess.Popen(['echo', '\"say tjenare\" | ./p2pchat client%i %i -r 130.238.18.73:8888' % (i, 8000+i)], stdout=subprocess.PIPE)
