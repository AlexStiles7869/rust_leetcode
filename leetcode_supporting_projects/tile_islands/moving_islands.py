class TileSet:
    def __init__(self, tiles):
        self.tiles = tiles 

    def __str__(self):
        string_rep = ""
        for tile_row in self.tiles:
            for (i, tile) in enumerate(tile_row):
                if i != 0:
                    string_rep += " "
                string_rep += f"{tile}"
            string_rep += '\n'
        return string_rep[:-1]

    def place_tile(self, tile, location):
        if self.tiles[location[0]][location[1]] == '1':
            print("Moving existing tiles to fit new tile")
            # Tiles need to be rearranged to allow for new tile to fit in


        else:
            self.tiles[location[0]][location[1]] = tile


if __name__ == "__main__":
    tile_set = TileSet([['0','0','1'], ['0', '0', '0'], ['0', '0', '0']])
    print(tile_set)
    tile_set.place_tile('1', (0, 2))
    print(tile_set)



