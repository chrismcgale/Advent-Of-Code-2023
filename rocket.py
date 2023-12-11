times = [56977793]
dist = [499221010971440]

times_test = [7, 15, 30]
dist_test = [9, 40, 200]

ans = 0
for t, d in zip(times, dist):
    ways = 0
    start = t
    end = 0
    
    for i in range(1, t):
        if i * (t - i) > d:
            start = i
            break
        
    for i in range(t - 1, 0, -1):
        if i * (t - i) > d:
            end = i
            break
        
    ways += end - start + 1
    print(start, end)
    ans = ways if ans == 0 else (ans * ways)
    
print(ans)