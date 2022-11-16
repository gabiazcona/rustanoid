use tetra::{Context, State};
use tetra::graphics::text::{Text, Font};
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};

use tetra::math::Vec2;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};
use crate::entity::Entity;
use crate::blocks;


const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;

const PADDLE_SPIN: f32 = 4.0;
const BALL_ACCELERATION: f32 = 0.05;


pub struct GameState {
    paddle: Entity,
    ball: Entity,
    lost_text: Text,
    win_text: Text,
    game_end: bool,
    player_win: bool,
    blocks: Vec<Entity>,
}




impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.992,0.0, 0.329));
        
        self.paddle.texture.draw(ctx, self.paddle.position);
        self.ball.texture.draw(ctx, self.ball.position);

        if self.game_end {
            if self.player_win {
                self.win_text.draw(ctx, Vec2::new(WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0));
            } else {
                self.lost_text.draw(ctx, Vec2::new(WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0));
            }
        }

        for block in self.blocks.iter() {
            if block.alive {
                block.texture.draw(ctx, block.position);
            }
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if !self.game_end {
            if input::is_key_down(ctx, Key::Left) 
                && (self.paddle.position.x > 0.0) {
                    self.paddle.position.x -= PADDLE_SPEED;
            }
            if input::is_key_down(ctx, Key::Right) 
                // not move past window border
                && (self.paddle.position.x + (self.paddle.texture.width() as f32) < WINDOW_WIDTH) {
                    self.paddle.position.x += PADDLE_SPEED;
            }
            self.ball.position += self.ball.velocity;

            let paddle_bounds = self.paddle.bounds();
            let ball_bounds = self.ball.bounds();

            let paddle_hit = if ball_bounds.intersects(&paddle_bounds) {
                Some(&self.paddle)
            } else { 
                None 
            };

            if let Some(paddle) = paddle_hit {
                self.ball.velocity.y = -(self.ball.velocity.y + (BALL_ACCELERATION * self.ball.velocity.y.signum()));

                let offset = (paddle.centre().x - self.ball.centre().x) / paddle.width();

                self.ball.velocity.x += PADDLE_SPIN * -offset;
            }

            if self.ball.position.x <= 0.0 
            || self.ball.position.x + self.ball.width() >= WINDOW_WIDTH {
                self.ball.velocity.x = -self.ball.velocity.x;
            }
            if self.ball.position.y  <= 0.0 {
                self.ball.velocity.y = -self.ball.velocity.y;
            }
            
            // check if ball collides with any block and if any block alive
            let mut block_alive = false;
            for block in self.blocks.iter_mut() {
                if ball_bounds.intersects(&block.bounds()) {
                    block.kill();
                }

                if block.alive {
                    block_alive = true;
                }
            }

            // check if player loses
            if self.ball.position.y >= WINDOW_HEIGHT {
                self.game_end = true;
                println!("YOU LOST!");
            }

            // check if player win
            if !block_alive {
                self.game_end = true;
                self.player_win = true;
            }

        }
        Ok(())
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "./resources/paddleBlu.png")?;
        let paddle_position = Vec2::new((WINDOW_WIDTH - paddle_texture.width() as f32) / 2.0, WINDOW_HEIGHT - paddle_texture.height() as f32);
         
        let ball_texture = Texture::new(ctx, "./resources/ballGrey.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(0.0, BALL_SPEED);
        
        let lost_text = Text::new(
            "You lost!",
            Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 36.0)?,
        );
        let win_text = Text::new(
            "You win!",
            Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 36.0)?,
        );

        let blocks = Self::load(ctx, &blocks::LEVEL1)?;

        Ok(GameState {
            game_end: false,
            lost_text: lost_text,
            win_text: win_text,
            player_win: false,
            paddle: Entity::new(paddle_texture, paddle_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            blocks: blocks,
        })
    }

    fn load(ctx: &mut Context, level: &[(&str, (f32, f32))]) -> tetra::Result<Vec<Entity>> {
        let mut blocks = Vec::new();
        for block in level.iter() {
            blocks.push(
                Entity::new(Texture::new(ctx, block.0)?, Vec2::new(block.1.0, block.1.1)),
            );
        };
        Ok(blocks)
        
    }
}