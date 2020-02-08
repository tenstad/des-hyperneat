with open('wine.data') as f:
    lines = f.readlines()

with open('wine', 'w') as f:
    for line in lines:
        line = line.strip().split(',')
        f.write(', '.join(line[1:]))
        f.write('\n')
    f.write('\n')
    for line in lines:
            line = line.strip().split(',')
            f.write({'1': '0, 0, 1', '2': '0, 1, 0', '3': '1, 0, 0'}[line[0][0]])
            f.write('\n')
