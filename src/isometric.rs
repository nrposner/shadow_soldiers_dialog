// I have commented out a couple of functions bc I'm not really testing the navigation gui now and they were throwing errors

#[allow(dead_code)]
pub struct IsometricSpace {
    pub grid_size: (usize, usize),
    pub tile_size: (f32, f32),  // Tile size for the isometric grid
    pub player_position: (usize, usize),  // Player position on the grid
    pub player_target_position: Option<(usize, usize)>,  // Where the player is moving to
    pub player_anim_progress: f32,  // Progress of the animation (from 0.0 to 1.0)
}

impl Default for IsometricSpace {
    fn default() -> Self {
        Self {
            grid_size: (10, 10),  // A 10x10 grid
            tile_size: (64.0, 32.0),  // Isometric tile size
            player_position: (5, 5),  // Player starts in the center
            player_target_position: None,
            player_anim_progress: 1.0,
        }
    }
}

#[allow(dead_code)]
impl IsometricSpace {

    pub fn start_player_move(&mut self, target_position: (usize, usize)) {
        self.player_target_position = Some(target_position);
        self.player_anim_progress = 0.0;  // Reset animation progress
    }

    pub fn animate_player(&mut self, delta_time: f32) {
        if let Some(target) = self.player_target_position {
            // Increment the animation progress over time
            self.player_anim_progress += delta_time * 2.0;  // Adjust speed by multiplying delta_time

            // Cap the progress at 1.0
            if self.player_anim_progress >= 1.0 {
                self.player_anim_progress = 1.0;
                self.player_position = target;  // Move the player to the final position
                self.player_target_position = None;  // Clear the target, as movement is complete
            }
        }
    }

    // Drawing the isometric grid can use &self since it doesn't modify the state
    pub fn draw_isometric_grid(&self, ui: &mut egui::Ui) {
        for row in 0..self.grid_size.0 {
            for col in 0..self.grid_size.1 {
                // Calculate the isometric position of the tile
                let x = (col as f32 - row as f32) * (self.tile_size.0 / 2.0);
                let y = (col as f32 + row as f32) * (self.tile_size.1 / 2.0);

                // Draw the isometric tile (diamond shape)
                let points = [
                    egui::pos2(x, y - (self.tile_size.1 / 2.0)),             // Top point
                    egui::pos2(x + (self.tile_size.0 / 2.0), y),              // Right point
                    egui::pos2(x, y + (self.tile_size.1 / 2.0)),              // Bottom point
                    egui::pos2(x - (self.tile_size.0 / 2.0), y),              // Left point
                ];
                ui.painter().add(egui::Shape::closed_line(
                    points.to_vec(),
                    egui::Stroke::new(1.0, egui::Color32::GRAY),
                ));
            }
        }
    }

    pub fn draw_player(&self, ui: &mut egui::Ui) {
        let (start_row, start_col) = self.player_position;
        let (end_row, end_col) = self.player_target_position.unwrap_or(self.player_position);

        // Interpolate between start and end positions
        let row = start_row as f32 * (1.0 - self.player_anim_progress) + end_row as f32 * self.player_anim_progress;
        let col = start_col as f32 * (1.0 - self.player_anim_progress) + end_col as f32 * self.player_anim_progress;

        let x = (col - row - 1.0) * (self.tile_size.0 / 2.0);
        let y = (col + row - 1.0) * (self.tile_size.1 / 2.0);

        let player_rect = egui::Rect::from_min_size(
            egui::pos2(x + self.tile_size.0 / 4.0, y + self.tile_size.1 / 4.0),
            egui::vec2(self.tile_size.0 / 2.0, self.tile_size.1 / 2.0),
        );

        ui.painter().rect_filled(player_rect, 0.0, egui::Color32::RED);
    }

    // Handling mouse movement requires mutability because it modifies player_position
    // pub fn handle_mouse_movement(&mut self, ctx: &egui::Context) {
    //     if ctx.input().pointer.any_click() {
    //         if let Some(mouse_pos) = ctx.input().pointer.interact_pos() {
    //             // Adjusted isometric grid coordinate calculation
    //             let tile_x = (mouse_pos.x / self.tile_size.0 + mouse_pos.y / self.tile_size.1) / 2.0;
    //             let tile_y = (mouse_pos.y / self.tile_size.1 - (mouse_pos.x / self.tile_size.0)) / 2.0;

    //             let row = tile_y.floor() as usize;
    //             let col = tile_x.floor() as usize;

    //             // Ensure the clicked position is within grid bounds
    //             if row < self.grid_size.0 && col < self.grid_size.1 {
    //                 // Start moving the player to the clicked tile
    //                 self.start_player_move((row, col));
    //             }
    //         }
    //     }
    // }

    // pub fn handle_hover(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> Option<(usize, usize)> {
    //     if let Some(mouse_pos) = ctx.input().pointer.interact_pos() {
    //         // Convert mouse position to grid coordinates
    //         let tile_x = (mouse_pos.x / (self.tile_size.0 / 2.0) + mouse_pos.y / (self.tile_size.1 / 2.0)) / 2.0;
    //         let tile_y = (mouse_pos.y / (self.tile_size.1 / 2.0) - (mouse_pos.x / (self.tile_size.0 / 2.0))) / 2.0;

    //         let row = tile_y as usize;
    //         let col = tile_x as usize;

    //         // Ensure the hovered tile is within bounds
    //         if row < self.grid_size.0 && col < self.grid_size.1 {
    //             return Some((row, col));  // Return the hovered tile coordinates
    //         }
    //     }
    //     None  // No tile hovered
    // }

    pub fn draw_hovered_tile(&self, ui: &mut egui::Ui, hovered_tile: Option<(usize, usize)>) {
        if let Some((row, col)) = hovered_tile {
            // Calculate isometric position
            let x = (col as f32 - row as f32) * (self.tile_size.0 / 2.0);
            let y = (col as f32 + row as f32) * (self.tile_size.1 / 2.0);

            // Draw the hovered tile with a different color
            let points = [
                egui::pos2(x, y - (self.tile_size.1 / 2.0)),
                egui::pos2(x + (self.tile_size.0 / 2.0), y),
                egui::pos2(x, y + (self.tile_size.1 / 2.0)),
                egui::pos2(x - (self.tile_size.0 / 2.0), y),
            ];
            ui.painter().add(egui::Shape::closed_line(
                points.to_vec(),
                egui::Stroke::new(2.0, egui::Color32::YELLOW),
            ));
        }
    }
}
