x1 = "8dcbafa1779616565ad430bc834b9389"
x2 = "7b2d78d14c26df82827bd98d057a78cd" 
val = "08f92872c3bcf5d8dd500a4988c60c56"

c1 = int(x1, 16)
c2 = int(x2, 16)
c = (c1 + c2) & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF

z = format(c, '032x')
print(z)