from typing import Dict, List, Tuple
import numpy as np
import planet_sim
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import json

print_flag = False

result = planet_sim.read_config_and_simulate_system("config.json")
if print_flag:
    for r in result:
        print(f'Time: {r.time}')
        print(r.positions)
        for k, v in r.positions.items():
            print(f'{k}: x={v.x}, y={v.y}')

with open('config.json') as f:
    d = json.load(f)

planet_names = [planet_config['id'] for planet_config in d['planets']]


def get_point_at(t, time_positions, planet_name):
    time_position = time_positions[t]
    planet_position = time_position.positions[planet_name]
    return [planet_position.x], [planet_position.y]


fig, ax = plt.subplots()

lines = []
for planet_name in planet_names:
    l, = ax.plot(*get_point_at(0, result, planet_name), marker='o')
    lines.append(l)


def animate(i):
    for j, planet_name in enumerate(planet_names):
        lines[j].set_data(*get_point_at(i, result, planet_name))


line_ani = animation.FuncAnimation(fig, animate, len(result), interval=1)

plt.show()