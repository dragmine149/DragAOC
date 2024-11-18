from helper import helper, math, AOCException

def get_combined_numbers(data):
	num1 = '-1'
	for char in data:
		if char.isdigit():
			num1 = char
			break

	num2 = '-1'
	for char in reversed(data):
		if char.isdigit():
			num2 = char
			break

	return int(num1 + num2)


if __name__ == "__main__":
	h = helper(1)
	dayData = h.main()
	numbers = []
	for line in dayData:
		numbers.append(get_combined_numbers(line.strip()))
	print(numbers)

	sum = 0
	for num in numbers:
		sum += num

	print(sum)
