with open('1.input', 'r') as file:
    array = file.readlines()

sum = 0

for l in array:
    start = 0
    end = 0
    
    for c in l:
        if c.isdigit():
            start = ord(c) - ord('0')
            break
            
    for c in l[::-1]:
        if c.isdigit():
            end = ord(c) - ord('0')
            break
    
    sum += start * 10 + end
    
    
print(sum)