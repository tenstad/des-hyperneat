with open('iris.data') as f:
    lines = f.readlines()

with open('generated/iris', 'w') as f:
    f.write('true\ntrue\n\n')

    for line in lines:
        line = line.strip().split(',')
        f.write(', '.join(line[:-1]))
        f.write('\n')
    
    f.write('\n')

    for line in lines:
            line = line.strip().split(',')
            f.write({'o': '0, 0, 1', 's': '0, 1, 0', 'g': '1, 0, 0'}[line[-1][8]])
            f.write('\n')
