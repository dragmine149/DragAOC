extends Node2D
var Utils = utils.new();
@export var data:utils.Data;
@export var part:utils.Part;

const symbols = ["/", "*", "%", "$", "&", "@", "=", "#", "-", "+"];
const numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
var game_data:PackedStringArray;

func get_surrounding_cells(data:PackedStringArray, Pos:Vector2i):
	var surrounding = [];
	if Pos.x - 1 >= 0 and Pos.y - 1 >= 0:
		surrounding.append(data[Pos.y - 1][Pos.x - 1]);
	#if Pos.x - 1 >= 0 and Pos.y - 1 >= 0:
	surrounding.append(data[Pos.y - 1][Pos.x]);
	if Pos.x + 1 < len(data[0]) and Pos.y - 1 >= 0:
		surrounding.append(data[Pos.y - 1][Pos.x + 1]);

	if Pos.x - 1 >= 0:
		surrounding.append(data[Pos.y][Pos.x - 1]);
	if Pos.x + 1 < len(data[0]):
		surrounding.append(data[Pos.y][Pos.x + 1]);

	if Pos.y + 1 < len(data):
		if Pos.x - 1 >= 0:
			surrounding.append(data[Pos.y + 1][Pos.x - 1]);
		surrounding.append(data[Pos.y + 1][Pos.x]);
		if Pos.x + 1 < len(data[0]):
			surrounding.append(data[Pos.y + 1][Pos.x + 1]);
	
	return surrounding;

func get_left_right(Pos:Vector2i, left:bool = true, right:bool = true):
	var surrounding = {"Left": '', "Right": ''};
	if Pos.x - 1 >= 0 and left:
		if game_data[Pos.y][Pos.x - 1] in numbers:
			surrounding.Left = game_data[Pos.y][Pos.x - 1];
	if Pos.x + 1 < len(game_data[0]) and right:
		if game_data[Pos.y][Pos.x + 1] in numbers:
			surrounding.Right = game_data[Pos.y][Pos.x + 1];
	
	return surrounding;

func get_symbol(cells:Array):
	cells = cells.filter(func(symbol): return symbol != '.' and symbol not in numbers);
	return cells;

func has_number(cells:Array):
	cells = cells.filter(func(num): return num in numbers);
	return cells;

func find_number(pos:Vector2i, left:bool = true, right:bool = true):
	var num = game_data[pos.y][pos.x];
	var cells = get_left_right(pos, left, right);
	
	if cells.Left == '' and cells.Right == '' and (left or right):
		return '';
	
	# Numbers are only left or right
	if cells.Left == '' and cells.Right == '':
		return num;
	
	#if cells.Left != '' and cells.Right == '':
		#num = find_number(Vector2i(pos.x - 1, pos.y), true, false) + num;
	#
	#if cells.Right != '' and cells.Left == '':
		#num += find_number(Vector2i(pos.x + 1, pos.y), false, true) + num;
	
	if cells.Left != '' and cells.Right != '':
		# Going to assume there are no 4 digit numbers
		return game_data[pos.y][pos.x - 1] + num + game_data[pos.y][pos.x + 1];
	
	if cells.Left != '' and cells.Right == '':
		var returnv = cells.Left + num;
		if game_data[pos.y][pos.x - 2] in numbers:
			returnv = game_data[pos.y][pos.x - 2] + returnv;
		
		return returnv;
	
	if cells.Left == '' and cells.Right != '':
		var returnv = num + cells.Right;
		if game_data[pos.y][pos.x + 2] in numbers:
			returnv += game_data[pos.y][pos.x + 2];
		
		return returnv;
	
	return num;
	

func process():
	var sum = 0;
	var line_pos = 0;
	for line in game_data:
		var element_pos = 0
		var temp_num = "";
		var add_to_sum = false;
		for element in line:
			# Process for a valid int
			if element.is_valid_int():
				temp_num += element;
				var cells = get_surrounding_cells(game_data, Vector2i(element_pos, line_pos));
				var symbol = get_symbol(cells);
				print(symbol)
				if len(symbol) > 0:
					add_to_sum = true;
				
				element_pos += 1;
				continue;
			
			# Otherwise, reset and add to sum
			if temp_num != "":
				if add_to_sum:
					print("Added: %s" % temp_num);
					sum += int(temp_num);
					add_to_sum = false;
				else:
					print("Didn't add %s" % temp_num);
				temp_num = "";
			
			element_pos += 1;
		
		if temp_num != "":
			if add_to_sum:
				print("Added: %s" % temp_num);
				sum += int(temp_num);
			else:
				print("Didn't add %s" % temp_num);
			
		line_pos += 1;
	$Result.text = "Result: %d" % sum;

func process2():
	var sum = 0;
	for line_pos in range(len(game_data)):
		var line = game_data[line_pos];
		for element_pos in range(len(line)):
			var element = line[element_pos]
			if element == "*":
				var cells = get_surrounding_cells(game_data, Vector2i(element_pos, line_pos));
				print(cells);
				if len(has_number(cells)) < 2:
					# Only one number (or less) which automatically breaks the rules
					continue;
				
				var numbers = [];
				# Middle first
				if cells[1].is_valid_int():
					numbers.append(find_number(Vector2i(element_pos, line_pos - 1)));
				else:
					# Then edges
					if cells[0].is_valid_int():
						numbers.append(find_number(Vector2i(element_pos - 1, line_pos - 1)));
					if cells[2].is_valid_int():
						numbers.append(find_number(Vector2i(element_pos + 1, line_pos - 1)));
				
				if cells[3].is_valid_int():
					numbers.append(find_number(Vector2i(element_pos - 1, line_pos)));
				if cells[4].is_valid_int():				
					numbers.append(find_number(Vector2i(element_pos + 1, line_pos)));
				
				# Middle first
				if cells[6].is_valid_int():
					numbers.append(find_number(Vector2i(element_pos, line_pos + 1)));
				else:
					# Then edges
					if cells[5].is_valid_int():
						numbers.append(find_number(Vector2i(element_pos - 1, line_pos + 1)));
					if cells[7].is_valid_int():
						numbers.append(find_number(Vector2i(element_pos + 1, line_pos + 1)));
				
				if len(numbers) > 2:
					continue;
					
				if len(numbers) == 1:
					continue;
				
				sum += int(numbers[0]) * int(numbers[1]);
				print("n1 %s n2 %s" % [numbers[0], numbers[1]])
				print(int(numbers[0]) * int(numbers[1]));				

	$Result.text = "Result: %d" % sum;

func start():
	game_data = Utils.get_data(3, data, part);
	if part == utils.Part.Part1:
		process();
		return;
	
	process2();
