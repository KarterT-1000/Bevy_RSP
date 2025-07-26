use bevy::prelude::*;
use rand::{Rng, thread_rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, rps_system)
        .run();
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hand {
    Rock,
    Scissors,
    Paper,
}

#[derive(Component)]
pub struct RspButton(Hand);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.),
            ..default()
        })
        .with_children(|parent| {
            for (hand, path) in [
                (Hand::Rock, "rsp/rock.png"),
                (Hand::Scissors, "rsp/scissors.png"),
                (Hand::Paper, "rsp/paper.png"),
            ] {
                parent.spawn((
                    ImageNode::from(asset_server.load(path)),
                    Node {
                        //position_type: PositionType::Absolute,
                        width: Val::Px(500.),
                        height: Val::Px(500.),
                        ..default()
                    },
                    Interaction::None,
                    RspButton(hand),
                ));
            }
        });
}

pub fn random_hand() -> Hand {
    match thread_rng().gen_range(0..=2) {
        0 => Hand::Rock,
        1 => Hand::Paper,
        _ => Hand::Scissors,
    }
}

pub fn rps_system(mut query: Query<(&Interaction, &RspButton), Changed<Interaction>>) {
    let cpu_hand = random_hand();
    for (interaction, rps_button) in &mut query {
        if *interaction == Interaction::Pressed {
            let player_hand = rps_button.0;

            println!("『あなた: {player_hand:?}』 VS 『CPU: {cpu_hand:?}』");

            match (player_hand, cpu_hand) {
                (a, b) if a == b => println!("引き分け！"),
                (Hand::Rock, Hand::Scissors)
                | (Hand::Paper, Hand::Rock)
                | (Hand::Scissors, Hand::Paper) => println!("あなたの勝ち！"),
                _ => println!("あなたの負け！"),
            }
        }
    }
}
