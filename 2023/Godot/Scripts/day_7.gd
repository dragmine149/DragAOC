extends Node2D
var Utils = utils.new();
@export var data:utils.Data;
@export var part:utils.Part;
var game_data:Array;
const strength = {
	"A": 13,
	"K": 12,
	"Q": 11,
	"J": 10,
	"T": 9,
	"9": 8,
	"8": 7,
	"7": 6,
	"6": 5,
	"5": 4,
	"4": 3,
	"3": 2,
	"2": 1
}
enum type {FIVE, FOUR, THREE, PAIR2, PAIR1, HIGH};
var games = {}

func _ready():
	var gd = Utils.get_data(7, data, part);
	game_data = Array(gd);
	start();

func get_type_from_regex(hand:String):
	var fiveEx = RegEx.new();
	fiveEx.compile(r"(A|K|Q|J|T|[2-9])+");
	var result = fiveEx.search(hand).get_string();
	if result.length() == 5:
		return type.FIVE;
	
	if result.length() == 4:
		return type.FOUR;
	
	if result.length() == 3:
		return type.THREE;
	
	var pair = RegEx.new();
	pair.compile(r"()+")
	

func start():
	for item in game_data:
		# Each item should be given it's own individual strength value.
		var info = item.split(" ");
		var hand:String = info[0];
		var hand_strength = 0;
		for letter in hand:
			hand_strength += strength.get(letter);
		
		get_type_from_regex(hand);
		
		games[info[0]] = {
			"Strength": 0,
			"bid": int(info[1])
		};
