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


def build_planet_position_lists(time_positions, planet_names):
    """
    Reformat export for matplot lib, Returns a dict of the following format
    {
        'planet_1': {'x': [1,2,3],
                     'y': [4,5,6],
                    },

        'planet_2': {'x': [1,2,3],
                     'y': [4,5,6],
                    },
        ...
    }
    """
    planet_dict = {}
    for planet_name in planet_names:
        x_positions = [time_position.positions[planet_name].x for time_position in time_positions]
        y_positions = [time_position.positions[planet_name].y for time_position in time_positions]
        planet_dict[planet_name] = {'x': x_positions, 'y': y_positions}

    return planet_dict

position_lists = build_planet_position_lists(result, planet_names)


fig, ax = plt.subplots()
colors = ['firebrick',
          'limegreen',
          'dodgerblue',
          'gold',
          'darkorange',
          'midnightblue'
          'darkviolet',
          'deeppink'
        ]

# Build the initial line objects to update in the animation
points = []
lines = []

for i, planet_name in enumerate(planet_names):
    p, = ax.plot(position_lists[planet_name]['x'][0], position_lists[planet_name]['y'][0], marker='o', color=colors[i])
    points.append(p)

    l, = ax.plot(position_lists[planet_name]['x'][0], position_lists[planet_name]['y'][0], marker='', color=colors[i], alpha=0.5)
    lines.append(l)


def animate(i):
    for j, planet_name in enumerate(planet_names):
        points[j].set_data(position_lists[planet_name]['x'][i], position_lists[planet_name]['y'][i])
        lines[j].set_data(position_lists[planet_name]['x'][:i+1], position_lists[planet_name]['y'][:i+1])


# Animate the plot and show
line_ani = animation.FuncAnimation(fig, animate, len(result), interval=1)
plt.show()