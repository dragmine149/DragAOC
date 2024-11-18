extends Node2D
var Utils = utils.new();
@export var data:utils.Data;
@export var part:utils.Part;

var games = {}
var game_data;
var game_index:int = 0;

func _ready():
	game_data = Utils.get_data(2, data, part);

func process_game(game:String):
	$Game.text = game;
	var game_info:PackedStringArray = game.split(":");
	var game_id = game_info[0].trim_prefix("Game ");
	$ID.text = "ID: %s" % game_id;
	var games = game_info[1].split(";", false);
	$Cubes.text = "Data: %s" % str(games);
	
	var minimum = {
		"red": 0,
		"green": 0,
		"blue": 0
	}
	
	for pull in games:
		var cube_colours = pull.split(",");
		# NOTE: This uses a feature of godot where convert a string to an int automatically
		# 		removes all of the letters instead of raising an error.
		for cube in cube_colours:
			if cube.contains("red"):
				if int(cube) > 12 and part == utils.Part.Part1:
					return {game_id: false}; # no need to check the others
				
				if int(cube) > minimum.red:
					minimum.red = int(cube);
			
			if cube.contains("green"):
				if int(cube) > 13 and part == utils.Part.Part1:
					return {game_id: false};
				
				if int(cube) > minimum.green:
					minimum.green = int(cube);
			
			if cube.contains("blue"):
				if int(cube) > 14 and part == utils.Part.Part1:
					return {game_id: false};
				
				if int(cube) > minimum.blue:
					minimum.blue = int(cube);
	
	if part == utils.Part.Part1:
		return {game_id: true};
	
	return {game_id: minimum.red * minimum.green * minimum.blue};
	
func start():
	game_index = 0;
	for _v in game_data:
		next();

func next():
	if game_index == len(game_data) - 1:
		$Next.disabled = true;
		result();
		return;

	var result = process_game(game_data[game_index]);
	games.merge(result);
	game_index += 1;
	$Allowed.text = "Allowed?: %s" % str(result);
	

func result():
	var sum = 0;
	for game in games:
		if part == utils.Part.Part1:
			sum += int(game) if games.get(str(game)) else 0;
			continue;
		
		sum += games.get(game);
	
	$Result.text = "Result: %d" % sum;
