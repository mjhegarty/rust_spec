import numpy as np

import scipy.signal as signal


import matplotlib.pyplot as plt
data = open("data.txt", "r")

for length, l in enumerate(data):
    pass
length += 1
data = open("data.txt", "r")
reader = data.readlines()
data = np.zeros((int(length/2)), dtype = 'complex')
data1 = np.zeros((int(length)))
for i, x in enumerate(reader):
    data1[i]= float((float(x)-127.5)/127.5)

for i in range(int(length/2)):
    data[i] = data1[i] + 1j*data1[i*2]




f, t, spec1 = signal.spectrogram(data, fs=2.048e6)
print (spec1.shape)
print (spec1)

plt.figure()
plt.title('fm spectrogram, Centered at 99.5 MHz, no bandwidth specified')
plt.pcolormesh(t, np.fft.fftshift(f), np.fft.fftshift(spec1,axes=0))
plt.xlabel('time')
plt.ylabel('freq')
plt.savefig('rust_data')
plt.show()
