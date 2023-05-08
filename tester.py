import os
import subprocess
from matplotlib import pyplot as plt
DATA = [
    3,
    17,
    37,
    263,
    757,
    2589,
    7762,
    23779,
    71105,
    213629,
]

REPES = 50

def plot():
    X = [x for x in range(1,11)]
    print(str(len(X)) + ", " + str(len(DATA)))
    plt.figure()
    plt.plot(X, DATA)
    plt.ylabel("N Elements")
    plt.xlabel("Disco")
    plt.show()
    print("Done ")

def main():
    total_time: int = 0
    counter = 0

    # Warming up
    for _ in range(0, 5):
        process = subprocess.run(["./target/release/strips", "14"], capture_output=True)

    mejor_caso: int = 1000000000
    peor_caso: int = 0

    while REPES != counter:
        process = subprocess.run(["./target/release/strips", "14"], capture_output=True)
        stdout = process.stdout.decode("utf-8").splitlines()
        for line in stdout:
            if "ms" in line:
                time = line.removesuffix("ms")

        time = int(time)
        if time < mejor_caso:
            mejor_caso = time
        if time > peor_caso:
            peor_caso = time

        total_time += time
        counter += 1
    print(f"Media: {total_time/counter}ms")
    print(f"Peor: {peor_caso}ms || Mejor {mejor_caso}ms")

if __name__ == "__main__":
    main()
