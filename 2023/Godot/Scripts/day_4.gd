extends Node2D
@export var part:utils.Part;
@export var data:utils.Data;
var Utils = utils.new();
var game_data;
var game_index:int = 0;

func process_game(info:String) -> int:
	$GameInfo.text = info;
	var game_info = info.split(":");
	var game_id = game_info[0];
	var gdata = game_info[1].split('|');
	var winnings = gdata[0].split(' ', false);
	var numbers = gdata[1].split(' ', false);
	$Winnings.text = "Winnings: %s" % str(winnings);
	$Numbers.text = "Numbers: %s" % str(numbers);
	
	var score = 0;
	
	for num in winnings:
		if numbers.has(num):
			if score == 0:
				score = 1;
				continue;
			
			score *= 2;
			continue;
	
	$Score.text = "Score: %d" % score;
	return score;

var copies = {};
func process_game_copies(info:String):
	$GameInfo.text = info;
	var game_info = info.split(":");
	var game_id = int(game_info[0]);
	var gdata = game_info[1].split('|');
	var winnings = gdata[0].split(' ', false);
	var numbers = gdata[1].split(' ', false);
	$Winnings.text = "Winnings: %s" % str(winnings);
	$Numbers.text = "Numbers: %s" % str(numbers);
	
	if copies.get(str(game_id)) == null:
		copies[str(game_id)] = 1;

	var desired_copies = 0;
	
	for num in winnings:
		if numbers.has(num):
			# Depending on amount of copies of the card we have
			desired_copies += 1;
	
	for i in range(game_id + 1, desired_copies + game_id + 1):
		if i > len(game_data) - 1:
			continue;
		
		if copies.get(str(i)) == null:
			copies[str(i)] = 1 + copies[str(game_id)];
			continue;

		copies[str(i)] += copies[str(game_id)];
	
	$Score.text = str(desired_copies);
	$Result.text = str(copies);

func arr_sum(arr):
	var sum = 0;
	for num in arr:
		sum += num;
	return sum;

func next():
	if part == utils.Part.Part1:
		process_game(game_data[game_index]);
	
	if part == utils.Part.Part2:
		process_game_copies(game_data[game_index]);
	
	game_index += 1;

func start():
	var score = 0;
	for line in game_data:
		if part == utils.Part.Part1:
			score += process_game(line);
			continue;
		
		process_game_copies(line);
	
	if part == utils.Part.Part1:
		$Result.text = "Result: %d" % score;
		return;
	
	$Score.text = "Score: %s" % JSON.stringify(copies);
	$Result.text = "Result: %d" % arr_sum(copies.values());
		

# Called when the node enters the scene tree for the first time.
func _ready():
	game_data = Utils.get_data(4, data, part);
