#!/usr/bin/env python
#
# Kudos to Jatin Chowdhury
# * https://jatinchowdhury18.medium.com/complex-nonlinearities-episode-3-hysteresis-fdeb2cd3e3f6
# * https://dafx2019.bcu.ac.uk/papers/DAFx2019_paper_3.pdf
# * https://ccrma.stanford.edu/~jatin/papers/Complex_NLs.pdf
# * https://github.com/jatinchowdhury18/audio_dspy

from csv import DictWriter
import argparse
import sys
import concurrent.futures

from scipy.optimize import curve_fit
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

FS = 48000 * 8


class Differentiator:
    """Time domain differentiation using the trapezoidal rule"""

    def __init__(self, fs):
        self.T = 1.0 / fs
        self.x_1 = 0.0
        self.x_d_n1 = 0.0

    def differentiate(self, x):
        d_alpha = 0.75
        x_d = (((1 + d_alpha) / self.T) * (x - self.x_1)) - d_alpha * self.x_d_n1
        self.x_1 = x
        self.x_d_n1 = x_d
        return x_d


class Hysteresis:
    """Class to implement hysteresis processing"""

    def __init__(self, drive, saturation, width, fs, makeup=True):
        """
        Parameters
        ----------
        drive : float
            Hysteresis drive parameter
        saturation : float
            Saturation parameter
        width : float
            Hysteresis width parameter
        fs : float
            Sample rate
        """
        self.deriv = Differentiator(fs)
        self.T = 1.0 / fs
        self.M_s = 0.5 + 1.5 * (1 - saturation)  # saturation
        self.a = self.M_s / (0.01 + 6 * drive)  # adjustable parameter
        self.alpha = 1.6e-3
        self.k = 30 * (1 - 0.5) ** 6 + 0.01  # coercivity
        self.c = (1 - width) ** 0.5 - 0.01  # changes slope
        if makeup:
            # TODO: first 0.2 of is getting up to 1, then it gets undercompensated
            # self.makeup = (1.1566 + 0.2968 * width ** 1.37) / (1.5665 - saturation ** 1.43)
            x1 = drive
            x2 = saturation
            x3 = width
            x4 = 1

            a1 = 2.92183732e-01
            a2 = 3.83587066e-01
            a3 = -4.12807883e-01
            a4 = 2.63565646e+00
            a5 = -2.02954036e-02
            a6 = -2.33793445e-01
            a7 = 1.47444536e-01
            a8 = -2.47464017e+00
            a9 = 4.68106521e-04
            a10 = -1.95089142e-02
            a11 = -5.35647575e-01
            a12 = 8.21883132e-01
            a13 = -1.38625427e-01
            a14 = -2.78652110e-02
            a15 = -1.40898473e-02
            a16 = 5.76600047e-02
            a17 = -1.85152167e+00
            a18 = 2.55584425e-01
            a19 = 4.61574452e-03
            a20 = 1.31761628e-03
            a21 = -4.46917078e-04
            a22 = 2.63399370e-02
            a23 = 1.18055271e-01
            a24 = 4.34739804e-01
            a25 = 7.24152304e-03
            a26 = -5.88168302e-03
            a27 = 1.03339067e-02
            a28 = 3.47648636e-01
            a29 = 9.15414904e-01
            a30 = -1.35347946e-01
            b = -1.78693175e-01

            self.makeup = (
                a1 * x1 +
                a2 * x2 +
                a3 * x3 +
                a4 * x4 +
                a5 * x1 ** 2 +
                a6 * x2 ** 2 +
                a7 * x3 ** 2 +
                a8 * x4 ** 2 +
                a9 * x1 ** 3 +
                a10 * x2 ** 3 +
                a11 * x3 ** 3 +
                a12 * x4 ** 3 +
                a13 * x1 * x2 +
                a14 * x1 * x3 +
                a15 * x1 * x4 +
                a16 * x2 * x3 +
                a17 * x2 * x4 +
                a18 * x3 * x4 +
                a19 * x1 ** 2 * x2 +
                a20 * x1 ** 2 * x3 +
                a21 * x1 ** 2 * x4 +
                a22 * x2 ** 2 * x3 +
                a23 * x2 ** 2 * x4 +
                a24 * x3 ** 2 * x4 +
                a25 * x1 * x2 ** 2 +
                a26 * x1 * x3 ** 2 +
                a27 * x1 * x4 ** 2 +
                a28 * x2 * x3 ** 2 +
                a29 * x2 * x4 ** 2 +
                a30 * x3 * x4 ** 2 +
                b
            )
        else:
            self.makeup = 1.0

    @staticmethod
    def langevin(x):
        """Langevin function: coth(x) - (1/x)"""
        if (abs(x) > 10 ** -4):
            return (1 / np.tanh(x)) - (1 / x)
        else:
            return (x / 3)

    @staticmethod
    def langevin_deriv(x):
        """Derivative of the Langevin function: (1/x^2) - coth(x)^2 + 1"""
        if (abs(x) > 10 ** -4):
            return (1 / x ** 2) - (1 / np.tanh(x)) ** 2 + 1
        else:
            return (1 / 3)

    def dMdt(self, M, H, H_d):
        """Jiles-Atherton differential equation

        Parameters
        ----------
        M : float
            Magnetisation
        H : float
            Magnetic field
        H_d : float
            Time derivative of magnetic field

        Returns
        -------
        dMdt : float
            Derivative of magnetisation w.r.t time
        """
        Q = (H + self.alpha * M) / self.a
        M_diff = self.M_s * self.langevin(Q) - M
        delta_S = 1 if H_d > 0 else -1
        delta_M = 1 if np.sign(delta_S) == np.sign(M_diff) else 0
        L_prime = self.langevin_deriv(Q)

        denominator = 1 - self.c * self.alpha * (self.M_s / self.a) * L_prime

        t1_num = (1 - self.c) * delta_M * M_diff
        t1_den = (1 - self.c) * delta_S * self.k - self.alpha * M_diff
        t1 = (t1_num / t1_den) * H_d

        t2 = self.c * (self.M_s / self.a) * H_d * L_prime

        return (t1 + t2) / denominator

    def RK4(self, M_n1, H, H_n1, H_d, H_d_n1):
        """Compute hysteresis function with Runge-Kutta 4th order

        Parameters
        ----------
        M_n1 : float
            Previous magnetisation
        H : float
            Magnetic field
        H_n1 : float
            Previous magnetic field
        H_d : float
            Magnetic field derivative
        H_d_n1 : float
            Previous magnetic field derivative

        Returns
        -------
        M : float
            Current magnetisation
        """
        k1 = self.T * self.dMdt(M_n1, H_n1, H_d_n1)
        k2 = self.T * self.dMdt(M_n1 + k1 / 2, (H + H_n1) /
                                2, (H_d + H_d_n1) / 2)
        k3 = self.T * self.dMdt(M_n1 + k2 / 2, (H + H_n1) /
                                2, (H_d + H_d_n1) / 2)
        k4 = self.T * self.dMdt(M_n1 + k3, H, H_d)
        return M_n1 + (k1 / 6) + (k2 / 3) + (k3 / 3) + (k4 / 6)

    def process_block(self, x):
        """Process block of samples"""
        M_out = np.zeros(len(x))
        M_n1 = 0
        H_n1 = 0
        H_d_n1 = 0

        n = 0
        for H in x:
            H_d = self.deriv.differentiate(H)
            M = self.RK4(M_n1, H, H_n1, H_d, H_d_n1)

            M_n1 = M
            H_n1 = H
            H_d_n1 = H_d

            M_out[n] = M * self.makeup
            n += 1

        return M_out


def generate_sine(frequency, length):
    time = np.linspace(0, length, int(length * FS))
    return np.sin(frequency * 2 * np.pi * time)


def plot_harmonic_response(ax, signal):
    N = len(signal)

    Y = np.fft.rfft(signal)
    Y = Y / np.max(np.abs(Y))

    f = np.linspace(0, FS / 2, int(N / 2 + 1))

    ax.semilogx(f, 20 * np.log10(np.abs(Y)))
    ax.set_xlim([20, 20000])
    ax.set_ylim([-90, 5])


def plot_hysteresis_loop(ax, original, processed):
    ax.plot(original, processed)


def plot_signal(ax, time, signal):
    ax.plot(time, signal)


def analyze_processor(axs, column, processor, attributes):
    FREQUENCY = 100
    LENGTH = 0.08

    ax_loop = axs[0, column]
    ax_signal = axs[1, column]
    ax_response = axs[2, column]

    legend = []

    # plot only the second half, after hysteresis stabilizes
    signal = generate_sine(FREQUENCY, length=LENGTH) * 3
    half = int(len(signal) / 2)
    half_signal = signal[half:]
    time = np.linspace(0, LENGTH, int(FS * LENGTH))
    half_time = time[:half]

    for a in attributes:
        processed = processor(signal, a)
        half_processed = processed[half:]
        plot_hysteresis_loop(ax_loop, half_signal, half_processed)
        plot_signal(ax_signal, half_time, half_processed)
        plot_harmonic_response(ax_response, half_processed)
        legend.append(a)

    ax_loop.legend(legend, loc='upper center', bbox_to_anchor=(0.5, 1.4), ncol=3)

    plot_signal(ax_signal, half_time, half_signal)


def response():
    fig, axs = plt.subplots(3, 3)

    axs[0, 0].set_ylabel('Hysteresis loop')
    axs[1, 0].set_ylabel('Processed signal')
    axs[2, 0].set_ylabel('Harmonic response')

    def processor(
        block,
        drive=1.0,
        saturation=0.9,
        width=1.0,
    ):
       return Hysteresis(drive, saturation, width, FS).process_block(block)

    axs[0, 0].set_title('Drive')
    analyze_processor(
        axs, 0,
        lambda block, drive: processor(block, drive=drive),
        [0.0, 0.1, 0.25, 0.5, 1.0, 5.0, 10.0, 20.0],
    )

    axs[0, 1].set_title('Saturation')
    analyze_processor(
        axs, 1,
        lambda block, saturation: processor(block, saturation=saturation),
        [0.0, 0.5, 1.0],
    )

    axs[0, 2].set_title('Width')
    analyze_processor(
        axs, 2,
        lambda block, width: processor(block, width=width),
        [0.0, 0.5, 0.99],
    )

    plt.show()


def amplitude_generate():
    I = 10
    D = 20
    S = 20
    W = 20

    input_configs = []
    for i in np.linspace(0.1, 1.0, I):
        for d in np.linspace(0.1, 20.0, D):
            for s in np.linspace(0, 1, S):
                for w in np.linspace(0, 0.999, W):
                    input_configs.append({
                        'i': i,
                        'd': d,
                        's': s,
                        'w': w,
                    })

    i = 1
    m = I * D * S * W
    configs = []
    with concurrent.futures.ProcessPoolExecutor() as executor:
        for config in executor.map(set_max_amplitude, input_configs):
            configs.append(config)
            print(f'{i}/{m}')
            i += 1

    with open('amplitude_dataset.csv', 'w', newline='') as f:
        writer = DictWriter(f, fieldnames=configs[0].keys())
        writer.writeheader()
        writer.writerows(configs)

    print('Done')


def set_max_amplitude(config):
    FREQUENCY = 100.0
    LENGTH = 0.02
    signal = generate_sine(FREQUENCY, length=LENGTH) * config['i']
    config['a'] = np.max(Hysteresis(config['d'], config['s'], config['w'], FS, makeup=False).process_block(signal))
    return config


def amplitude_fitting():
    data_frame = pd.read_csv('amplitude_dataset.csv')
    d_data = data_frame['d'].values
    s_data = data_frame['s'].values
    w_data = data_frame['w'].values
    i_data = data_frame['i'].values
    a_data = data_frame['a'].values

    input_configs = [
        # (func_a, d_data, s_data, w_data, i_data, a_data),
        # (func_b, d_data, s_data, w_data, i_data, a_data),
        # (func_c, d_data, s_data, w_data, i_data, a_data),
        # (func_d, d_data, s_data, w_data, i_data, a_data),
        (func_e, d_data, s_data, w_data, i_data, a_data),
        # (func_f, d_data, s_data, w_data, i_data, a_data),
        # (func_g, d_data, s_data, w_data, i_data, a_data),
        # (func_h, d_data, s_data, w_data, i_data, a_data),
        # (func_i, d_data, s_data, w_data, i_data, a_data),
        # (func_i, d_data, s_data, w_data, i_data, a_data),
        # (func_k, d_data, s_data, w_data, i_data, a_data),
    ]

    configs = []
    with concurrent.futures.ProcessPoolExecutor() as executor:
        for i, config in enumerate(executor.map(measure_fitting_accuracy, input_configs)):
            configs.append(config)
            print('{}/{}'.format(i, len(input_configs)))

    configs = sorted(configs, key=lambda x: x['rmse'])

    for config in configs:
        print(config)


def measure_fitting_accuracy(config):
    f = config[0]
    d_data = config[1]
    s_data = config[2]
    w_data = config[3]
    i_data = config[4]
    a_data = config[5]
    fitted_parameters, _ = curve_fit(f, [d_data, s_data, w_data, i_data], a_data, maxfev=15000)
    model_predictions = f([d_data, s_data, w_data, i_data, a_data], *fitted_parameters)
    abs_errors = model_predictions - a_data
    squared_errors = np.square(abs_errors)
    mean_squared_errors = np.mean(squared_errors)
    root_mean_squared_errors = np.sqrt(mean_squared_errors)
    r_squared = 1.0 - (np.var(abs_errors) / np.var(a_data))
    print(fitted_parameters)
    return {
        'func': f.__name__,
        'rmse': root_mean_squared_errors,
        'rs': r_squared,
    }


def func_a(data, a1, a2, a3, a4, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return (
        a1 * x1 +
        a2 * x2 +
        a3 * x3 +
        a4 * x4 +
        b
    )

def func_b(data, a1, a2, a3, a4, a5, a6, a7, a8, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return (
        a1 * x1 +
        a2 * x2 +
        a3 * x3 +
        a4 * x4 +
        a5 * x1 ** 2 +
        a6 * x2 ** 2 +
        a7 * x3 ** 2 +
        a8 * x4 ** 2 +
        b
    )

def func_c(data, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return (
        a1 * x1 +
        a2 * x2 +
        a3 * x3 +
        a4 * x4 +
        a5 * x1 ** 2 +
        a6 * x2 ** 2 +
        a7 * x3 ** 2 +
        a8 * x4 ** 2 +
        a9 * x1 * x2 +
        a10 * x1 * x3 +
        a11 * x1 * x4 +
        a12 * x2 * x3 +
        a13 * x2 * x4 +
        a14 * x3 * x4 +
        b
    )

def func_d(data, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return (
        a1 * x1 +
        a2 * x2 +
        a3 * x3 +
        a4 * x4 +
        a5 * x1 ** 2 +
        a6 * x2 ** 2 +
        a7 * x3 ** 2 +
        a8 * x4 ** 2 +
        a9 * x1 ** 3 +
        a10 * x2 ** 3 +
        a11 * x3 ** 3 +
        a12 * x4 ** 3 +
        b
    )

def func_e(data, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19, a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return (
        a1 * x1 +
        a2 * x2 +
        a3 * x3 +
        a4 * x4 +
        a5 * x1 ** 2 +
        a6 * x2 ** 2 +
        a7 * x3 ** 2 +
        a8 * x4 ** 2 +
        a9 * x1 ** 3 +
        a10 * x2 ** 3 +
        a11 * x3 ** 3 +
        a12 * x4 ** 3 +
        a13 * x1 * x2 +
        a14 * x1 * x3 +
        a15 * x1 * x4 +
        a16 * x2 * x3 +
        a17 * x2 * x4 +
        a18 * x3 * x4 +
        a19 * x1 ** 2 * x2 +
        a20 * x1 ** 2 * x3 +
        a21 * x1 ** 2 * x4 +
        a22 * x2 ** 2 * x3 +
        a23 * x2 ** 2 * x4 +
        a24 * x3 ** 2 * x4 +
        a25 * x1 * x2 ** 2 +
        a26 * x1 * x3 ** 2 +
        a27 * x1 * x4 ** 2 +
        a28 * x2 * x3 ** 2 +
        a29 * x2 * x4 ** 2 +
        a30 * x3 * x4 ** 2 +
        b
    )

def func_f(data, a1, a2, a3, a4, a5, a6, a7, a8, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return ((a1 + a2 * x1) * (a3 + a4 * x2)) / ((a5 + a6 * x3) * (a7 + a8 * x4)) + b

def func_g(data, a1, a2, a3, a4, a5, a6, a7, a8, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return ((a1 + a2 * x1) * (a3 + a4 * x2)) * ((a5 + a6 * x3) * (a7 + a8 * x4)) + b

def func_h(data, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return ((a1 + a2 * x1 ** a9) * (a3 + a4 * x2 ** a10)) / ((a5 + a6 * x3 ** a11) * (a7 + a8 * x4 ** a12)) + b

def func_i(data, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return ((a1 + a2 * x1 ** a9) * (a3 + a4 * x4 ** a10)) / ((a5 + a6 * x3 ** a11) * (a7 + a8 * x2 ** a12)) + b

def func_j(data, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return (a1 + a2 * x1 ** a9) * (a3 + a4 * x4 ** a10) * (a5 + a6 * x3 ** a11) * (a7 + a8 * x2 ** a12) + b

def func_k(data, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19, a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30, a31, a32, a33, a34, a35, a36, a37, a38, a39, a40, a41, a42, b):
    x1 = data[0]
    x2 = data[1]
    x3 = data[2]
    x4 = data[3]
    return (
        a1 * x1 +
        a2 * x2 +
        a3 * x3 +
        a4 * x4 +
        a5 * x1 ** 2 +
        a6 * x2 ** 2 +
        a7 * x3 ** 2 +
        a8 * x4 ** 2 +
        a9 * x1 ** 3 +
        a10 * x2 ** 3 +
        a11 * x3 ** 3 +
        a12 * x4 ** 3 +
        a13 * x1 * x2 +
        a14 * x1 * x3 +
        a15 * x1 * x4 +
        a16 * x2 * x3 +
        a17 * x2 * x4 +
        a18 * x3 * x4 +
        a19 * x1 ** 2 * x2 +
        a20 * x1 ** 2 * x3 +
        a21 * x1 ** 2 * x4 +
        a22 * x2 ** 2 * x3 +
        a23 * x2 ** 2 * x4 +
        a24 * x3 ** 2 * x4 +
        a25 * x1 * x2 ** 2 +
        a26 * x1 * x3 ** 2 +
        a27 * x1 * x4 ** 2 +
        a28 * x2 * x3 ** 2 +
        a29 * x2 * x4 ** 2 +
        a30 * x3 * x4 ** 2 +
        (a31 + a32 * x1 ** a33) * (a34 + a35 * x4 ** a36) * (a37 + a38 * x3 ** a39) * (a40 + a41 * x2 ** a42) +
        b
    )


if __name__ == '__main__':
    parser = argparse.ArgumentParser(prog=sys.argv[0])
    subparsers = parser.add_subparsers(help='sub-command help', required=True, dest='subparser')
    subparsers.add_parser('response', help='Plot processed signal, hysteresis loop, and harmonic response')
    subparsers.add_parser('amplitude_generate', help='Generate dataset mapping input arguments to amplitude')
    subparsers.add_parser('amplitude_fitting', help='Find the best fitting approximation for amplitude')
    args = parser.parse_args()

    if args.subparser == 'response':
        response()
    elif args.subparser == 'amplitude_generate':
        amplitude_generate()
    elif args.subparser == 'amplitude_plot':
        amplitude_plot()
    elif args.subparser == 'amplitude_fitting':
        amplitude_fitting()
