NAME = 'des_hyperneat'


with open('flamegraph.svg') as f:
	content = f.read()
i = 0

while 1:
	i = content.find('<g><title>', i)
	if i < 0:
		break
	i += 10
	j = content.find('</title>', i)
	name = content[i:j]
	if not NAME in name or '{{closure}}' in name:
		k = content.find('rgb(', i)
		l = content.find(')', k) + 1
		content = content[:k] + 'rgba(0, 0, 0, 0.2)' + content[l:]
	i = j

with open('flamegraph.svg', 'w') as f:
	f.write(content)
