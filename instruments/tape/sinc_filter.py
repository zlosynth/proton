#!/usr/bin/env python
#
# Kudos to Nigel Redmon, www.earlevel.com
# * https://www.earlevel.com/main/2010/11/23/towards-a-practical-oversampling-filter/
# * https://www.earlevel.com/main/2010/12/05/building-a-windowed-sinc-filter/
# * https://www.earlevel.com/main/2010/12/05/sample-rate-conversion-up/
# * https://www.earlevel.com/main/2010/12/20/sample-rate-conversion-down/

from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

FS = 48000
OVERSAMPLING = 4
COEFFICIENTS = [
-0.00001658437577130305, -0.00003181232670095903, -0.00003352015145106342,
 -0.000004671859636728758, 0.00005875352160012468, 0.00013520653374628018,
 0.0001773896830432971, 0.0001312712384565898, -0.000029455696595708266,
 -0.00026983181693310954, -0.00048423704860093214, -0.0005285767549968089,
 -0.0002918895186311946, 0.0002207887293712792, 0.0008407869494875228,
 0.0012663041193296497, 0.0011830044939183499, 0.0004375890314515462,
 -0.0008167522338228116, -0.002095526952171281, -0.00273325133318234,
 -0.002182462740866956, -0.0003442238463218608, 0.0022397794211392232,
 0.004478006970946177, 0.0051359670969108855, 0.003432943922854553,
 -0.0004323908549797062, -0.005119749677828014, -0.008526238238920226,
 -0.008648934114947129, -0.004610862205796096, 0.002629500809523906,
 0.010316372839985715, 0.01483514831676312, 0.013287073876396941,
 0.005061696910505933, -0.0073665919847468305, -0.01897176826022916,
 -0.024056021897325516, -0.01884119998840754, -0.0036665732443587006,
 0.01633827942393259, 0.03272392666922933, 0.03705042853698633,
 0.024858954546110628, -0.0014564872871843408, -0.03242213012196274,
 -0.05449823348282212, -0.05554414988525402, -0.030688439278073416,
 0.013979988174388525, 0.061951552732907526, 0.09159528727337946,
 0.0847614306538222, 0.035583799847597505, -0.04417946127154801,
 -0.12602524050828023, -0.17240787925963671, -0.14921504832397428,
 -0.038851446962645905, 0.15091218372535148, 0.3860170497838923,
 0.614157015080748, 0.7795955332668679, 0.8399972586479154, 0.7795955332668679,
 0.614157015080748, 0.3860170497838923, 0.15091218372535148,
 -0.038851446962645905, -0.14921504832397428, -0.17240787925963671,
 -0.12602524050828023, -0.04417946127154801, 0.035583799847597505,
 0.0847614306538222, 0.09159528727337946, 0.061951552732907526,
 0.013979988174388525, -0.030688439278073416, -0.05554414988525402,
 -0.05449823348282212, -0.03242213012196274, -0.0014564872871843408,
 0.024858954546110628, 0.03705042853698633, 0.03272392666922933,
 0.01633827942393259, -0.0036665732443587006, -0.01884119998840754,
 -0.024056021897325516, -0.01897176826022916, -0.0073665919847468305,
 0.005061696910505933, 0.013287073876396941, 0.01483514831676312,
 0.010316372839985715, 0.002629500809523906, -0.004610862205796096,
 -0.008648934114947129, -0.008526238238920226, -0.005119749677828014,
 -0.0004323908549797062, 0.003432943922854553, 0.0051359670969108855,
 0.004478006970946177, 0.0022397794211392232, -0.0003442238463218608,
 -0.002182462740866956, -0.00273325133318234, -0.002095526952171281,
 -0.0008167522338228116, 0.0004375890314515462, 0.0011830044939183499,
 0.0012663041193296497, 0.0008407869494875228, 0.0002207887293712792,
 -0.0002918895186311946, -0.0005285767549968089, -0.00048423704860093214,
 -0.00026983181693310954, -0.000029455696595708266, 0.0001312712384565898,
 0.0001773896830432971, 0.00013520653374628018, 0.00005875352160012468,
 -0.000004671859636728758, -0.00003352015145106342, -0.00003181232670095903,
 -0.00001658437577130305
]


# TODO: Optimize filters to only handle needed


def create_saw(freq, harmonics, time):
    signal = np.zeros(len(time))
    for i in range(1, harmonics + 1):
        signal += np.sin(freq * i * 2 * np.pi * time) / i
    return signal


def oversample(signal):
    upsampled = np.zeros(len(signal) * OVERSAMPLING)

    for i in range(len(signal)):
        upsampled[i * OVERSAMPLING] = signal[i]

    return upsampled


def oversample_filter(signal):
    filtered = np.zeros(len(signal))

    for i in range(len(filtered)):
        coef_index = 0

        while coef_index < len(COEFFICIENTS) and i - coef_index >= 0:
            filtered[i] += signal[i - coef_index] * COEFFICIENTS[coef_index]
            coef_index += 1

    return filtered


def distort(signal):
    distorted = np.zeros(len(signal))

    for i in range(len(signal)):
        x = signal[i]
        if x > 2 / 3:
            distorted[i] = 2 / 3
        elif x < -2 / 3:
            distorted[i] = -2 / 3
        else:
            distorted[i] = x

    return distorted


def decimate_filter(signal):
    filtered = np.zeros(len(signal))

    for i in range(len(filtered)):
        coef_index = 0

        while coef_index < len(COEFFICIENTS) and i - coef_index >= 0:
            filtered[i] += signal[i - coef_index] * COEFFICIENTS[coef_index]
            coef_index += 1

    return filtered


def decimate(signal):
    decimated = np.zeros(int(len(signal) / OVERSAMPLING))

    for i in range(len(signal)):
        decimated[int(i / OVERSAMPLING)] = signal[i]

    return decimated


def plot_harmonic_response(ax, signal, fs):
    N = len(signal)

    Y = np.fft.rfft(signal)
    Y = Y / np.max(np.abs(Y))

    f = np.linspace(0, fs / 2, int(N / 2 + 1))

    ax.semilogx(f, 20 * np.log10(np.abs(Y)))
    ax.set_xlim([20, fs * 2])
    ax.set_ylim([-90, 5])

if __name__ == '__main__':
    length = 0.5
    frequency = 40.0
    harmonics = 500

    fig, axs = plt.subplots(3, 4)
    fig.tight_layout()

    time = np.linspace(0, length, int(length * FS))
    end = int((length * FS) / frequency) * 4

    os_time = np.linspace(0, length, int(length * FS * OVERSAMPLING))
    os_end = int((length * FS * OVERSAMPLING) / frequency) * 4

    axs[0, 0].set_title('1. Original wave')
    axs[0, 1].set_title('1. Original response')
    signal = create_saw(frequency, harmonics, time)
    axs[0, 0].plot(time[:end], signal[:end])
    plot_harmonic_response(axs[0, 1], signal, FS)

    axs[1, 0].set_title('2. Oversampled wave')
    axs[1, 1].set_title('2. Oversampled response')
    signal = oversample(signal)
    axs[1, 0].plot(os_time[:os_end], signal[:os_end])
    plot_harmonic_response(axs[1, 1], signal, FS * OVERSAMPLING)

    axs[2, 0].set_title('3. Filtered wave')
    axs[2, 1].set_title('3. Filtered response')
    signal = oversample_filter(signal)
    axs[2, 0].plot(os_time[:os_end], signal[:os_end])
    plot_harmonic_response(axs[2, 1], signal, FS * OVERSAMPLING)

    axs[0, 2].set_title('4. Distorted wave')
    axs[0, 3].set_title('4. Distorted response')
    signal = distort(signal)
    axs[0, 2].plot(os_time[:os_end], signal[:os_end])
    plot_harmonic_response(axs[0, 3], signal, FS * OVERSAMPLING)

    axs[1, 2].set_title('5. Filtered wave')
    axs[1, 3].set_title('5. Filtered response')
    signal = decimate_filter(signal)
    axs[1, 2].plot(os_time[:os_end], signal[:os_end])
    plot_harmonic_response(axs[1, 3], signal, FS * OVERSAMPLING)

    axs[2, 2].set_title('6. Decimated wave')
    axs[2, 3].set_title('6. Decimated response')
    signal = decimate(signal)
    axs[2, 2].plot(time[:end], signal[:end])
    plot_harmonic_response(axs[2, 3], signal, FS)

    plt.show()
