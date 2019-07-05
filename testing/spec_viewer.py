import sys
import numpy as np

import scipy.signal as signal


import matplotlib.pyplot as plt
if len(sys.argv) > 1:
    filename = sys.argv[1]
else:
    filename = "data.txt"
data = open(filename, "r")

for length, l in enumerate(data):
    pass
length += 1
data = open(filename, "r")
reader = data.readlines()
data = np.zeros((int(length/2)), dtype = 'complex')
data1 = np.zeros((int(length)))
for i,x in enumerate(reader):
    data1[i] = x
for i in range(int(length/2)):
    data[i] = data1[i] + 1j*data1[i+int(length/2)]




f, t, spec1 = signal.spectrogram(data, fs=1.14e6)
print (spec1.shape)
print (spec1)

plt.figure()
plt.title('fm spectrogram, Centered at 99.5 MHz, no bandwidth specified')
plt.pcolormesh(t, np.fft.fftshift(f), np.fft.fftshift(spec1,axes=0))
plt.xlabel('time')
plt.ylabel('freq')
#plt.savefig('rust_data')
plt.show()
