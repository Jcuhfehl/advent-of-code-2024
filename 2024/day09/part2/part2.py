with open("input.txt", "r") as file:
    input_data = file.readline()[:-1]
# input_data = "2333133121414131402"

data = [
    i // 2 if i % 2 == 0 else -1
    for i, n in enumerate(input_data)
    for _ in range(int(n))
]

for file_number in range(data[-1], -1, -1):
    print(file_number)
    file_size = data.count(file_number)
    file_index = data.index(file_number)
    for i in range(0, file_index):
        if data[i : i + file_size] == file_size * [-1]:
            data[i : i + file_size] = file_size * [file_number]
            data[file_index : file_index + file_size] = file_size * [-1]
            break
print("".join([str(x) for x in data]).replace("-1", "."))

data_with_empty_being_zero = [0 if x == -1 else x for x in data]
checksum = sum(
    [file_number * i for i, file_number in enumerate(data_with_empty_being_zero)]
)
print(checksum)
