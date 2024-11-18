from helper import helper, math, AOCException

class Main:
	def generate_json(self, start, end):
		s_to_e = {}
		for line in range(start, end):
			lineInfo = self.game_data[line].split(' ')
			s_to_e[int(lineInfo[1])] = {
				"range": int(lineInfo[2]),
				"var": int(lineInfo[0])
			}

		return s_to_e

	def process_data(self):
		self.seeds = self.game_data[0].split(':')[1].split(' ')
		ss = self.game_data.find('seed-to-soil map:')
		sf = self.game_data.find('soil-to-fertilizer map:')
		fw = self.game_data.find('fertilizer-to-water map:')
		wl = self.game_data.find('water-to-light map:')
		lt = self.game_data.find('light-to-temperature map:')
		th = self.game_data.find('temperature-to-humidity map:')
		hl = self.game_data.find

		self.seeds_to_soil = self.generate_json(


if __name__ == "__main__":
    h = helper(-10)
    dayData = h.main()
