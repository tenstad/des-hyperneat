import cv2
import tensorflow as tf
import numpy as np

(x_train, y_train), (x_test, y_test) = tf.keras.datasets.mnist.load_data()

print(x_train.shape, y_train.shape)
indexes = np.arange(x_train.shape[0])
np.random.shuffle(indexes)

n = 1000
size = (8, 8)

with open('generated/mnist', 'w') as f:
    f.write('true\ntrue\n\n')

    for i in range(n):
        x = x_train[indexes[i]]
        x = cv2.resize(x, dsize=size, interpolation=cv2.INTER_CUBIC)
        x = x.reshape(-1) / 255
        x = list(map(str, x))
        f.write(', '.join(x) + '\n')

    f.write('\n')

    for i in range(n):
        y = [0] * 10
        y[y_train[i]] = 1
        y = list(map(str, y))
        f.write(', '.join(y) + '\n')

    f.write('\n')
