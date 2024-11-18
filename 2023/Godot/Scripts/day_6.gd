extends Node2D
var Utils = utils.new();
@export var data:utils.Data;
@export var part:utils.Part;
var game_data:PackedStringArray;
var races:Array;

func _ready():
	game_data = Utils.get_data(6, data, part);
	get_races();
	print(races);

func get_races():
	races = [];
	var time_info = game_data[0].replace('Time:', '').split(' ', false);
	var distance_info = game_data[1].replace('Distance:', '').split(' ', false);
	
	if part == utils.Part.Part1:
		for i in range(len(time_info)):
			races.append([int(time_info[i]), int(distance_info[i])]);
	else:
		races = [[int(''.join(time_info)), int(''.join(distance_info))]];

func process_race(race_data:Array):
	var record_destroyed:int = 0;
	for number in race_data[0]:
		var max_distance = (race_data[0] - number) * number;
		if max_distance > race_data[1]:
			record_destroyed += 1;
	
	return record_destroyed;

func start():
	var max = -1;
	for race in races:
		var race_result = process_race(race);
		if max == -1:
			max = race_result;
		else:
			max *= race_result;
	
	print(max);
