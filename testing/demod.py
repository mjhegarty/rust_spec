# code adapted from
#https://witestlab.poly.edu/blog/capture-and-decode-fm-radio/

#Basically what this does is it takes my data that has been correctly formatted
#By bash and makes it into a numpy array centered around +1 to -1, then does
#a whole bunch of DSP in a nutshell:
#1. Downsample signal: The sampling rate of IQ data usually only needs to be
#as high as the bandwidth of the signal, which is about 200KHz in this case.
#We also use a lowpass filter to get rid of any externous signals that would
#affect us. (Think it is a similiar concept to an image freq filter)

#2. Do polar discrimination on signal to demodulate it: This stage uses a nifty
#DSP trick that lets you get ϕ(t) of the signal

#3. Do de-emphasis on the demodulated signal to balance it out:
#When sending signals over the air they use a highpass filter to increase
#the power of the higher frequency signals, since there is more noise at
#the higher signals. We therefore need to use a lowpass filter to remove this
#increase. The author of the article just manually defined the coefficents for #a FIR filter which I honestly forget how to calculate

#4. Downsample again to just get mono signal: FM has a mono channel, along with
#several other channels, but all we want is the mono channel so we just keep
#about the first 40 KHz where it is stored

#5. Store as 16bit LE in a raw audio file

import numpy as np
import scipy.signal as signal
import sys

# see http://stackoverflow.com/a/3054314/3524528

F_station = int(99.5e6)   
Fc = F_station
#Fs = int(1140000)# Sample rate
Fs = int(2.048e6)# Sample rate
if len(sys.argv) > 1:
    filename = sys.argv[1]
else :
    filename = "data.txt"
data = open(filename, "r")

for length, l in enumerate(data):
    pass
length += 1
data = open(filename, "r")
reader = data.readlines()
data = np.zeros((int(length/2)), dtype = 'complex')
data1 = np.zeros((int(length)))
for i, x in enumerate(reader):
    data1[i]= float((float(x)/127.5))

for i in range(int(length/2)):
    data[i] = data1[i] + 1j*data1[i+int(length/2)]




x1 = data 
# To mix the data down, generate a digital complex exponential
# (with the same length as x1) with phase -F_offset/Fs
# Now, just multiply x1 and the digital complex expontential
# An FM broadcast signal has  a bandwidth of 200 kHz
f_bw = 200000
dec_rate = int(Fs / f_bw)
x2 = signal.decimate(x1, dec_rate)
# Calculate the new sampling rate
Fs_y = Fs/dec_rate


y3 = x2[1:] * np.conj(x2[:-1])
x3 = np.angle(y3)
# The de-emphasis filter
# Given a signal 'x3' (in a numpy array) with sampling rate Fs_y
d = Fs_y * 75e-6   # Calculate the # of samples to hit the -3dB point
x = np.exp(-1/d)   # Calculate the decay between each sample
b = [1-x]          # Create the filter coefficients
a = [1,-x]
x4 = signal.lfilter(b,a,x3)
# Find a decimation rate to achieve audio sampling rate between 44-48 kHz
audio_freq = 44100.0
dec_audio = int(Fs_y/audio_freq)
Fs_audio = Fs_y / dec_audio

x5 = signal.decimate(x4, dec_audio)

# Scale audio to adjust volume
x5 *= 10000 / np.max(np.abs(x5))
# Save to file as 16-bit signed single-channel audio samples
x5.astype("int16").tofile("data.raw")








