extends Resource;
class_name utils;

enum Data { Generated, Default, Test };
@export var selected:Data = Data.Generated;
enum Part { Part1, Part2 };
@export var selected_part:Part = Part.Part1;

func get_data_path(Day:int) -> String:
	match selected:
		Data.Test:
			return "res://Data/Day%d/Test.txt" % Day;
		Data.Generated:
			return "res://Data/Day%d/Generated.txt" % Day;

	if selected_part == Part.Part2:
		return "res://Data/Day%d/Default2.txt" % Day;
	return "res://Data/Day%d/Default.txt" % Day;

func get_data(Day:int, selected:Data, selected_part:Part):
	self.selected = selected;
	self.selected_part = selected_part;
	print(get_data_path(Day));
	var data = FileAccess.open(get_data_path(Day), FileAccess.READ);
	return data.get_as_text().split('\n', false);
