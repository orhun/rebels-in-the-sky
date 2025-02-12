use std::collections::HashMap;

use crate::engine::timer::Timer;
use crate::engine::types::GameStats;
use crate::types::{PlanetId, Tick};
use crate::world::position::{Position, MAX_POSITION};
use crate::{
    engine::types::TeamInGame,
    types::{AppResult, GameId, TeamId},
    world::{player::Player, team::Team, world::World},
};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Display, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChallengeState {
    #[default]
    Syn,
    SynAck,
    Ack,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Challenge {
    pub state: ChallengeState,
    pub home_peer_id: PeerId,
    pub away_peer_id: PeerId,
    pub home_team: Option<TeamInGame>,
    pub away_team: Option<TeamInGame>,
    pub game_id: Option<GameId>,
    pub starting_at: Option<Tick>,
    pub error_message: Option<String>,
}

impl Challenge {
    pub fn new(home_peer_id: PeerId, away_peer_id: PeerId) -> Self {
        Self {
            state: ChallengeState::Syn,
            home_peer_id,
            away_peer_id,
            home_team: None,
            away_team: None,
            game_id: None,
            starting_at: None,
            error_message: None,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "Challenge: {} {} {} - {} vs {} ",
            self.state,
            self.home_peer_id,
            self.away_peer_id,
            self.home_team
                .as_ref()
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "None".to_string()),
            self.away_team
                .as_ref()
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "None".to_string()),
        )
    }

    pub fn generate_game(&self, world: &mut World) -> AppResult<GameId> {
        if self.starting_at.is_none() {
            return Err("Cannot generate game, starting_at not set".into());
        }
        world.generate_game(
            self.game_id.unwrap(),
            self.home_team
                .as_ref()
                .ok_or("Cannot generate game, home team not found in challenge".to_string())?
                .clone(),
            self.away_team
                .as_ref()
                .ok_or("Cannot generate game, away team not found in challenge".to_string())?
                .clone(),
            self.starting_at.unwrap(),
        )?;
        Ok(self.game_id.unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkTeam {
    pub team: Team,
    pub players: Vec<Player>,
}

impl NetworkTeam {
    pub fn new(team: Team, players: Vec<Player>) -> Self {
        Self { team, players }
    }

    pub fn from_team_id(world: &World, team_id: &TeamId) -> AppResult<Self> {
        let team = world.get_team_or_err(*team_id)?.clone();
        let players = world.get_players_by_team(&team)?;
        Ok(Self::new(team, players))
    }

    pub fn set_peer_id(&mut self, peer_id: PeerId) {
        self.team.peer_id = Some(peer_id);
        for player in self.players.iter_mut() {
            player.peer_id = Some(peer_id.clone());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkGame {
    pub id: GameId,
    pub home_team_in_game: TeamInGame,
    pub away_team_in_game: TeamInGame,
    pub location: PlanetId,
    pub attendance: u32,
    pub starting_at: Tick,
    pub timer: Timer,
}

impl NetworkGame {
    pub fn from_game_id(world: &World, game_id: GameId) -> AppResult<Self> {
        let game = world.get_game_or_err(game_id)?.clone();

        let mut home_team_in_game = game.home_team_in_game.clone();
        let mut stats = HashMap::new();
        for (idx, player_id) in home_team_in_game.initial_positions.iter().enumerate() {
            let mut player_stats = GameStats::default();
            if (idx as Position) < MAX_POSITION {
                player_stats.position = Some(idx as Position);
            }
            let player_stat = home_team_in_game
                .stats
                .get(player_id)
                .ok_or("Cannot get player stats for home team in game".to_string())?;
            player_stats.initial_tiredness = player_stat.initial_tiredness;
            player_stats.tiredness = player_stat.tiredness;
            stats.insert(player_id.clone(), player_stats.clone());
        }
        home_team_in_game.stats = stats;

        let mut away_team_in_game = game.away_team_in_game.clone();
        let mut stats = HashMap::new();
        for (idx, player_id) in away_team_in_game.initial_positions.iter().enumerate() {
            let mut player_stats = GameStats::default();
            if (idx as Position) < MAX_POSITION {
                player_stats.position = Some(idx as Position);
            }
            let player_stat = away_team_in_game
                .stats
                .get(player_id)
                .ok_or("Cannot get player stats for away team in game".to_string())?;
            player_stats.initial_tiredness = player_stat.initial_tiredness;
            player_stats.tiredness = player_stat.tiredness;
            stats.insert(player_id.clone(), player_stats.clone());
        }
        away_team_in_game.stats = stats;

        Ok(Self {
            id: game.id,
            home_team_in_game,
            away_team_in_game,
            location: game.location,
            attendance: game.attendance,
            starting_at: game.starting_at,
            timer: game.timer,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeedInfo {
    pub connected_peers_count: usize,
    pub version_major: usize,
    pub version_minor: usize,
    pub version_patch: usize,
    pub message: Option<String>,
}

impl SeedInfo {
    pub fn new(connected_peers_count: usize, message: Option<String>) -> Self {
        Self {
            connected_peers_count,
            version_major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            version_minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            version_patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
            message,
        }
    }
}
