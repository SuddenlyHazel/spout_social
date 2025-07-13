//! Drunken Bishop algorithm implementation for generating visual fingerprints
//!
//! This module provides a way to generate ASCII art representations of hex fingerprints
//! using the "drunken bishop" algorithm, similar to SSH key fingerprints.

use bincode::{Encode, config};

/// Default character set for the drunken bishop visualization
pub const DEFAULT_CHARSET: &[&str] = &[
    "\u{A828}", "\u{A829}", "\u{A82A}", "\u{A82B}", "\u{2500}", // Horizontal bar
    " ",        // Space
    "┼",        // Box cross
    // Above this I am very happy with
    "\u{25AC}", // fat horizontal box
    "\u{2349}", // slash o
    "\u{2341}", // slash box
    "\u{25A2}", // round box
    "\u{2297}", // x circle
    "❖", "█", "◎", "\u{2592}", // half shade
    "\u{2715}", // little x
    "◠", "◫", "◡",
];

/* Cool characters:
 * \u{a28b} ꠫
 * \u{a28a} ꠪
 * \u{a289} ꠩
 * \u{a288} ꠨
 * \u{4DC0}-\u{4DFF} ䷅ etc.
 * \u{}
 * \u{}
 */

/// Default end character to mark the final position
pub const DEFAULT_END_CHAR: char = '\u{A52A}'; //'✱';

/// Border charset. N, NE, E, SE, S, SW, W, NW
pub const DEFAULT_BORDER: (&str, &str, &str, &str, &str, &str, &str, &str) =
    ("─", "╮", "│", "╯", "─", "╰", "│", "╭");

/// Characters that should connect with borders and their corresponding border characters
/// (sequence to match, north border, east border, south border, west border)
pub const DEFAULT_BORDER_OVERRIDES: &[(&str, &str, &str, &str, &str)] = &[
    ("┼", "┬", "┤", "┴", "├"),
    ("┬", "─", "┤", "┴", "├"),
    ("┤", "┬", "│", "┴", "├"),
    ("┴", "┬", "┤", "─", "├"),
    ("├", "┬", "┤", "┴", "│"),
];

/// Direction mappings for bishop movement
const DIRECTIONS: [(i8, i8); 4] = [
    (-1, -1), // 00
    (1, -1),  // 01
    (-1, 1),  // 10
    (1, 1),   // 11
];

/// Configuration for the drunken bishop algorithm
#[derive(Debug, Clone)]
pub struct DrunkenBishopConfig {
    /// Width of one quadrant (total width will be width * 2)
    pub width: usize,
    /// Height of one quadrant (total height will be height)
    pub height: usize,
    /// Character set to use for different visit counts
    pub charset: Vec<String>,
    /// Character to mark the end position
    pub end_char: char,
    /// Border charset: (N, NE, E, SE, S, SW, W, NW)
    pub border: (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ),
    /// Border overrides: (sequence to match, north, east, south, west)
    pub border_overrides: Vec<(String, String, String, String, String)>,
}

impl Default for DrunkenBishopConfig {
    fn default() -> Self {
        Self {
            width: 9,
            height: 4,
            charset: DEFAULT_CHARSET.iter().map(|s| s.to_string()).collect(),
            end_char: DEFAULT_END_CHAR,
            border: (
                DEFAULT_BORDER.0.to_string(),
                DEFAULT_BORDER.1.to_string(),
                DEFAULT_BORDER.2.to_string(),
                DEFAULT_BORDER.3.to_string(),
                DEFAULT_BORDER.4.to_string(),
                DEFAULT_BORDER.5.to_string(),
                DEFAULT_BORDER.6.to_string(),
                DEFAULT_BORDER.7.to_string(),
            ),
            border_overrides: DEFAULT_BORDER_OVERRIDES
                .iter()
                .map(|(chars, north, east, south, west)| {
                    (
                        chars.to_string(),
                        north.to_string(),
                        east.to_string(),
                        south.to_string(),
                        west.to_string(),
                    )
                })
                .collect(),
        }
    }
}

/// Drunken Bishop algorithm implementation
pub struct DrunkenBishop {
    config: DrunkenBishopConfig,
}

impl DrunkenBishop {
    /// Create a new DrunkenBishop with default configuration
    pub fn new() -> Self {
        Self {
            config: DrunkenBishopConfig::default(),
        }
    }

    /// Create a new DrunkenBishop with custom configuration
    pub fn with_config(config: DrunkenBishopConfig) -> Self {
        Self { config }
    }

    /// Generate a grid from a hex fingerprint
    pub fn generate_grid(&self, fingerprint: &str) -> Vec<Vec<GridCell>> {
        let clean_hex = fingerprint.replace(":", "");
        let moves = self.extract_moves_from_hex(&clean_hex);
        self.generate_grid_from_moves(moves)
    }

    /// Generate a grid from any serializable object
    pub fn generate_grid_from_object<T: Encode>(
        &self,
        object: &T,
    ) -> Result<Vec<Vec<GridCell>>, Box<dyn std::error::Error>> {
        let bytes = bincode::encode_to_vec(object, config::standard())?;
        let moves = self.extract_moves_from_bytes(&bytes);
        Ok(self.generate_grid_from_moves(moves))
    }

    /// Generate a grid from raw bytes
    pub fn generate_grid_from_bytes(&self, bytes: &[u8]) -> Vec<Vec<GridCell>> {
        let moves = self.extract_moves_from_bytes(bytes);
        self.generate_grid_from_moves(moves)
    }

    /// Internal method to generate grid from move sequence
    fn generate_grid_from_moves(&self, moves: Vec<u8>) -> Vec<Vec<GridCell>> {
        let mut grid = vec![vec![GridCell::Count(0); self.config.width]; self.config.height];
        let mut bishop = [self.config.width / 2, self.config.height / 2];
        let mut _total_moves = 0;

        for i in 0..moves.len() {
            // Forward move
            let direction = DIRECTIONS[moves[i] as usize];
            bishop[0] = self.clamp_x(bishop[0] as i32 + direction.0 as i32) as usize;
            bishop[1] = self.clamp_y(bishop[1] as i32 + direction.1 as i32) as usize;

            if let GridCell::Count(count) = grid[bishop[1]][bishop[0]] {
                grid[bishop[1]][bishop[0]] = GridCell::Count(count + 1);
            }
            _total_moves += 1;

            // Reverse move (with flipped direction)
            let reverse_move = moves[moves.len() - 1 - i];
            let reverse_direction = DIRECTIONS[reverse_move as usize];
            bishop[0] = self.clamp_x(bishop[0] as i32 + reverse_direction.1 as i32) as usize; // flipped on purpose
            bishop[1] = self.clamp_y(bishop[1] as i32 + reverse_direction.0 as i32) as usize;

            if let GridCell::Count(count) = grid[bishop[1]][bishop[0]] {
                grid[bishop[1]][bishop[0]] = GridCell::Count(count + 1);
            }
            _total_moves += 1;
        }

        // Mark the end position
        grid[bishop[1]][bishop[0]] = GridCell::End;

        grid
    }

    /// Render the grid as a string with borders and mirroring
    pub fn render_grid(&self, grid: &[Vec<GridCell>]) -> String {
        let mut result = String::new();
        let charset_strings = &self.config.charset;

        // Create the main grid content
        let mut grid_lines = Vec::new();

        for row in grid {
            let mut line = String::from('├');

            // Left half
            for cell in row {
                match cell {
                    GridCell::End => line.push(self.config.end_char),
                    GridCell::Count(count) => {
                        if charset_strings.is_empty() {
                            line.push(' ');
                        } else {
                            line.push_str(&charset_strings[*count % charset_strings.len()]);
                        }
                    }
                };
            }

            // Right half (mirrored)
            for cell in row.iter().rev() {
                match cell {
                    GridCell::End => line.push(self.config.end_char),
                    GridCell::Count(count) => {
                        if charset_strings.is_empty() {
                            line.push(' ');
                        } else {
                            line.push_str(&charset_strings[*count % charset_strings.len()]);
                        }
                    }
                };
            }

            line.push('┤');
            grid_lines.push(line);
        }

        // Create top border with smart connections
        let mut top_border = String::new();
        top_border.push_str(&self.config.border.7); // NW corner
        if let Some(first_line) = grid_lines.first() {
            for i in 1..first_line.len() - 1 {
                let char_below = first_line.chars().nth(i).unwrap_or('─');
                let border_char = self.get_border_char(char_below, "north");
                top_border.push_str(&border_char);
            }
        } else {
            for _ in 0..self.config.width * 2 {
                top_border.push_str(&self.config.border.0); // N
            }
        }
        top_border.push_str(&self.config.border.1); // NE corner

        // Create bottom border with smart connections
        let mut bottom_border = String::new();
        bottom_border.push_str(&self.config.border.5); // SW corner
        if let Some(last_line) = grid_lines.last() {
            for i in 1..last_line.len() - 1 {
                let char_above = last_line.chars().nth(i).unwrap_or('─');
                let border_char = self.get_border_char(char_above, "south");
                bottom_border.push_str(&border_char);
            }
        } else {
            for _ in 0..self.config.width * 2 {
                bottom_border.push_str(&self.config.border.4); // S
            }
        }
        bottom_border.push_str(&self.config.border.3); // SE corner

        // Combine everything
        result.push_str(&top_border);
        result.push('\n');

        for line in &grid_lines {
            result.push_str(line);
            result.push('\n');
        }

        // Add mirrored bottom half
        for line in grid_lines.iter().rev() {
            result.push_str(line);
            result.push('\n');
        }

        result.push_str(&bottom_border);

        result
    }

    /// Extract move pairs from hex string
    fn extract_moves_from_hex(&self, hex: &str) -> Vec<u8> {
        let mut moves = Vec::new();

        for chunk in hex.as_bytes().chunks(2) {
            let hex_str = std::str::from_utf8(chunk).unwrap_or("00");
            if let Ok(byte_val) = u8::from_str_radix(hex_str, 16) {
                let binary = format!("{:08b}", byte_val);
                for pair in binary.as_bytes().chunks(2) {
                    if pair.len() == 2 {
                        let pair_str = std::str::from_utf8(pair).unwrap_or("00");
                        if let Ok(move_val) = u8::from_str_radix(pair_str, 2) {
                            moves.push(move_val);
                        }
                    }
                }
            }
        }

        moves
    }

    /// Extract move pairs from raw bytes
    fn extract_moves_from_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut moves = Vec::new();

        for &byte_val in bytes {
            let binary = format!("{:08b}", byte_val);
            for pair in binary.as_bytes().chunks(2) {
                if pair.len() == 2 {
                    let pair_str = std::str::from_utf8(pair).unwrap_or("00");
                    if let Ok(move_val) = u8::from_str_radix(pair_str, 2) {
                        moves.push(move_val);
                    }
                }
            }
        }

        moves
    }

    /// Clamp X coordinate to grid bounds
    fn clamp_x(&self, x: i32) -> i32 {
        x.clamp(0, self.config.width as i32 - 1)
    }

    /// Clamp Y coordinate to grid bounds
    fn clamp_y(&self, y: i32) -> i32 {
        y.clamp(0, self.config.height as i32 - 1)
    }

    /// Get the appropriate border character for connecting with a grid character
    fn get_border_char(&self, grid_char: char, direction: &str) -> String {
        for (chars, north, east, south, west) in &self.config.border_overrides {
            if chars.contains(grid_char) {
                return match direction {
                    "north" => north.clone(),
                    "east" => east.clone(),
                    "south" => south.clone(),
                    "west" => west.clone(),
                    _ => self.config.border.0.clone(), // default to N
                };
            }
        }
        match direction {
            "north" | "south" => self.config.border.0.clone(), // N/S horizontal line
            "east" | "west" => self.config.border.2.clone(),   // E/W vertical line
            _ => self.config.border.0.clone(),                 // default
        }
    }
}

impl Default for DrunkenBishop {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a cell in the grid
#[derive(Debug, Clone, Copy)]
pub enum GridCell {
    /// Number of times this cell was visited
    Count(usize),
    /// End position marker
    End,
}

/// Convenience function to generate a drunken bishop visualization with default settings
pub fn generate_fingerprint(fingerprint: &str) -> String {
    let bishop = DrunkenBishop::new();
    let grid = bishop.generate_grid(fingerprint);
    bishop.render_grid(&grid)
}

/// Convenience function to generate a drunken bishop visualization with custom size
pub fn generate_fingerprint_with_size(fingerprint: &str, width: usize, height: usize) -> String {
    let config = DrunkenBishopConfig {
        width,
        height,
        ..Default::default()
    };
    let bishop = DrunkenBishop::with_config(config);
    let grid = bishop.generate_grid(fingerprint);
    bishop.render_grid(&grid)
}

/// Convenience function to generate a drunken bishop visualization with custom charset
pub fn generate_fingerprint_with_charset(fingerprint: &str, charset: &[&str]) -> String {
    let config = DrunkenBishopConfig {
        charset: charset.iter().map(|s| s.to_string()).collect(),
        ..Default::default()
    };
    let bishop = DrunkenBishop::with_config(config);
    let grid = bishop.generate_grid(fingerprint);
    bishop.render_grid(&grid)
}

/// Convenience function to generate a drunken bishop visualization with custom border overrides
pub fn generate_fingerprint_with_border_overrides(
    fingerprint: &str,
    border_overrides: &[(&str, &str, &str, &str, &str)],
) -> String {
    let config = DrunkenBishopConfig {
        border_overrides: border_overrides
            .iter()
            .map(|(chars, north, east, south, west)| {
                (
                    chars.to_string(),
                    north.to_string(),
                    east.to_string(),
                    south.to_string(),
                    west.to_string(),
                )
            })
            .collect(),
        ..Default::default()
    };
    let bishop = DrunkenBishop::with_config(config);
    let grid = bishop.generate_grid(fingerprint);
    bishop.render_grid(&grid)
}

/// Convenience function to generate a drunken bishop visualization from any serializable object
pub fn generate_fingerprint_from_object<T: Encode>(
    object: &T,
) -> Result<String, Box<dyn std::error::Error>> {
    let bishop = DrunkenBishop::new();
    let grid = bishop.generate_grid_from_object(object)?;
    Ok(bishop.render_grid(&grid))
}

/// Convenience function to generate a drunken bishop visualization from raw bytes
pub fn generate_fingerprint_from_bytes(bytes: &[u8]) -> String {
    let bishop = DrunkenBishop::new();
    let grid = bishop.generate_grid_from_bytes(bytes);
    bishop.render_grid(&grid)
}
