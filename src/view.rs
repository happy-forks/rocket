use opengl_graphics::GlGraphics;
use piston_window::{self, Context, Transformed};

use drawing::color;
use models::{Bullet, Enemy, Particle, Player, World, PLAYER_POLYGON};
use resources::Resources;
use geometry::{Advance, Collide, Position};

/// Renders the game to the screen
pub fn render_game(c: Context, g: &mut GlGraphics, res: &mut Resources, world: &World, score: u32) {
    // Clear everything
    piston_window::clear(color::BLACK, g);

    // Render the world
    render_world(world, c, g);

    // Render the score
    piston_window::text(color::ORANGE,
            22,
            &format!("Score: {}", score),
            &mut res.font,
            c.trans(10.0, 20.0).transform,
            g);
}

/// Renders the world and everything in it
pub fn render_world(world: &World, c: Context, g: &mut GlGraphics) {
    for particle in &world.particles {
        render_particle(particle, &c, g);
    }

    for bullet in &world.bullets {
        render_bullet(bullet, &c, g);
    }

    for enemy in &world.enemies {
        render_enemy(enemy, &c, g);
    }

    render_player(&world.player, &c, g);
}

/// Renders a particle
pub fn render_particle(particle: &Particle, c: &Context, gl: &mut GlGraphics) {
    let radius = 5.0 * particle.ttl;
    piston_window::ellipse(
        color::VIOLET,
        [0.0, 0.0, radius * 2.0, radius * 2.0],
        c.trans(particle.x() - radius, particle.y() - radius).transform,
        gl);
}

/// Renders a bullet
pub fn render_bullet(bullet: &Bullet, c: &Context, gl: &mut GlGraphics) {
    piston_window::ellipse(
        color::BLUE,
        [0.0, 0.0, bullet.diameter(), bullet.diameter()],
        c.trans(bullet.x() - bullet.radius(), bullet.y() - bullet.radius()).transform,
        gl);
}

/// Renders an enemy
pub fn render_enemy(enemy: &Enemy, c: &Context, gl: &mut GlGraphics) {
    piston_window::ellipse(
        color::YELLOW,
        [0.0, 0.0, 20.0, 20.0],
        c.trans(enemy.x() - 10.0, enemy.y() - 10.0).transform,
        gl);
}

/// Render the player
pub fn render_player(player: &Player, c: &Context, gl: &mut GlGraphics) {
    // Set the center of the player as the origin and rotate it
    let transform = c.transform
        .trans(player.x(), player.y())
        .rot_rad(player.direction());

    // Draw a rectangle on the position of the player
    piston_window::polygon(color::RED, PLAYER_POLYGON, transform, gl);
}
