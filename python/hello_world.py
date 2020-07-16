import planet_sim


result = planet_sim.read_config_and_simulate_system("config.json")
for r in result:
    print(f'Time: {r.time}')
    print(r.positions)
    for k, v in r.positions.items():
        print(f'{k}: x={v.x}, y={v.y}')