use super::{
    super::{vek::*, Animation},
    CharacterSkeleton, SkeletonAttr,
};
use common::comp::item::{Hands, ToolKind};
use std::{f32::consts::PI, ops::Mul};

pub struct Input {
    pub attack: bool,
}
pub struct BlockIdleAnimation;

impl Animation for BlockIdleAnimation {
    type Dependency = (Option<ToolKind>, Option<ToolKind>, f64);
    type Skeleton = CharacterSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"character_blockidle\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "character_blockidle")]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        (active_tool_kind, second_tool_kind, global_time): Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let wave_ultra_slow = (anim_time as f32 * 3.0 + PI).sin();
        let wave_ultra_slow_cos = (anim_time as f32 * 3.0 + PI).cos();
        let wave_slow_cos = (anim_time as f32 * 6.0 + PI).cos();

        let _head_look = Vec2::new(
            ((global_time + anim_time) as f32 / 1.5)
                .floor()
                .mul(7331.0)
                .sin()
                * 0.3,
            ((global_time + anim_time) as f32 / 1.5)
                .floor()
                .mul(1337.0)
                .sin()
                * 0.15,
        );
        next.head.position = Vec3::new(
            0.0 + wave_slow_cos * 0.2,
            -1.0 + skeleton_attr.head.0,
            skeleton_attr.head.1 + wave_ultra_slow * 0.2,
        );
        next.head.orientation = Quaternion::rotation_x(0.0);
        next.head.scale = Vec3::one() * 1.01 * skeleton_attr.head_scale;

        next.chest.position = Vec3::new(
            0.0 + wave_slow_cos * 0.2,
            0.0,
            skeleton_attr.chest.1 + wave_ultra_slow * 0.2,
        );
        next.chest.orientation = Quaternion::rotation_y(wave_ultra_slow_cos * 0.01);
        next.chest.scale = Vec3::one();

        next.belt.position = Vec3::new(
            0.0 + wave_slow_cos * 0.2,
            0.0,
            skeleton_attr.belt.1 + wave_ultra_slow * 0.2,
        );
        next.belt.orientation =
            Quaternion::rotation_x(0.0) * Quaternion::rotation_y(wave_ultra_slow_cos * 0.008);
        next.belt.scale = Vec3::one() * 1.01;

        next.shorts.position = Vec3::new(
            0.0 + wave_slow_cos * 0.2,
            0.0,
            skeleton_attr.shorts.1 + wave_ultra_slow * 0.2,
        );
        next.shorts.orientation = Quaternion::rotation_x(0.1);
        next.shorts.scale = Vec3::one();

        match active_tool_kind {
            Some(ToolKind::Sword(_)) => {
                next.hand_l.position = Vec3::new(0.0, -5.0, -5.0);
                next.hand_l.orientation = Quaternion::rotation_x(1.27);
                next.hand_l.scale = Vec3::one() * 1.04;
                next.hand_r.position = Vec3::new(0.0, -6.0, -8.0);
                next.hand_r.orientation = Quaternion::rotation_x(1.27);
                next.hand_r.scale = Vec3::one() * 1.05;
                next.main.position = Vec3::new(0.0, 0.0, -6.0);
                next.main.orientation = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.main.scale = Vec3::one();

                next.control.position = Vec3::new(-8.0, 13.0, 8.0);
                next.control.orientation = Quaternion::rotation_x(0.2)
                    * Quaternion::rotation_y(0.4)
                    * Quaternion::rotation_z(-1.57);
                next.control.scale = Vec3::one();
            },
            Some(ToolKind::Axe(_)) => {
                next.hand_l.position = Vec3::new(
                    -6.0 + wave_ultra_slow_cos * 1.0,
                    3.5 + wave_ultra_slow_cos * 0.5,
                    0.0 + wave_ultra_slow * 1.0,
                );
                next.hand_l.orientation = Quaternion::rotation_x(-0.3);
                next.hand_l.scale = Vec3::one() * 1.01;
                next.hand_r.position = Vec3::new(
                    -6.0 + wave_ultra_slow_cos * 1.0,
                    3.0 + wave_ultra_slow_cos * 0.5,
                    -2.0 + wave_ultra_slow * 1.0,
                );
                next.hand_r.orientation = Quaternion::rotation_x(-0.3);
                next.hand_r.scale = Vec3::one() * 1.01;
                next.main.position = Vec3::new(-6.0, 4.5, 0.0 + wave_ultra_slow * 1.0);
                next.main.orientation = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.main.scale = Vec3::one();
            },
            Some(ToolKind::Hammer(_)) => {
                next.hand_l.position = Vec3::new(-7.0, 3.5 + wave_ultra_slow * 2.0, 6.5);
                next.hand_l.orientation = Quaternion::rotation_x(2.07)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(-0.2);
                next.hand_l.scale = Vec3::one() * 1.01;
                next.hand_r.position = Vec3::new(7.0, 2.5 + wave_ultra_slow * 2.0, 3.75);
                next.hand_r.orientation = Quaternion::rotation_x(2.07)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(-0.2);
                next.hand_r.scale = Vec3::one() * 1.01;
                next.main.position = Vec3::new(5.0, 8.75 + wave_ultra_slow * 2.0, 5.5);
                next.main.orientation = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.35)
                    * Quaternion::rotation_z(-0.85);
                next.main.scale = Vec3::one();
            },
            Some(ToolKind::Dagger(_)) => {
                let hand_scale = 1.12;

                next.control.position = Vec3::new(0.0, 0.0, 0.0);

                next.hand_l.position = Vec3::new(0.0, 0.0, 0.0);
                next.hand_l.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.hand_l.scale = Vec3::one() * hand_scale;

                next.main.position = Vec3::new(0.0, 0.0, 0.0);
                next.main.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.main.scale = Vec3::one();

                next.control_l.position = Vec3::new(-7.0, 0.0, 0.0);
                // next.control_l.orientation = Quaternion::rotation_x(u_slow * 0.15 + 1.0)
                //     * Quaternion::rotation_y(0.0)
                //     * Quaternion::rotation_z(u_slowalt * 0.08);
                // next.control_l.scale = Vec3::one();

                next.hand_r.position = Vec3::new(0.0, 0.0, 0.0);
                next.hand_r.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.hand_r.scale = Vec3::one() * hand_scale;

                next.second.position = Vec3::new(0.0, 0.0, 0.0);
                next.second.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.second.scale = Vec3::one();

                next.control_r.position = Vec3::new(7.0, 0.0, 0.0);
            },
            Some(ToolKind::Shield(_)) => {
                let hand_scale = 1.12;

                next.control.position = Vec3::new(0.0, 0.0, 0.0);

                next.hand_l.position = Vec3::new(0.0, 0.0, 0.0);
                next.hand_l.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.hand_l.scale = Vec3::one() * hand_scale;

                next.main.position = Vec3::new(0.0, 0.0, 0.0);
                next.main.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);

                next.control_l.position = Vec3::new(-7.0, 0.0, 0.0);

                next.hand_r.position = Vec3::new(0.0, 0.0, 0.0);
                next.hand_r.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.hand_r.scale = Vec3::one() * hand_scale;

                next.second.position = Vec3::new(0.0, 0.0, 0.0);
                next.second.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.second.scale = Vec3::one();

                next.control_r.position = Vec3::new(7.0, 0.0, 0.0);
            },
            Some(ToolKind::Debug(_)) => {
                next.hand_l.position = Vec3::new(-7.0, 3.5 + wave_ultra_slow * 2.0, 6.5);
                next.hand_l.orientation = Quaternion::rotation_x(2.07)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(-0.2);
                next.hand_l.scale = Vec3::one() * 1.01;
                next.hand_r.position = Vec3::new(7.0, 2.5 + wave_ultra_slow * 2.0, 3.75);
                next.hand_r.orientation = Quaternion::rotation_x(2.07)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(-0.2);
                next.hand_r.scale = Vec3::one() * 1.01;
                next.main.position = Vec3::new(5.0, 8.75 + wave_ultra_slow * 2.0, 5.5);
                next.main.orientation = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_y(-1.35)
                    * Quaternion::rotation_z(-0.85);
                next.main.scale = Vec3::one();
            },
            _ => {},
        }

        match second_tool_kind {
            Some(ToolKind::Shield(_)) => {
                let hand_scale = 1.12;

                next.control.position = Vec3::new(0.0, 0.0, 0.0);

                next.hand_l.position = Vec3::new(0.0, 0.0, 0.0);
                next.hand_l.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.hand_l.scale = Vec3::one() * hand_scale;

                next.main.position = Vec3::new(0.0, 0.0, 0.0);
                next.main.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.main.scale = Vec3::one();

                next.control_l.position = Vec3::new(-7.0, 0.0, 0.0);

                next.hand_r.position = Vec3::new(0.0, 0.0, 0.0);
                next.hand_r.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.hand_r.scale = Vec3::one() * hand_scale;

                next.second.position = Vec3::new(0.0, 0.0, 0.0);
                next.second.orientation = Quaternion::rotation_x(0.0 * PI)
                    * Quaternion::rotation_y(0.0 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
                next.second.scale = Vec3::one();

                next.control_r.position = Vec3::new(3.0, 7.0, 5.0);
                next.control_r.orientation = Quaternion::rotation_x(0.5 * PI)
                    * Quaternion::rotation_y(0.5 * PI)
                    * Quaternion::rotation_z(0.0 * PI);
            },
            Some(ToolKind::Dagger(_)) => {},
            _ => {},
        }

        next.foot_l.position =
            Vec3::new(-3.4, 0.3, skeleton_attr.foot.1 + wave_ultra_slow_cos * 0.1);
        next.foot_l.orientation = Quaternion::rotation_x(-0.3);
        next.foot_l.scale = Vec3::one();

        next.foot_r.position = Vec3::new(3.4, 1.2, skeleton_attr.foot.1 + wave_ultra_slow * 0.1);
        next.foot_r.orientation = Quaternion::rotation_x(0.3);
        next.foot_r.scale = Vec3::one();

        next.shoulder_l.position = Vec3::new(
            -skeleton_attr.shoulder.0,
            skeleton_attr.shoulder.1,
            skeleton_attr.shoulder.2,
        );
        next.shoulder_l.scale = Vec3::one() * 1.1;

        next.shoulder_r.position = Vec3::new(
            skeleton_attr.shoulder.0,
            skeleton_attr.shoulder.1,
            skeleton_attr.shoulder.2,
        );
        next.shoulder_r.scale = Vec3::one() * 1.1;

        next.glider.position = Vec3::new(0.0, 0.0, 10.0);
        next.glider.scale = Vec3::one() * 0.0;

        next.lantern.position = Vec3::new(
            skeleton_attr.lantern.0,
            skeleton_attr.lantern.1,
            skeleton_attr.lantern.2,
        );
        next.lantern.orientation = Quaternion::rotation_x(0.0);
        next.lantern.scale = Vec3::one();
        next.hold.scale = Vec3::one() * 0.0;

        next.torso.position = Vec3::new(0.0, -0.2, 0.1) * skeleton_attr.scaler;
        next.torso.orientation = Quaternion::rotation_x(0.0);
        next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;

        next.control.scale = Vec3::one();
        next.control_l.scale = Vec3::one();
        next.control_r.scale = Vec3::one();

        next.second.scale = match (
            active_tool_kind.map(|tk| tk.hands()),
            second_tool_kind.map(|tk| tk.hands()),
        ) {
            (Some(Hands::OneHand), Some(Hands::OneHand)) => Vec3::one(),
            (_, _) => Vec3::zero(),
        };

        next
    }
}
