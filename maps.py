import threading


dict1 = {}

with open('8.input', 'r') as file:
    instructions = file.readline().strip()
    for line in file.readlines()[1:]:
        line = line.strip()  # Remove leading/trailing whitespaces
        if line:
            key, values_str = line.split(' = ')
            values = tuple(values_str.strip('()').split(', '))
            dict1[key] = {'L': values[0], 'R': values[1]}

class MyThread(threading.Thread):
    max_step_count = 0
    together = 0
    lock = threading.Lock()
    condition = threading.Condition()

    def __init__(self, i, start):
        super().__init__()
        self.step_count = 0
        self.id = i
        self.curr = start

    def run(self):
        while True:
            with MyThread.condition:
                self.curr = dict1[self.curr][instructions[self.step_count % len(instructions)]]
                self.step_count += 1
                #print(self.id, self.curr, self.step_count)
                
                if self.curr.endswith("Z"):

                    if self.step_count > MyThread.max_step_count:
                        MyThread.max_step_count = self.step_count
                        MyThread.together = 1
                        MyThread.condition.notify_all()
                        MyThread.condition.wait()

                    if self.step_count == MyThread.max_step_count:
                        MyThread.together += 1
                        if MyThread.together >= num_threads:
                            print(self.step_count)
                            with MyThread.condition:
                                MyThread.condition.notify_all()
                                return
                        else:
                            MyThread.condition.wait()
                            if MyThread.together == num_threads:
                                return


threads = []
for i, key in enumerate(dict1.keys()):
    if key.endswith('A'):
        thread = MyThread(i, key)
        threads.append(thread)
        
num_threads = len(threads)
print(num_threads)
        
# Start threads
for thread in threads:
    thread.start()

# Wait for all threads to finish
for thread in threads:
    thread.join()