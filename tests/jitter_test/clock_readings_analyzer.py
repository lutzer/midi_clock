# requires matplotlib and numpy

import matplotlib.pyplot as plt
import numpy as np

data = np.loadtxt("readings_300bpm.txt")
times = data

diffs = np.diff(times)

print(times)

diffs_mean = np.mean(diffs)
diffs_var = np.std(diffs)

print("sum: ", np.sum(diffs))
print("mean difference in us", diffs_mean)
print("time std dev in us", diffs_var)
print("min/max deviation in us", np.amin(diffs) - diffs_mean,np.amax(diffs) - diffs_mean)
print("frequency/bpm", 1000000/diffs_mean, 60000000/(diffs_mean*24) )

plt.plot(diffs)
plt.axhline(diffs_mean, color='r')
plt.show()

