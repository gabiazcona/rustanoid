use tetra::{Context, ContextBuilder, State};
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Rustanoid", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }
}

struct GameState {
    paddle: Entity,
    ball: Entity,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.992,0.0, 0.329));

        self.paddle.texture.draw(ctx, self.paddle.position);
        self.ball.texture.draw(ctx, self.ball.position);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
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

        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "./resources/paddleBlu.png")?;
        let paddle_position = Vec2::new((WINDOW_WIDTH - paddle_texture.width() as f32) / 2.0, WINDOW_HEIGHT - paddle_texture.height() as f32);
        
        let ball_texture = Texture::new(ctx, "./resources/ballGrey.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(0.0, BALL_SPEED);
        
        Ok(GameState {
            paddle: Entity::new(paddle_texture, paddle_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
        })
    }
}