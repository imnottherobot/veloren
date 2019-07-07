use super::{
    super::{Animation, SkeletonAttr},
    CharacterSkeleton,
};
use common::comp::item::Weapon;
use std::f32::consts::PI;
use vek::*;

pub struct CjumpAnimation;

impl Animation for CjumpAnimation {
    type Skeleton = CharacterSkeleton;
    type Dependency = f64;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        _global_time: f64,
        anim_time: f64,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let wave_slow = (anim_time as f32 * 7.0).sin();
        let wave_stop = (anim_time as f32 * 4.5).min(PI / 2.0).sin();

        next.head.offset = Vec3::new(
            0.0 + skeleton_attr.neck_right,
            0.0 + skeleton_attr.neck_forward,
            skeleton_attr.neck_height + 15.0,
        );
        next.head.ori = Quaternion::rotation_x(0.25 + wave_stop * 0.1 + wave_slow * 0.04);
        next.head.scale = Vec3::one() * skeleton_attr.head_scale;

        next.chest.offset = Vec3::new(0.0, 0.0, 8.0);
        next.chest.ori = Quaternion::rotation_z(0.0);
        next.chest.scale = Vec3::one();

        next.belt.offset = Vec3::new(0.0, 0.0, 6.0);
        next.belt.ori = Quaternion::rotation_z(0.0);
        next.belt.scale = Vec3::one();

        next.shorts.offset = Vec3::new(0.0, 0.0, 3.0);
        next.shorts.ori = Quaternion::rotation_z(0.0);
        next.shorts.scale = Vec3::one();

        match Weapon::Hammer {
            //TODO: Inventory
            Weapon::Sword => {
                next.l_hand.offset = Vec3::new(-7.0, 3.25, 0.25 + wave_stop * 2.0);
                next.l_hand.ori = Quaternion::rotation_x(-0.3);
                next.l_hand.scale = Vec3::one() * 1.01;
                next.r_hand.offset = Vec3::new(-7.0, 3.0, -2.0 + wave_stop * 2.0);
                next.r_hand.ori = Quaternion::rotation_x(-0.3);
                next.r_hand.scale = Vec3::one() * 1.01;
                next.weapon.offset = Vec3::new(
                    -7.0 + skeleton_attr.weapon_x,
                    4.0 + skeleton_attr.weapon_y,
                    0.0 + wave_stop * 2.0,
                );
                next.weapon.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.weapon.scale = Vec3::one();
            }
            Weapon::Axe => {
                next.l_hand.offset = Vec3::new(-6.0, 3.5, 0.0);
                next.l_hand.ori = Quaternion::rotation_x(-0.3);
                next.l_hand.scale = Vec3::one() * 1.01;
                next.r_hand.offset = Vec3::new(-6.0, 3.0, -2.0);
                next.r_hand.ori = Quaternion::rotation_x(-0.3);
                next.r_hand.scale = Vec3::one() * 1.01;
                next.weapon.offset = Vec3::new(
                    -6.0 + skeleton_attr.weapon_x,
                    4.5 + skeleton_attr.weapon_y,
                    0.0,
                );
                next.weapon.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.weapon.scale = Vec3::one();
            }
            Weapon::Hammer => {
                next.l_hand.offset = Vec3::new(-7.0, 8.25, 2.0 + wave_stop * 2.0);
                next.l_hand.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.2)
                    * Quaternion::rotation_z(0.0);
                next.l_hand.scale = Vec3::one() * 1.01;
                next.r_hand.offset = Vec3::new(7.0, 7.0, -3.0 + wave_stop * 2.0);
                next.r_hand.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.2)
                    * Quaternion::rotation_z(0.0);
                next.r_hand.scale = Vec3::one() * 1.01;
                next.weapon.offset = Vec3::new(
                    5.0 + skeleton_attr.weapon_x,
                    8.75 + skeleton_attr.weapon_y,
                    -2.5 + wave_stop * 2.0,
                );
                next.weapon.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.2)
                    * Quaternion::rotation_z(0.0);
                next.weapon.scale = Vec3::one();
            }
            Weapon::Staff => {
                next.l_hand.offset = Vec3::new(-7.0, 7.5, 0.0);
                next.l_hand.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.7)
                    * Quaternion::rotation_z(1.0);
                next.l_hand.scale = Vec3::one() * 1.01;
                next.r_hand.offset = Vec3::new(7.0, 6.25, 1.5);
                next.r_hand.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.7)
                    * Quaternion::rotation_z(1.0);
                next.r_hand.scale = Vec3::one() * 1.01;
                next.weapon.offset = Vec3::new(
                    5.0 + skeleton_attr.weapon_x,
                    8.0 + skeleton_attr.weapon_y,
                    1.0,
                );
                next.weapon.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.7)
                    * Quaternion::rotation_z(1.0);
                next.weapon.scale = Vec3::one();
            }
            Weapon::SwordShield => {
                next.l_hand.offset = Vec3::new(-6.0, 3.5, 0.0);
                next.l_hand.ori = Quaternion::rotation_x(-0.3);
                next.l_hand.scale = Vec3::one() * 1.01;
                next.r_hand.offset = Vec3::new(-6.0, 3.0, -2.0);
                next.r_hand.ori = Quaternion::rotation_x(-0.3);
                next.r_hand.scale = Vec3::one() * 1.01;
                next.weapon.offset = Vec3::new(
                    -6.0 + skeleton_attr.weapon_x,
                    4.5 + skeleton_attr.weapon_y,
                    0.0,
                );
                next.weapon.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.weapon.scale = Vec3::one();
            }
            Weapon::Bow => {
                next.l_hand.offset = Vec3::new(-6.0, 3.5, 0.0);
                next.l_hand.ori = Quaternion::rotation_x(-0.3);
                next.l_hand.scale = Vec3::one() * 1.01;
                next.r_hand.offset = Vec3::new(-6.0, 3.0, -2.0);
                next.r_hand.ori = Quaternion::rotation_x(-0.3);
                next.r_hand.scale = Vec3::one() * 1.01;
                next.weapon.offset = Vec3::new(
                    -6.0 + skeleton_attr.weapon_x,
                    4.5 + skeleton_attr.weapon_y,
                    0.0,
                );
                next.weapon.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.weapon.scale = Vec3::one();
            }
            Weapon::Daggers => {
                next.l_hand.offset = Vec3::new(-6.0, 3.5, 0.0);
                next.l_hand.ori = Quaternion::rotation_x(-0.3);
                next.l_hand.scale = Vec3::one() * 1.01;
                next.r_hand.offset = Vec3::new(-6.0, 3.0, -2.0);
                next.r_hand.ori = Quaternion::rotation_x(-0.3);
                next.r_hand.scale = Vec3::one() * 1.01;
                next.weapon.offset = Vec3::new(
                    -6.0 + skeleton_attr.weapon_x,
                    4.5 + skeleton_attr.weapon_y,
                    0.0,
                );
                next.weapon.ori = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.weapon.scale = Vec3::one();
            }
        }

        next.l_foot.offset = Vec3::new(-3.4, 1.0, 6.0);
        next.l_foot.ori = Quaternion::rotation_x(wave_stop * -1.2 - wave_slow * 0.2);
        next.l_foot.scale = Vec3::one();

        next.r_foot.offset = Vec3::new(3.4, -1.0, 6.0);
        next.r_foot.ori = Quaternion::rotation_x(wave_stop * 1.2 + wave_slow * 0.2);
        next.r_foot.scale = Vec3::one();

        next.l_shoulder.offset = Vec3::new(-10.0, -3.2, 2.5);
        next.l_shoulder.ori = Quaternion::rotation_x(0.0);
        next.l_shoulder.scale = Vec3::one() * 1.04;

        next.r_shoulder.offset = Vec3::new(0.0, -3.2, 2.5);
        next.r_shoulder.ori = Quaternion::rotation_x(0.0);
        next.r_shoulder.scale = Vec3::one() * 1.04;

        next.draw.offset = Vec3::new(0.0, 5.0, 0.0);
        next.draw.ori = Quaternion::rotation_y(0.0);
        next.draw.scale = Vec3::one() * 0.0;

        next.torso.offset = Vec3::new(0.0, -0.2, 0.0) * skeleton_attr.scaler;
        next.torso.ori = Quaternion::rotation_x(-0.2);
        next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;

        next
    }
}
