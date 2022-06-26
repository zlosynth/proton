#!/usr/bin/env python
#
# Kudos to Jatin Chowdhury
# * https://ccrma.stanford.edu/~jatin/ComplexNonlinearities/Hysteresis.html
# * https://dafx2019.bcu.ac.uk/papers/DAFx2019_paper_3.pdf
# * https://ccrma.stanford.edu/~jatin/papers/Complex_NLs.pdf
# * https://github.com/jatinchowdhury18/audio_dspy

import numpy as np
import matplotlib.pyplot as plt
import numpy as np

SR = 48000 * 8


def process(block, n):
    out = np.zeros(len(block))

    i = 0
    for x in block:
        if x < -1:
            out[i] = -2 / 3
        elif x > 1:
            out[i] = 2 / 3
        else:
            out[i] = x - x ** n / 3
        i += 1

    return out


def generate_sine(frequency, length=0.001):
    time = np.linspace(0, length, int(length * SR))
    return np.sin(frequency * 2 * np.pi * time)


def plot_harmonic_response(ax, signal):
    N = len(signal)

    Y = np.fft.rfft(signal)
    Y = Y / np.max(np.abs(Y))

    f = np.linspace(0, SR / 2, int(N / 2 + 1))

    ax.semilogx(f, 20 * np.log10(np.abs(Y)))
    ax.set_xlim([20, 120000])
    ax.set_ylim([-90, 5])


def plot_hysteresis_loop(ax, original, processed):
    ax.plot(original, processed)


def plot_signal(ax, time, signal):
    ax.plot(time, signal)


def analyze_processor(axs, column, processor, attributes):
    FREQUENCY = 100
    LENGTH = 0.01

    ax_loop = axs[0, column]
    ax_signal = axs[1, column]
    ax_response = axs[2, column]

    legend = []
    signal = generate_sine(FREQUENCY, length=LENGTH)
    time = np.linspace(0, LENGTH, int(SR * LENGTH))

    for a in attributes:
        processed = process(signal, a)
        plot_hysteresis_loop(ax_loop, signal, processed)
        plot_signal(ax_signal, time, processed)
        plot_harmonic_response(ax_response, processed)
        legend.append(a)

    ax_loop.legend(legend, loc='upper center', bbox_to_anchor=(0.5, 1.3), ncol=3)

    plot_signal(ax_signal, time, signal)


if __name__ == '__main__':
    fig, axs = plt.subplots(3, 5)
    fig.suptitle('Magnetic Hysteresis')

    axs[0, 0].set_ylabel('Hysteresis loop')
    axs[1, 0].set_ylabel('Processed signal')
    axs[2, 0].set_ylabel('Harmonic response')

    axs[0, 0].set_title('Drive')
    axs[0, 1].set_title('Saturation')
    axs[0, 2].set_title('Width')
    axs[0, 3].set_title('Mode')
    axs[0, 4].set_title('Approximation')

    analyze_processor(axs, 0, process, [3, 5, 7])
    analyze_processor(axs, 1, process, [3, 5, 7])
    analyze_processor(axs, 2, process, [3, 5, 7])
    analyze_processor(axs, 3, process, [3, 5, 7])
    analyze_processor(axs, 4, process, [3, 5, 7])

    plt.show()
