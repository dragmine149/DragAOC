import helper

class Rope:
    def __init__(self, instructions) -> None:
        self.bridge = [[]]
        self.instructions = instructions
        self.HeadPos = {"X": 0, "Y": 0}
        self.TailPos = {"X": 0, "Y": 0}
    
    def movePieceRight(self):
        if self.HeadPos["X"] + 1 > len(self.bridge[0]):
            for row in self.bridge:
                row.append(".")
        
        self.HeadPos["X"] += 1
        
        # Tail position check
        if self.TailPos["X"]
        
    
    def TranslateInstruction(self, instruction: str):
        direction, amount = instruction.split(" ")
        
        if direction == "R":
            self.HeadPos["X"] += amount
        
        if direction == "L":
            self.HeadPos["X"] -= amount
        
        if direction == "U":
            self.HeadPos["Y"] += amount
        
        if direction == "D":
            self.HeadPos["Y"] -= amount
    
    def main(self):
        

if __name__ == "__main__":
    data = helper.main()