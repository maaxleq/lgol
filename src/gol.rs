#[derive(Clone, PartialEq, Copy)]
pub enum TileState {
    DEAD,
    ALIVE,
}

#[derive(Clone)]
pub struct Tile {
    state: TileState,
}

#[derive(Clone)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    size_x: u16,
    size_y: u16,
}

pub struct Game {
    main_grid: Grid,
    temp_grid: Grid,
}

fn new_tileset(size_x: u16, size_y: u16) -> Vec<Vec<Tile>> {
    return vec![
        vec![
            Tile {
                state: TileState::DEAD
            };
            size_y as usize
        ];
        size_x as usize
    ];
}

fn new_grid(size_x: u16, size_y: u16) -> Grid {
    return Grid {
        tiles: new_tileset(size_x, size_y),
        size_x: size_x,
        size_y: size_y,
    };
}

pub fn new_game(size_x: u16, size_y: u16) -> Game {
    return Game {
        main_grid: new_grid(size_x, size_y),
        temp_grid: new_grid(size_x, size_y),
    };
}

impl Tile {
    pub fn get_state(&self) -> TileState {
        return self.state;
    }
}

impl Grid {
    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        return &self.tiles;
    }
    pub fn get_size_x(&self) -> u16 {
        return self.size_x;
    }

    pub fn get_size_y(&self) -> u16 {
        return self.size_y;
    }

    fn clean(&mut self) {
        self.tiles = new_tileset(self.size_x, self.size_y);
    }

    fn get_alive_neighbors_count(&self, x: u16, y: u16) -> u8 {
        let mut count: u8 = 0;

        let ix = i32::from(x);
        let iy = i32::from(y);

        let potential_neighbors_pos = [
            (ix - 1, iy - 1),
            (ix, iy - 1),
            (ix + 1, iy - 1),
            (ix - 1, iy),
            (ix + 1, iy),
            (ix - 1, iy + 1),
            (ix, iy + 1),
            (ix + 1, iy + 1),
        ];

        for pos in potential_neighbors_pos.iter() {
            if pos.0 >= 0
                && pos.0 < i32::from(self.size_x)
                && pos.1 >= 0
                && pos.1 < i32::from(self.size_y)
            {
                if self.tiles[pos.0 as usize][pos.1 as usize].state == TileState::ALIVE {
                    count += 1;
                }
            }
        }

        return count;
    }
}

impl Game {
    pub fn get_main_grid(&self) -> &Grid {
        return &self.main_grid;
    }

    fn write_next_state_on_temp(&mut self) {
        for x in 0..self.main_grid.size_x {
            for y in 0..self.main_grid.size_y {
                let tile = &self.main_grid.tiles[x as usize][y as usize];
                let alive_neightbors_count = self.main_grid.get_alive_neighbors_count(x, y);
                let new_state: TileState;

                if tile.state == TileState::ALIVE {
                    if alive_neightbors_count == 2 || alive_neightbors_count == 3 {
                        new_state = TileState::ALIVE
                    } else {
                        new_state = TileState::DEAD
                    }
                } else {
                    if alive_neightbors_count == 3 {
                        new_state = TileState::ALIVE
                    } else {
                        new_state = TileState::DEAD
                    }
                }

                self.temp_grid.tiles[x as usize][y as usize] = Tile { state: new_state }
            }
        }
    }

    fn write_temp_on_main(&mut self) {
        self.main_grid = self.temp_grid.clone();
    }

    fn clean_temp(&mut self) {
        self.temp_grid.clean();
    }

    pub fn next(&mut self) {
        self.write_next_state_on_temp();
        self.write_temp_on_main();
        self.clean_temp();
    }

    pub fn randomize(&mut self) {
        for x in 0..self.main_grid.size_x {
            for y in 0..self.main_grid.size_y {
                let state: TileState;

                if rand::random() {
                    state = TileState::ALIVE;
                } else {
                    state = TileState::DEAD;
                }

                self.main_grid.tiles[x as usize][y as usize] = Tile { state: state }
            }
        }
    }
}
