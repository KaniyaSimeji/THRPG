use std::path::Path;

use crate::setting::chara::CharaData;
use tokio::fs;

pub(crate) async fn new_enemy<T: AsRef<Path>>(toml_path: T) -> Option<CharaData> {
    let chara_data = {
        let file_content = fs::read_to_string(toml_path).await.unwrap();

        toml::from_str(file_content.as_str())
    }
    .unwrap();

    Some(chara_data)
}

pub mod charabase {
    #[derive(Debug, Clone)]
    pub struct CharaBase {
        pub name: String,
        pub power: u8,
        pub guard: u8,
        pub speed: u8,
        pub hp: u8,
        pub mp: u8,
    }

    pub struct CharaAttack {
        pub name: String,
        pub damage: u32,
        pub hit_rate: f32, // 0.08 etc...
        pub abnormal_state: Option<AbnormalState>,
    }

    pub enum AbnormalState {
        Slowed,   // チルノとか
        Poisoned, //メディスンとか
        Unlucky,  // 鍵山雛とか
    }

    pub trait BaseController {
        fn power(&self) -> u8;
        fn power_mut(&mut self) -> &mut u8;
        fn guard(&self) -> u8;
        fn guard_mut(&mut self) -> &mut u8;
        fn speed(&self) -> u8;
        fn speed_mut(&mut self) -> &mut u8;
        fn hp(&self) -> u8;
        fn hp_mut(&mut self) -> &mut u8;
        fn mp(&self) -> u8;
        fn mp_mut(&mut self) -> &mut u8;
    }

    impl BaseController for CharaBase {
        fn power(&self) -> u8 {
            self.speed
        }

        fn power_mut(&mut self) -> &mut u8 {
            &mut self.speed
        }

        fn guard(&self) -> u8 {
            self.guard
        }

        fn guard_mut(&mut self) -> &mut u8 {
            &mut self.guard
        }

        fn speed(&self) -> u8 {
            self.speed
        }

        fn speed_mut(&mut self) -> &mut u8 {
            &mut self.speed
        }

        fn hp(&self) -> u8 {
            self.hp
        }

        fn hp_mut(&mut self) -> &mut u8 {
            &mut self.hp
        }

        fn mp(&self) -> u8 {
            self.mp
        }

        fn mp_mut(&mut self) -> &mut u8 {
            &mut self.mp
        }
    }
}

pub mod chara_enemy {
    pub struct CharaEnemy {
        pub base: super::charabase::CharaBase,
        pub attack: Vec<super::charabase::CharaAttack>,
        pub exp: u32,
    }

    trait EnemyController: super::charabase::BaseController {}

    impl super::charabase::BaseController for CharaEnemy {
        fn power(&self) -> u8 {
            self.base.power
        }

        fn power_mut(&mut self) -> &mut u8 {
            &mut self.base.power
        }

        fn guard(&self) -> u8 {
            self.base.guard
        }

        fn guard_mut(&mut self) -> &mut u8 {
            &mut self.base.guard
        }

        fn speed(&self) -> u8 {
            self.base.speed
        }

        fn speed_mut(&mut self) -> &mut u8 {
            &mut self.base.speed
        }

        fn hp(&self) -> u8 {
            self.base.hp
        }

        fn hp_mut(&mut self) -> &mut u8 {
            &mut self.base.hp
        }

        fn mp(&self) -> u8 {
            self.base.mp
        }

        fn mp_mut(&mut self) -> &mut u8 {
            &mut self.base.mp
        }
    }

    impl EnemyController for CharaEnemy {}
}

pub mod chara_player {
    pub struct CharaPlayer {
        pub base: super::charabase::CharaBase,
        pub attack: Vec<super::charabase::CharaAttack>,
        pub required_exp: u32,
    }

    enum CharaSkill {}
}
