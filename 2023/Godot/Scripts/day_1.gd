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

func arr_sum(arr):
	var sum = 0;
	#for num in arr.values():
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
	
	if last:
		# one -> eno
		# two -> owt
		# three -> eerht
		# four -> ruof
		# five -> evif
		# six -> xis
		# seven -> neves
		# eight -> thgie
		# nine -> enin
		next_char = {
			"e": ["n", "e", "v"],
			"o": ["w"],
			"r": ["u"],
			"x": ["i"],
			"n": ["e"],
			"t": ["h"],
			"en": ["o", "i"],
			"ow": ["t"],
			"ee": ["r"],
			"ru": ["o"],
			"ev": ["i"],
			"xi": ["s"],
			"ne": ["v"],
			"th": ["g"],
			"eer": ["h"],
			"ruo": ["f"],
			"evi": ["f"],
			"nev": ["e"],
			"thg": ["i"],
			"eni": ["n"],
			"eerh": ["t"],
			"neve": ["s"],
			"thgi": ["e"],
			"eno": 1,
			"owt": 2,
			"eerht": 3,
			"ruof": 4,
			"evif": 5,
			"xis": 6,
			"neves": 7,
			"thgie": 8,
			"enin": 9
		}
		line = line.reverse();
	
	for char in line:
		if temp != "":
			if typeof(next_char.get(temp)) == TYPE_INT:
				return next_char.get(temp);
			
			if char in next_char.get(temp):
				temp += char;
				continue;
				
			if next_char.get(temp.right(1)) != null:
				if next_char.get(temp.right(1) + char) != null:
					temp = temp.right(1) + char;
					continue;
			
			if next_char.get(char) != null:
				temp = char;
				continue;
			
			# reset temp
			temp = "";
		
		if char.is_valid_int():
			return char;

		if next_char.get(char) != null:
			temp = char;

	push_error("No number found in line!");

var data;
var lines;
var numbers;
var line_index;
# Called when the node enters the scene tree for the first time.
func _ready():
	data = FileAccess.open(get_data_path(), FileAccess.READ);
	lines = data.get_as_text().split('\n');
	#numbers = {};
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

	#numbers[line] = int(str(lowest) + str(highest));
	numbers.append(int(str(lowest) + str(highest)));
	$List.text = "List: " + str(numbers);


func _on_next_pressed():
	process_line(lines[line_index]);
	line_index += 1;
	$Index.text = "Index: %d" % line_index;
	if line_index == len(lines):
		$Result.text = "Result: " + str(arr_sum(numbers));

func _on_start_pressed():
	data = FileAccess.open(get_data_path(), FileAccess.READ);
	lines = data.get_as_text().split('\n');
	#numbers = {};
	numbers = [];
	line_index = 0;
	for line in lines:
		process_line(line);
	
	$Result.text = "Result: " + str(arr_sum(numbers));
	
	#FileAccess.open("res://Data/Day1/Check.json", FileAccess.WRITE).store_string(JSON.stringify(numbers));
