use tetra::graphics::{Texture, Rectangle};
use tetra::math::Vec2;


pub struct Entity {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub alive: bool,
}


impl Entity {
    pub fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    pub fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
            alive: true,
        }
    }

    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    pub fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

}
