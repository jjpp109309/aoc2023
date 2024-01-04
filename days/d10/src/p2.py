import itertools
import tqdm
import matplotlib.pyplot as plt

from typing import List, Optional, Tuple
from shapely import Point, Polygon


def load_ids(path: str) -> List[str]:
    with open(path, "r") as file:
        return file.readlines()


def ids2coords(path: str) -> List[Tuple]:
    X = []
    Y = []
    with open(path, "r") as file:
        for idx, line in enumerate(file):
            x, y = line.split("_")
            X.append(int(x))
            Y.append(int(y))

    y_max = max(Y)
    Y = map(lambda t: y_max - t, Y)

    return [(x, y) for x, y in zip(X, Y)]


def winding_number_algorithm(point, polygon):
    wn = 0  # Winding number

    for i in range(len(polygon)):
        x1, y1 = polygon[i]
        x2, y2 = polygon[(i + 1) % len(polygon)]

        if y1 <= point[1] < y2 or y2 <= point[1] < y1:
            if point[0] < (x2 - x1) * (point[1] - y1) / (y2 - y1) + x1:
                wn += 1

    return wn


def plot_polygon(p: Polygon, points: Optional[List[Point]]):
    fig, ax = plt.subplots()

    x, y = p.exterior.coords.xy
    x = x.tolist()
    y = y.tolist()
    x.append(x[0])
    y.append(y[0])
    ax.plot(x, y)

    if points:
        for point in points:
            fmt = 'og' if p.contains_properly(point) else "or"
            px = point.coords[0][0]
            py = point.coords[0][1]
            ax.plot(px, py, fmt)

    ax.set_xticks([i for i in range(int(max(x)) + 1)])
    ax.set_yticks([i for i in range(int(max(y)) + 1)])
    ax.grid()

    ax.set_aspect('equal', adjustable='box')

    fig.show()
    input()
    plt.close()


def point_in_polygon(point, polygon):
    x, y = point
    n = len(polygon)
    inside = False

    for i in range(n):
        x1, y1 = polygon[i]
        x2, y2 = polygon[(i + 1) % n]

        if ((y1 <= y < y2) or (y2 <= y < y1)) and (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1):
            inside = not inside

    return inside


def count_enclosed(polygon: List[Tuple]) -> int:
    x_min = min(x for x, _ in polygon)
    x_max = max(x for x, _ in polygon)

    y_min = min(y for _, y in polygon)
    y_max = max(y for _, y in polygon)

    count = 0
    for point in tqdm.tqdm(itertools.product(range(x_min, x_max), range(y_min, y_max))):
        if point in polygon:
            continue
        elif point_in_polygon(point, polygon):
            # print(point)
            count += 1

    return count


if __name__ == "__main__":
    # path = "./test_loop.txt"
    path = "./loop.txt"
    print("parsing input")
    polygon = ids2coords(path)
    points = count_enclosed(polygon)
    print(f"total points {points}")
    # plot_polygon(polygon, points)
