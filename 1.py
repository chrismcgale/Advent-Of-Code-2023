with open('1.input', 'r') as file:
    array = file.readlines()

sum = 0

digits = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'zero']

stringToInt = {
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9,
    'zero': 0
}

for l in array:
    start = 0
    end = 0
        
    length = len(l)
    pos = length
    
    for i, c in enumerate(l):
        if c.isdigit():
            start = ord(c) - ord('0')
            pos = i
            break
        

    for d in digits:
        index = l.find(d)
        if index != -1 and index < pos:
            pos = index
            start = stringToInt[d]
              
    
    pos = 0
            
    for i, c in enumerate(l[::-1]):
        if c.isdigit():
            end = ord(c) - ord('0')
            pos = length - i
            break
        
    rev = l[::-1]
    
        
    for d in digits:
        index = rev.find(d[::-1])
        if index != -1 and (length - index) > pos:
            pos = length - index
            end = stringToInt[d]

    sum += start * 10 + end
    

    
print(sum)