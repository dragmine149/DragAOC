extends Node2D
var Utils = utils.new();
@export var data:utils.Data;
@export var part:utils.Part;
var game_data:PackedStringArray;
var seed_index:int = 0;

# This is bit of a cheat
var numbers_to_search = 1_606_226_378;

# Called when the node enters the scene tree for the first time.
func _ready():
	game_data = Utils.get_data(5, data, part);
	process_data();

func generate_json(start, end) -> Dictionary:
	var s_to_e = {
		# "source-source end": "destination-destination end"
	};
	for line in range(start, end):
		var lineInfo = game_data[line].split(' ');
		#s_to_e["%s-%s" % [lineInfo[1], int(lineInfo[1]) + (int(lineInfo[2]) - 1)]] = int(lineInfo[0]);
		
		#s_to_e[int(lineInfo[1])] = {
			#"range": int(lineInfo[2]),
			#"var": int(lineInfo[0])
		#};
		s_to_e[int(lineInfo[1])] = [int(lineInfo[2]), int(lineInfo[0])];
	
	return s_to_e;

var seeds:PackedStringArray;
var seeds_to_soil:Dictionary;
var soil_to_fertilizer:Dictionary;
var fertilizer_to_water:Dictionary;
var water_to_light:Dictionary;
var light_to_temperature:Dictionary;
var temperature_to_humidity:Dictionary;
var humidity_to_location:Dictionary;
var seeds_to_soil_arr:Array;
var soil_to_fertilizer_arr:Array;
var fertilizer_to_water_arr:Array;
var water_to_light_arr:Array;
var light_to_temperature_arr:Array;
var temperature_to_humidity_arr:Array;
var humidity_to_location_arr:Array;

func process_data():
	seeds = game_data[0].split(':')[1].split(' ', false);	
	var ss = game_data.find('seed-to-soil map:');
	var sf = game_data.find('soil-to-fertilizer map:');
	var fw = game_data.find('fertilizer-to-water map:');
	var wl = game_data.find('water-to-light map:');
	var lt = game_data.find('light-to-temperature map:');
	var th = game_data.find('temperature-to-humidity map:');
	var hl = game_data.find('humidity-to-location map:');
	seeds_to_soil = generate_json(ss + 1, sf);
	seeds_to_soil_arr = seeds_to_soil.keys();
	seeds_to_soil_arr.sort();
	soil_to_fertilizer = generate_json(sf + 1, fw);
	soil_to_fertilizer_arr = soil_to_fertilizer.keys();
	soil_to_fertilizer_arr.sort();
	fertilizer_to_water = generate_json(fw + 1, wl);
	fertilizer_to_water_arr = fertilizer_to_water.keys();
	fertilizer_to_water_arr.sort();
	water_to_light = generate_json(wl + 1, lt);
	water_to_light_arr = water_to_light.keys();
	water_to_light_arr.sort();
	light_to_temperature = generate_json(lt + 1, th);
	light_to_temperature_arr = light_to_temperature.keys();
	light_to_temperature_arr.sort();
	temperature_to_humidity = generate_json(th + 1, hl);
	temperature_to_humidity_arr = temperature_to_humidity.keys();
	temperature_to_humidity_arr.sort();
	humidity_to_location = generate_json(hl + 1, len(game_data) - 1);
	humidity_to_location_arr = humidity_to_location.keys();
	humidity_to_location_arr.sort();
	
	#print(seeds);
	#print(seeds_to_soil);
	#print(soil_to_fertilizer);
	#print(fertilizer_to_water);
	#print(water_to_light);
	#print(light_to_temperature);
	#print(temperature_to_humidity);
	#print(humidity_to_location);

func get_item_from_dict(dictionary:Dictionary, array:Array, item:int):
	for key in array:
		#if item >= key and item <= key + dictionary.get(key).get("range") - 1:
		if item >= key and item <= key + dictionary.get(key)[0] - 1:
			var offset = item - key;
			return dictionary.get(key)[1] + offset;

	# In case of no match
	return item;

var locations = [];

func next():
	calc_seed(int(seeds[seed_index]));

var minimum:int = 999999999999999;
func calc_seed(seed:int):
	var start = Time.get_unix_time_from_system();

	#$Seed.text = "Seed: %d" % seed;
	var soil = get_item_from_dict(seeds_to_soil, seeds_to_soil_arr, seed);
	#$Soil.text = "Soil: %d" % soil;
	var fertilizer = get_item_from_dict(soil_to_fertilizer, soil_to_fertilizer_arr, soil);
	#$Fertilizer.text = "Fertilizer: %d" % fertilizer;
	var water = get_item_from_dict(fertilizer_to_water, fertilizer_to_water_arr, fertilizer);
	#$Water.text = "Water: %d" % water;
	var light = get_item_from_dict(water_to_light, water_to_light_arr, water);
	#$Light.text = "Light: %d" % light;
	var temperature = get_item_from_dict(light_to_temperature, light_to_temperature_arr, light);
	#$Temperature.text = "Temperature: %d" % temperature;
	var humidity = get_item_from_dict(temperature_to_humidity, temperature_to_humidity_arr, temperature);
	#$Humidity.text = "Humidity: %d" % humidity;
	var location = get_item_from_dict(humidity_to_location, humidity_to_location_arr, humidity);
	#$Location.text = "Location: %d" % location;
	seed_index += 1;
	
	if utils.Part.Part2 == part:
		if location < minimum:
			minimum = location;
	else:
		locations.append(location);
	
	var end = Time.get_unix_time_from_system();
	
	# Only print every 100mil
	#if seed_count % 100_000_000 == 0:
	if seed_count % 100_000 == 0:
	#if true:
		var secs = end - start;
		var mili = secs * 1000;
		var micro = mili * 1000;
		print("Seed took: %fs (%fms %fµs); Estimated time left: %fs; Items Searched: %d" % [secs, mili, micro, secs * (numbers_to_search - seed_count), seed_count]);

	if seed_index == len(seeds):
		if part == utils.Part.Part1:
			$Result.text = "Result: %d" % locations.min();
			return;
		
		$Result.text = "Result: %d" % minimum;
		return;

var seed_count = 0;
func start():
	if part == utils.Part.Part2:
		for index in range(len(seeds) - 1):
			if index % 2 == 1:
				continue;
			
			for length in range(int(seeds[index + 1])):
				seed_count += 1;
				calc_seed(int(seeds[index]) + length);
	
		return;
	
	for i in range(len(seeds) - 1):
		calc_seed(int(seeds[seed_index]));
	calc_seed(int(seeds[seed_index]));
