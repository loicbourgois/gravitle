print("plop")

# A: 0.5   (0.5+0.4)/2          ->  0.45
# B: 0.4   ((0.5+0.4)/2+(0.25+0.4)/2)/2 ->  0.3875
# C: 0.25  (0.4+0.25)/2         ->  0.325
# T: 1.15

a = 0.5
b = 0.4
c = 0.25
print(a+b+c)

#print(a+b)

ab = (a-b)/2*0.1
bc = (b-c)/2*0.1
cb = (c-b)/2*0.1
ba = (b-a)/2*0.1
#ab = (a-b)/2

a2 = a - ab
b2 = b - ba - bc
c2 = c - cb

print(a2+b2+c2)
print(a2,b2,c2)
#print(a2+b2)
