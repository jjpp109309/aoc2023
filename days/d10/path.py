import matplotlib.pyplot as plt
coords = ["4_12", "4_13", "5_13", "6_13", "7_13", "8_13", "9_13", "9_14", "8_14", "7_14", "7_15", "8_15", "9_15", "9_16", "8_16", "7_16", "6_16", "6_15", "5_15", "5_14", "4_14", "4_15", "4_16", "5_16", "5_17", "6_17", "6_18", "7_18", "7_19", "6_19", "5_19", "5_18", "4_18", "4_17", "3_17", "3_16", "3_15", "2_15", "2_14", "1_14", "1_15", "0_15", "0_14", "0_13", "1_13", "2_13", "3_13", "3_12", "2_12", "1_12", "0_12", "0_11", "1_11", "2_11", "3_11", "4_11", "4_10", "3_10", "2_10", "1_10", "0_10", "0_9", "1_9", "2_9", "3_9", "3_8", "2_8", "1_8", "0_8", "0_7", "1_7", "2_7", "3_7", "3_6", "2_6", "1_6", "0_6", "0_5", "0_4", "0_3", "0_2", "0_1", "1_1", "2_1", "3_1", "3_0", "4_0", "4_1", "4_2", "4_3", "3_3", "3_2", "2_2", "1_2", "1_3", "1_4", "1_5", "2_5", "2_4", "3_4", "3_5", "4_5", "4_6", "5_6", "5_5", "5_4", "6_4", "6_5", "7_5", "8_5", "8_4", "9_4", "9_5", "9_6", "9_7", "9_8", "8_8", "8_7", "8_6", "7_6", "7_7", "6_7", "6_8", "7_8", "7_9", "6_9", "5_9", "5_10", "6_10", "7_10", "8_10", "9_10", "9_11", "8_11", "7_11", "7_12", "6_12", "6_11", "5_11", "5_12"]
test_points = [(7, 5), (4, 5), (3, 7), (6, 3), (14, 3)]

# coords = ["1_1", "1_2", "1_3", "1_4", "1_5", "1_6", "1_7", "1_8", "1_9", "2_1", "2_2", "2_3", "2_4", "2_5", "2_6", "2_7", "2_8", "2_9", "3_1", "3_2", "3_8", "3_9", "4_1", "4_2", "4_8", "4_9", "5_1", "5_2", "5_3", "5_4", "5_6", "5_7", "5_8", "5_9", "6_1", "6_4", "6_6", "6_9", "7_1", "7_2", "7_3", "7_4", "7_6", "7_7", "7_8", "7_9"]
# test_points = [(2, 2), (3, 2), (7, 2), (8, 2)]

coords.append(coords[0])


def parse_coords(coords: str) -> int:
    x = coords.split("_")
    return int(x[0]), int(x[1])


coords = [parse_coords(i) for i in coords]
x = [x[1] for x in coords]
y = [-x[0] + 9 for x in coords]

fig, ax = plt.subplots()
ax.plot(x, y, '-')
ax.set_xticks([i for i in range(int(max(x))+1)])
ax.set_yticks([i for i in range(int(max(y))+1)])
ax.grid()
fig.show()


def is_point_inside_polygon(point, polygon):
    n = len(polygon)
    odd_intersections = False

    for i in range(n):
        x1, y1 = polygon[i]
        x2, y2 = polygon[(i + 1) % n]  # To handle the last edge connecting back to the first vertex

        # Handle horizontal edges
        if y1 == y2:
            if y1 == point[1] and min(x1, x2) <= point[0] <= max(x1, x2):
                return True  # Point lies on the horizontal edge, consider it inside
            continue  # Skip further processing for horizontal edges

        # Check for intersection with non-horizontal edges
        if ((y1 <= point[1] < y2) or (y2 <= point[1] < y1)) and \
           (point[0] < (x2 - x1) * (point[1] - y1) / (y2 - y1) + x1):
            odd_intersections = not odd_intersections

    return odd_intersections


# Test point
polygon_vertices = [(x, y) for x, y in zip(x, y)]

# Check if the point is inside the polygon
for test_point in test_points:
    result = is_point_inside_polygon(test_point, polygon_vertices)
    print(test_point, result)
