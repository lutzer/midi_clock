# requires matplotlib and numpy

import matplotlib.pyplot as plt
import numpy as np

data = np.loadtxt("readings_320bpm.txt")
times = data

diffs = np.diff(times)

print(times)

diffs_mean = np.mean(diffs)
diffs_var = np.std(diffs)

print("sum: ", np.sum(diffs))
print("mean frequency: ", 1/diffs_mean)
print("time std dev in us: ", diffs_var)

plt.plot(diffs)
plt.axhline(diffs_mean, color='r')
plt.show()

