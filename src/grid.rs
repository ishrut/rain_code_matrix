use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

static CHARACTERS: [char; 11] = [' ', 'ｱ', 'ｲ', 'б', 'ｳ', 'ｴ', 'ｶ', 'ｷ', 'ｹ', 'ж', 'z'];
static CHAR_PROB: [i32; 11] = [30, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
static GREEN_COLORS: [u8; 12] = [022, 028, 034, 040, 046, 064, 070, 076, 082, 106, 112, 118];

pub struct Grid {
    width: u16,
    buffer: Vec<Vec<GridChar>>,
}

struct GridChar {
    item: char,
    style: Style,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let mut outer_buf = Vec::<Vec<GridChar>>::with_capacity(width.into());
        for _i in 0..width {
            let mut inner_buf = Vec::<GridChar>::with_capacity(height.into());
            for _j in 0..height {
                let dist = WeightedIndex::new(&CHAR_PROB).unwrap();
                let mut rng = rand::thread_rng();
                let random_color = rand::thread_rng().gen_range(0..=11);
                let character = GridChar {
                    item: CHARACTERS[dist.sample(&mut rng)],
                    style: Style::new().fg(Color::Indexed(GREEN_COLORS[random_color])),
                };
                inner_buf.push(character);
            }
            outer_buf.push(inner_buf);
        }

        Self {
            width,
            buffer: outer_buf,
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        let mut selected_cols = Vec::new();
        let random_num_cols = rng.gen_range(0..=self.width - 1);
        for _ in 0..random_num_cols {
            let rand_col = rng.gen_range(0..=self.width - 1);
            selected_cols.push(rand_col);
        }
        for col in selected_cols {
            self.buffer[col as usize].pop();
            let dist = WeightedIndex::new(&CHAR_PROB).unwrap();
            let random_color = rng.gen_range(0..=11);
            let character = GridChar {
                item: CHARACTERS[dist.sample(&mut rng)],
                style: Style::new().fg(Color::Indexed(GREEN_COLORS[random_color])),
            };
            self.buffer[col as usize].insert(0, character);
        }
    }
}

impl Widget for &Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = area.width;
        let height = area.height;

        for i in 0..width {
            for j in 0..height {
                buf.set_string(
                    i,
                    j,
                    String::from(self.buffer[i as usize][j as usize].item),
                    self.buffer[i as usize][j as usize].style,
                )
            }
        }
    }
}
