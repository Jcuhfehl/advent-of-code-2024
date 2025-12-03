stones = {}
with open("input.txt") as file:
    for num in file.readline().split(" "):
        num = int(num)
        stones[num] = stones.get(num, 0) + 1
# stones = {125: 1, 17: 1}

for i in range(1000):
    new_stones = {}
    for num, count in stones.items():
        if num == 0:
            new_stones[1] = new_stones.get(1, 0) + count
        elif len(str(num)) % 2 == 0:
            new_numbers = [
                int(str(num)[: len(str(num)) // 2]),
                int(str(num)[len(str(num)) // 2 :]),
            ]
            for new_num in new_numbers:
                new_stones[new_num] = new_stones.get(new_num, 0) + count
        else:
            new_stones[num * 2024] = new_stones.get(num * 2024, 0) + count
    stones = new_stones

print(stones)
print(sum(stones.values()))
