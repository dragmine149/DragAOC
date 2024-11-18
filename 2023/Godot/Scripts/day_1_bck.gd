extends Node2D
enum Data { Default, Generated, Test };
@export var selected:Data = Data.Generated;
enum Part { Part1, Part2 };
@export var selected_part:Part = Part.Part1;

func get_data_path() -> String:
	match selected:
		Data.Test:
			return "res://Data/Day1/Test.txt";
		Data.Generated:
			return "res://Data/Day1/Generated.txt";

	if selected_part == Part.Part2:
		return "res://Data/Day1/Default2.txt";
	return "res://Data/Day1/Default.txt";

func get_lowest_num(line:String):
	for chr in line:
		if chr.is_valid_int():
			return chr;
	return -1;

func arr_sum(arr:Array):
	var sum = 0;
	for num in arr:
		sum += num;
	return sum;

func get_first_word_num(line:String, last:bool=false):
	var temp = "";
	var next_char = {
		"o": ["n"],
		"t": ["w", "h"],
		"f": ["o", "i"],
		"s": ["i", "e"],
		"e": ["i"],
		"n": ["i"],
		"on": ["e"],
		"tw": ["o"],
		"th": ["r"],
		"fo": ["u"],
		"fi": ["v"],
		"si": ["x"],
		"se": ["v"],
		"ei": ["g"],
		"ni": ["n"],
		"thr": ["e"],
		"fou": ["r"],
		"fiv": ["e"],
		"sev": ["e"],
		"eig": ["h"],
		"nin": ["e"],
		"thre": ["e"],
		"seve": ["n"],
		"eigh": ["t"],
		"one": 1,
		"two": 2,
		"three": 3,
		"four": 4,
		"five": 5,
		"six": 6,
		"seven": 7,
		"eight": 8,
		"nine": 9
	};
	var last_valid_num = -1;
	for char in line:
		if temp != "":
			if typeof(next_char.get(temp)) == TYPE_INT:
				if last:
					last_valid_num = next_char.get(temp);
					
					if next_char.get(temp.right(1)) != null:
						temp = temp.right(1);

						if char in next_char.get(temp):
							temp += char;

						if char.is_valid_int():
							last_valid_num = char;
							temp = "";
						
						if next_char.get(char) != null:
							temp = char;

						continue;						
					
					temp = "";
					if next_char.get(char) != null:
						temp += char;
						continue;
					
					if char.is_valid_int():
						last_valid_num = char;
					
					continue;

				return next_char.get(temp);
			
			if char in next_char.get(temp):
				temp += char;
				continue;
			
			if next_char.get(char) != null:
				temp = char;
				continue;
			
			# reset temp
			temp = "";
		
		if char.is_valid_int():
			if last:
				last_valid_num = char;
				temp = "";
				continue;
			return char;

		if next_char.get(char) != null:
			temp += char;
	
	if typeof(next_char.get(temp)) == TYPE_INT:
		last_valid_num = next_char.get(temp);
	
	if line.right(1).is_valid_int():
		last_valid_num = line.right(1);

	if last:
		return last_valid_num;
	push_error("No number found in line!");

var data;
var lines;
var numbers;
var line_index;
# Called when the node enters the scene tree for the first time.
func _ready():
	data = FileAccess.open(get_data_path(), FileAccess.READ);
	lines = data.get_as_text().split('\n');
	numbers = [];
	line_index = 0;

func process_line(line:String):
	if line == "":
		return;
	$Line.text = "Line: " + line;
	
	var lowest = get_lowest_num(line);
	if selected_part == Part.Part2:
		lowest = get_first_word_num(line);
	$Lowest.text = "Lowest: " + str(lowest);
	
	var highest = get_lowest_num(line.reverse());
	if selected_part == Part.Part2:
		highest = get_first_word_num(line, true);
	$Highest.text = "Highest: " + str(highest);

	numbers.append(int(str(lowest) + str(highest)));
	$List.text = "List: " + str(numbers);

func verify_list():
	var invalid = [];
	for i in numbers:
		if len(str(i)) != 2:
			invalid.append(i);
	
	$Invalid.text = "Invalid: " + str(invalid);

func _on_next_pressed():
	process_line(lines[line_index]);
	line_index += 1;
	$Index.text = "Index: %d" % line_index;
	if line_index == len(lines):
		$Result.text = "Result: " + str(arr_sum(numbers));

func _on_start_pressed():
	data = FileAccess.open(get_data_path(), FileAccess.READ);
	lines = data.get_as_text().split('\n');
	numbers = [];
	line_index = 0;
	for line in lines:
		process_line(line);
	
	verify_list();
	$Result.text = "Result: " + str(arr_sum(numbers));
