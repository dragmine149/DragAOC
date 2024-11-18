from helper import helper, math, AOCException

def get_combined_numbers(data):
	numbers = [-1, -1]
	w1 = data.find('one')
	if w1 > 0:
		numbers[0] = 1
	w2 = data.find('two')
	if w2 > 0:
		if w2 > w1:
			numbers[1] = 2
		else:
			numbers[1] = numbers[0]
			numbers[0] = 2
	w3 = data.find('three')
	w4 = data.find('four')
	w5 = data.find('five')
	w6 = data.find('six')
	w7 = data.find('seven')
	w8 = data.find('eight')
	w9 = data.find('nine')
	n1 = data.find('1')
	n2 = data.find('2')
	n3 = data.find('3')
	n4 = data.find('4')
	n5 = data.find('5')
	n6 = data.find('6')
	n7 = data.find('7')
	n8 = data.find('8')
	n9 = data.find('9')

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
