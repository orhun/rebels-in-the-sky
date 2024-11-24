use super::{
    collisions::HitBox, networking::ImageType, space_callback::SpaceCallback, traits::*,
    utils::EntityState,
};
use crate::{register_impl, space_adventure::constants::*};
use glam::{I16Vec2, Vec2};
use image::{Pixel, Rgba, RgbaImage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParticleEntity {
    id: usize,
    color: Rgba<u8>,
    previous_position: Vec2,
    position: Vec2,
    velocity: Vec2,
    state: EntityState,
    image: RgbaImage,
    layer: usize,
    hit_box: HitBox,
}

impl Body for ParticleEntity {
    fn previous_position(&self) -> I16Vec2 {
        self.previous_position.as_i16vec2()
    }

    fn position(&self) -> I16Vec2 {
        self.position.as_i16vec2()
    }

    fn velocity(&self) -> I16Vec2 {
        self.velocity.as_i16vec2()
    }

    fn update_body(&mut self, deltatime: f32) -> Vec<SpaceCallback> {
        self.previous_position = self.position;
        self.position = self.position + self.velocity * deltatime;

        if self.position.x < 0.0 || self.position.x > SCREEN_SIZE.x as f32 {
            return vec![SpaceCallback::DestroyEntity { id: self.id() }];
        }
        if self.position.y < 0.0 || self.position.y > SCREEN_SIZE.y as f32 {
            return vec![SpaceCallback::DestroyEntity { id: self.id() }];
        }

        match self.state {
            EntityState::Decaying { lifetime } => {
                let new_lifetime = lifetime - deltatime;
                if new_lifetime > 0.0 {
                    self.state = EntityState::Decaying {
                        lifetime: new_lifetime,
                    };
                } else {
                    return vec![SpaceCallback::DestroyEntity { id: self.id() }];
                }
            }
            _ => {}
        }

        vec![]
    }
}

impl Sprite for ParticleEntity {
    fn image(&self) -> &RgbaImage {
        &self.image
    }

    fn network_image_type(&self) -> ImageType {
        ImageType::Particle {
            color: self.color.to_rgb().0,
        }
    }
}

impl Collider for ParticleEntity {
    fn hit_box(&self) -> &HitBox {
        &self.hit_box
    }
}

register_impl!(!ControllableSpaceship for ParticleEntity);
register_impl!(!ResourceFragment for ParticleEntity);

impl Entity for ParticleEntity {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn layer(&self) -> usize {
        self.layer
    }
}

impl ParticleEntity {
    pub fn new(
        position: Vec2,
        velocity: Vec2,
        color: Rgba<u8>,
        state: EntityState,
        layer: usize,
    ) -> Self {
        let image = RgbaImage::from_pixel(1, 1, color);
        let mut hit_box = HashMap::new();
        hit_box.insert(I16Vec2::ZERO, true);
        Self {
            id: 0,
            color,
            previous_position: position,
            position,
            velocity,
            state,
            image,
            layer,
            hit_box: hit_box.into(),
        }
    }
}
